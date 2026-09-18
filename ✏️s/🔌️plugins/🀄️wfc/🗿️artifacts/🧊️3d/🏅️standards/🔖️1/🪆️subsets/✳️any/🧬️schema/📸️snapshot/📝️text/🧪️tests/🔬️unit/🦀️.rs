//! 🧪️ The text facet's round-trip law: print → parse is the identity on every bundled example, and
//! the printed body carries this artifact's own envelope.

use super::*;

#[test]
fn every_example_round_trips_through_the_dsl() {
    for document in [crate::examples::two_room_corridor::snapshot(), crate::examples::wall_roof_facade_strip::snapshot(), crate::examples::tower_stack::snapshot()] {
        let text = print_dsl(&document);
        assert!(!text.trim().is_empty());
        assert_eq!(parse_dsl(&text).expect("printed dsl parses"), document);
    }
}

#[test]
fn an_empty_body_decodes_to_the_default_document() {
    assert_eq!(parse_dsl("").expect("an empty body is a document"), Wfc3dSnapshot::default());
}

#[test]
fn the_printed_document_carries_the_wfc3d_envelope() {
    let text = print_dsl(&crate::examples::two_room_corridor::snapshot());
    assert!(text.starts_with("semio wfc.wfc3d"), "the envelope preamble names this artifact: {}", text.lines().next().unwrap_or_default());
}

/// 🖼️ The one field that is not a derive: tile media bridges through `DslValue`, so a document whose
/// tiles carry real geometry has to survive the bridge unchanged.
#[test]
fn tile_media_survives_the_dsl_value_bridge() {
    let document = crate::examples::tower_stack::snapshot();
    let parsed = parse_dsl(&print_dsl(&document)).expect("tower parses");
    assert_eq!(parsed.tiles, document.tiles);
}
