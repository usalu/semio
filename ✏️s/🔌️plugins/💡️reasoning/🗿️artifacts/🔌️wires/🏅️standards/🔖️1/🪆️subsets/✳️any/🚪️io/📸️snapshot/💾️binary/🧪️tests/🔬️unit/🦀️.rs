use super::*;
use crate::empty_wires_snapshot;

#[semio_framework_async_macros::async_test]
async fn pack_round_trips_empty() {
    let snapshot = empty_wires_snapshot();
    let back = <WiresSnapshot as store::ArtifactPack>::decode_pack(&<WiresSnapshot as store::ArtifactPack>::encode_pack(&snapshot)).expect("decode");
    assert_eq!(back, snapshot);
}

/// ⚖️ LAW: the demo parent's pack schema identity is derived and the pack keeps every parent field and the exact handle.
#[semio_framework_async_macros::async_test]
async fn pack_schema_identity_is_derived_and_keeps_the_demo_parent() {
    let snapshot = crate::standards::v1::subsets::any::io::snapshot::text::parse_dsl(crate::examples::demo::PRIMARY_TEXT).expect("demo parses");
    store::os_store::test_support::assert_pack_schema_identity(&snapshot);
    let back = <WiresSnapshot as store::ArtifactPack>::decode_pack(&store::ArtifactPack::encode_pack(&snapshot)).expect("decode");
    assert_eq!(back, snapshot);
}
