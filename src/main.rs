use alloy_json_abi::JsonAbi;
use std::io::{Read, self};
 
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Unable to read the line");

    let abi = JsonAbi::from_json_str(input.as_str()).expect("Unable to parse the file");
    
    let result = abi.to_sol("Testxo", None);
    println!("{}", result);
}
