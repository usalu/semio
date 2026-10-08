//! 📡️ Native mutation binary and text codecs have identical semantic output.

use crate::standards::v1::subsets::any::io::binary::mutations::*;
use crate::{schema, WriterSnapshot};

#[semio_framework_async_macros::async_test]
async fn op_binary_round_trips_and_agrees_with_text() {
    let operation = WriterMutation::EditText(schema::mutations::EditText { text: "hello".into() });
    store::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    let bytes = encode_op(&operation).expect("encode");
    assert_eq!(decode_op(&bytes).expect("decode"), operation);
}
