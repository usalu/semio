use super::*;
use crate::document_dsl as dsl;

#[semio_framework_async_macros::async_test]
async fn pack_round_trips_and_agrees_with_dsl() {
    let snapshot = dsl::parse_dsl(dsl::FLOW_EXAMPLE_TEXT).expect("parse default snapshot");
    store::os_store::test_support::assert_dsl_pack_equivalence(&snapshot);
    let bytes = encode(&snapshot);
    assert_eq!(decode(&bytes).expect("decode"), snapshot);
}

#[semio_framework_async_macros::async_test]
async fn pack_protocol_names_snapshot_segment() {
    assert!(COMPONENT_PROTOCOL_SEMIO.contains("segment payload"));
}
