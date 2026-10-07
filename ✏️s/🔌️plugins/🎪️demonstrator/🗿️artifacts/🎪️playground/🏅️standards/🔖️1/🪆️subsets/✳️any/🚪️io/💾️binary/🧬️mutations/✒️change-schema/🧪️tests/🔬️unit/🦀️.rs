use crate::standards::v1::subsets::any::io::text::mutations::change_schema::{apply_playground_mutation_json,undo_playground_mutation_json};
use crate::standards::v1::subsets::any::schema::mutations::change_schema::*;

#[test]
fn binary_and_text_wire_forms_agree() {
    let operation = PlaygroundMutation::ChangeSchema(ChangeSchema { new_schema: "playground.custom".into() });
    store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    assert_eq!(decode_payload(&encode_payload(&ChangeSchema { new_schema: "playground.custom".into() }).expect("encode payload")).expect("decode payload").new_schema, "playground.custom");
}
