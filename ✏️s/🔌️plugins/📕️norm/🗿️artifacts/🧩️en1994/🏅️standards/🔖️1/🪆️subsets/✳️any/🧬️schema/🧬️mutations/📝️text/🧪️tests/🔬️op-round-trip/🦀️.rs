//! 🧪️ `unit_tests` — moved out of `📝️text/🦀️.rs` into its canonical test implementation.
use super::*;
use protocol::{OpBinary, OpText};
use crate::artifact_schema::mutations::demo_mutation_cases;

#[semio_framework_async_macros::async_test]
async fn op_text_binary_roundtrip_law() {
    for mutation in demo_mutation_cases() {
        let printed = mutation.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line");
        let parsed = <En1994Mutation as OpText>::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op failed: {e}"));
        assert_eq!(parsed, mutation);
        let encoded = mutation.encode_op().unwrap();
        let decoded = <En1994Mutation as OpBinary>::decode_op(&encoded).unwrap();
        assert_eq!(decoded, mutation);
    }
}
