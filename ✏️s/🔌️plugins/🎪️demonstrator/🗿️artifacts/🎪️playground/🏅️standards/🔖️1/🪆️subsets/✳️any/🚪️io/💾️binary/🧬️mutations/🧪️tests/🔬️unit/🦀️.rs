use crate::standards::v1::subsets::any::io::binary::mutations::*;

#[test]
fn op_binary_round_trips_and_agrees_with_text() {
    use crate::standards::v1::subsets::any::schema::mutations::change_schema::ChangeSchema;
    let operation = PlaygroundMutation::ChangeSchema(ChangeSchema { new_schema: "playground.custom".into() });
    semio_os_store_test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}
