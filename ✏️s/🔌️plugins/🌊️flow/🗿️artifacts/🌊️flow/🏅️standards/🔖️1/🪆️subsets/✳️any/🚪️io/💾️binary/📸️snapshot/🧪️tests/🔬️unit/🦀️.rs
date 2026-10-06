use crate::standards::v1::subsets::any::io::binary::snapshot::*;
use crate::standards::v1::subsets::any::io::text::snapshot as dsl;

#[semio_framework_async_macros::async_test]
async fn pack_round_trips_and_agrees_with_dsl() {
    let snapshot = dsl::parse_dsl(include_str!("../../../../📝️text/📸️snapshot/🧫️fixtures/🗣️.dsl.semio")).expect("parse default snapshot");
    store::os_store::test_support::assert_dsl_pack_equivalence(&snapshot);
    let bytes = encode(&snapshot);
    assert_eq!(decode(&bytes).expect("decode"), snapshot);
}

#[semio_framework_async_macros::async_test]
async fn pack_protocol_names_snapshot_segment() {
    assert!(COMPONENT_PROTOCOL_SEMIO.contains("segment payload"));
}
