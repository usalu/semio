
use super::*;

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    let operation = FormMutation::ChangeFormTitle(crate::mutations::change_form_title::mutation::ChangeFormTitle { new_title: Some("Renamed".into()) });
    store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}
