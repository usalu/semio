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
