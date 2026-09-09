use super::*;

#[semio_framework_async_macros::async_test]
async fn document_dsl_pack_equivalence() {
    store::os_store::test_support::assert_dsl_pack_equivalence(&Din4108Snapshot::default());
}

#[semio_framework_async_macros::async_test]
async fn pack_round_trips() {
    let document = Din4108Snapshot::default();
    let bytes = encode(&document);
    assert_eq!(decode(&bytes).expect("decode"), document);
}
