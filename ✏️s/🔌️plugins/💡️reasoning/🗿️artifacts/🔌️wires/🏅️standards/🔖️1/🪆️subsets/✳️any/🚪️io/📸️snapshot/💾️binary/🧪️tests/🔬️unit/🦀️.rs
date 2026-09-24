use super::*;
use crate::{empty_wires_snapshot, wires_working_board};

#[semio_framework_async_macros::async_test]
async fn pack_round_trips_empty() {
    let snapshot = empty_wires_snapshot();
    let bytes = <WiresSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
    let back = <WiresSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(back.wires_fixture, snapshot.wires_fixture);
    assert_eq!(wires_working_board(&back), wires_working_board(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn pack_schema_identity_is_derived_and_keeps_the_demo_board() {
    let snapshot = crate::standards::v1::subsets::any::io::snapshot::text::parse_dsl(crate::examples::demo::PRIMARY_TEXT).expect("demo parses");
    assert!(!crate::wires_working_scene(&snapshot).nodes.is_empty());
    store::os_store::test_support::assert_pack_schema_identity(&snapshot);
    let back = <WiresSnapshot as store::ArtifactPack>::decode_pack(&store::ArtifactPack::encode_pack(&snapshot)).expect("decode");
    assert_eq!((back.wires_fixture.clone(), back.meta.clone()), (snapshot.wires_fixture.clone(), snapshot.meta.clone()));
    assert_eq!(wires_working_board(&back), wires_working_board(&snapshot));
}
