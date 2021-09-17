
//? imports 
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    log::sol_log_compute_units,
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Define the type of state stored in accounts

//? account extends data from this packages
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub txt: String,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");

    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    msg!("Start instruction decode");
    let message = GreetingAccount::try_from_slice(instruction_data).map_err(|err| {
      msg!("Receiving message as string utf8 failed, {:?}", err);
      ProgramError::InvalidInstructionData  
    })?;
    msg!("Greeting passed to program is {:?}", message);

    let data = &mut &mut account.data.borrow_mut();
    msg!("Start save instruction into data");
    data[..instruction_data.len()].copy_from_slice(&instruction_data);

    sol_log_compute_units();
    msg!("Was sent message {}!", message.txt);

    Ok(())
}

