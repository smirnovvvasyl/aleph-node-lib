use scale::Encode;

use crate::{args::VerifyArgs, VerificationKeyIdentifier};

pub const IDENTIFIER: VerificationKeyIdentifier = [41; 8];
pub const PROOF: [u8; 20] = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3, 2, 3, 8, 4];
pub const INPUT: [u8; 11] = [0, 5, 7, 7, 2, 1, 5, 6, 6, 4, 9];

/// Returns encoded arguments to `verify` chain extension call.
pub fn verify_args() -> Vec<u8> {
    VerifyArgs {
        verification_key_identifier: IDENTIFIER,
        proof: PROOF.to_vec(),
        public_input: INPUT.to_vec(),
    }
    .encode()
}
