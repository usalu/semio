
use super::*;

#[semio_framework_async_macros::async_test]
async fn document_dsl_pack_equivalence() {
    store::os_store::test_support::assert_dsl_pack_equivalence(&Din18599Snapshot::default());
}

#[semio_framework_async_macros::async_test]
async fn pack_round_trips() {
    let document = Din18599Snapshot::default();
    let bytes = encode(&document);
    assert_eq!(decode(&bytes).expect("decode"), document);
}
