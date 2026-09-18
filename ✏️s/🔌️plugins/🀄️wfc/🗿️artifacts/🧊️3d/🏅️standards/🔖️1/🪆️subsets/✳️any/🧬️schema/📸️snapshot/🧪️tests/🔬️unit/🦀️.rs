//! 🧪️ The snapshot's own laws: a fresh document names its schema, the addressing helpers find what
//! exists and nothing else, and the canonical insertion index keeps a collection sorted.

use super::*;

fn document() -> Wfc3dSnapshot {
    crate::examples::two_room_corridor::snapshot()
}

#[test]
fn a_default_document_is_empty_but_names_its_own_schema() {
    let snapshot = Wfc3dSnapshot::default();
    assert_eq!(snapshot.schema, WFC3D_DOCUMENT_SCHEMA);
    assert_eq!(snapshot.seed, 0);
    assert!(snapshot.slots.is_empty() && snapshot.edges.is_empty() && snapshot.tiles.is_empty() && snapshot.rules.is_empty());
}

#[test]
fn the_addressing_helpers_find_every_member_by_id() {
    let document = document();
    assert_eq!(slot_index(&document, "room-a"), Some(1));
    assert_eq!(slot_index(&document, "ghost"), None);
    assert_eq!(edge_index(&document, "edge-corridor-b"), Some(1));
    assert_eq!(tile_index(&document, "room"), Some(1));
    assert_eq!(rule_index(&document, "rule-room-corridor"), Some(0));
    assert_eq!(rule_index(&document, "rule-room-room"), None, "the corridor example admits exactly one pair and states nothing else");
}

/// 🔤️ The canonical index is a count of strictly-smaller ids, so inserting there keeps the vector
/// sorted and makes the position an author-independent function of the id alone.
#[test]
fn the_canonical_index_is_where_the_id_belongs_in_sorted_order() {
    let document = document();
    assert_eq!(canonical_slot_index(&document, "aardvark"), 0);
    assert_eq!(canonical_slot_index(&document, "room-c"), 3);
    assert_eq!(canonical_slot_index(&document, "room-ab"), 2, "\"room-ab\" sorts after \"corridor\" and \"room-a\", but BEFORE \"room-b\"");
    assert_eq!(canonical_edge_index(&document, "edge-a-b"), 0);
    assert_eq!(canonical_tile_index(&document, "stair"), 2);
    assert_eq!(canonical_rule_index(&document, "rule-aa"), 0);
}

/// 🥽️ A tile's media defaults to inline geometry, never to a child handle — a document that authored
/// no media must still be self-contained.
#[test]
fn tile_media_defaults_to_inline_geometry() {
    assert!(matches!(TileMedia3d::default(), TileMedia3d::Mesh { .. }));
    assert_eq!(Tile::default().weight, 0.0, "the derived Default is the zero record; every authored tile sets a real weight");
}

#[test]
fn every_example_slot_carries_a_positive_box() {
    for slot in &document().slots {
        assert!(slot.width > 0.0 && slot.height > 0.0 && slot.depth > 0.0, "slot {} must have a positive extent", slot.id);
    }
}
