
use super::*;

#[semio_framework_async_macros::async_test]
async fn document_pack_round_trips_and_agrees_with_dsl() {
    let document = En1994Snapshot::default();
    store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    let bytes = encode(&document);
    assert_eq!(decode(&bytes).expect("decode"), document);
}
