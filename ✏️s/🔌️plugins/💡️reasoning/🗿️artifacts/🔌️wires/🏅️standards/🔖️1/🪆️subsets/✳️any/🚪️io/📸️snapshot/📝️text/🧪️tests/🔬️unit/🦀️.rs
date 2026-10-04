use super::*;
use crate::empty_wires_snapshot;

#[semio_framework_async_macros::async_test]
async fn dsl_round_trip_empty_document() {
    store::os_store::test_support::assert_dsl_round_trip(&empty_wires_snapshot());
}

/// ⚖️ LAW: the committed demo parent carries its seven identities and names its board child; the board itself (and the
/// relationships its edges carry) live in the child, never in the parent text.
#[semio_framework_async_macros::async_test]
async fn dsl_round_trip_metabolism_parent() {
    let document = crate::schema::metabolism_wires_example_snapshot().expect("valid metabolism parent");
    assert_eq!(document.wires_fixture.get("identities").and_then(|value| value.as_array()).map(|items| items.len()), Some(7));
    assert!(document.wires_fixture.get("relationships").is_none() && document.wires_fixture.get("board").is_none(), "the parent carries no board");
    assert_eq!(document.content.child_id, crate::WIRES_DEMO_CONTENT_ID);
    assert_eq!(parse_dsl(&print_dsl(&document)).expect("metabolism dsl round trip"), document);
    assert_eq!(print_dsl(&document), crate::examples::demo::PRIMARY_TEXT, "the committed asset is the canonical print");
}

/// ⚖️ LAW (codec retention): the parent's identity layer, meta and exact child handle survive the text codec of a fresh
/// decode; the board travels in the child, whose own text codec carries every node and edge.
#[semio_framework_async_macros::async_test]
async fn codec_retention_law_carries_the_parent_and_the_child_carries_the_board() {
    let document = crate::schema::metabolism_wires_example_snapshot().expect("valid metabolism parent");
    let back = <WiresSnapshot as store::ArtifactDsl>::parse_dsl(&<WiresSnapshot as store::ArtifactDsl>::print_dsl(&document)).expect("parse");
    assert_eq!((back.wires_fixture, back.meta, back.content), (document.wires_fixture.clone(), document.meta.clone(), document.content.clone()));
    let content = <crate::SemioGraphSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::CONTENT_TEXT).expect("demo board parses");
    assert_eq!((content.nodes.len(), content.edges.len()), (7, 9));
    assert_eq!(<crate::SemioGraphSnapshot as store::ArtifactDsl>::print_dsl(&content), crate::examples::demo::CONTENT_TEXT, "the committed board asset is the canonical print");
}
