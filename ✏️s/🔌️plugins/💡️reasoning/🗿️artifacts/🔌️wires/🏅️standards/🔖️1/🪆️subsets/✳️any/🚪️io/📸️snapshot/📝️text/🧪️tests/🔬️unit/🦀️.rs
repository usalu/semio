
use super::*;
use crate::{empty_wires_snapshot, wires_working_board};

fn populated() -> WiresSnapshot {
    let mut snapshot = empty_wires_snapshot();
    let node = dsl::to_dsl_value(&dsl::json!({ "id": "node-1", "nodeKind": "identity", "shape": "circle", "x": 1.0, "y": 2.0, "radius": 24.0, "text": "Alpha", "handles": [] })).unwrap();
    snapshot = store::apply_mutation(&snapshot, &crate::mutations::create_node(node)).expect("valid mutation").0;
    snapshot
}

#[semio_framework_async_macros::async_test]
async fn dsl_round_trip_empty_document() {
    let document = empty_wires_snapshot();
    store::os_store::test_support::assert_dsl_round_trip(&document);
}

#[semio_framework_async_macros::async_test]
async fn dsl_round_trip_metabolism_fixture() {
    let document = crate::schema::metabolism_wires_example_snapshot().expect("valid metabolism fixture mutations");
    assert_eq!(document.wires_fixture.get("identities").and_then(|value| value.as_array()).map(|items| items.len()), Some(7));
    assert_eq!(document.wires_fixture.get("relationships").and_then(|value| value.as_array()).map(|items| items.len()), Some(9));
    assert_eq!(wires_working_board(&document).get("nodes").and_then(|value| value.as_array()).map(|items| items.len()), Some(7));
    let reparsed = parse_dsl(&print_dsl(&document)).expect("metabolism dsl round trip");
    assert_eq!(wires_working_board(&reparsed).get("nodes").and_then(|value| value.as_array()).map(|items| items.len()), Some(7));
}

#[semio_framework_async_macros::async_test]
async fn dsl_text_round_trips_empty() {
    let snapshot = empty_wires_snapshot();
    let text = <WiresSnapshot as store::ArtifactDsl>::print_dsl(&snapshot);
    let back = <WiresSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(back.wires_fixture, snapshot.wires_fixture);
    assert_eq!(wires_working_board(&back), wires_working_board(&snapshot));
}

/// ⚖️ codec_retention_law: a populated snapshot (real node content, not just the default) survives
/// BOTH codecs — this is what a bare-handle-only codec would silently fail (see this file's module
/// doc, `dag`'s bug writeup).
#[semio_framework_async_macros::async_test]
async fn codec_retention_law_carries_real_node_content_not_just_the_handle() {
    let snapshot = populated();
    let text = <WiresSnapshot as store::ArtifactDsl>::print_dsl(&snapshot);
    let back_text = <WiresSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
    assert_eq!(wires_working_board(&back_text).get("nodes").and_then(|v| v.as_array()).map(|a| a.len()), Some(1), "node content must survive a FRESH decode, not just round-trip in-process");
    let bytes = <WiresSnapshot as store::ArtifactPack>::encode_pack(&snapshot);
    let back_pack = <WiresSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
    assert_eq!(wires_working_board(&back_pack).get("nodes").and_then(|v| v.as_array()).map(|a| a.len()), Some(1));
}
