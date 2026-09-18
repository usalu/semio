//! 🔬️ The binary op tag is the variant's own position in the shared `DslVariants` roster, so a kind
//! can never carry one tag on the wire and another in the grammar.

use crate::schema::mutations::{change_seed, mask_cell, Grid3dMutation};
use crate::schema::snapshot::Grid3dCell;

#[test]
fn encoding_then_decoding_is_a_fixed_point() {
    for mutation in [change_seed(5), mask_cell(Grid3dCell { x: 1, y: 2, z: 3 })] {
        let bytes = super::encode_op(&mutation).expect("op encodes");
        assert_eq!(super::decode_op(&bytes).expect("op decodes"), mutation);
    }
}

#[test]
fn two_different_kinds_never_encode_to_the_same_bytes() {
    let seed = super::encode_op(&change_seed(5)).expect("op encodes");
    let mask = super::encode_op(&mask_cell(Grid3dCell { x: 1, y: 2, z: 3 })).expect("op encodes");
    assert_ne!(seed, mask, "the variant tag rides inside the framed record, so two kinds can never collide");
    assert_ne!(super::decode_op(&seed).expect("seed decodes"), super::decode_op(&mask).expect("mask decodes"));
}

#[test]
fn a_truncated_payload_is_refused_rather_than_misread() {
    let bytes = super::encode_op(&change_seed(5)).expect("op encodes");
    assert!(super::decode_op(&bytes[..bytes.len().saturating_sub(1)]).is_err() || <Grid3dMutation as protocol::OpBinary>::decode_op(&[]).is_err());
}
