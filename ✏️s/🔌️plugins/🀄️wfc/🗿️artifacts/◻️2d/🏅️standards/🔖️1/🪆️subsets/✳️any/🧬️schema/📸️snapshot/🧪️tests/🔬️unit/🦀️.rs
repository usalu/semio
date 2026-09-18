//! 🧪️ `Wfc2dSnapshot` — default, addressing and relation-universe laws.

use crate::schema::snapshot::{relations, slot_index, tile_index, Wfc2dSnapshot, WFC_2D_DEFAULT_RELATION, WFC_2D_DOCUMENT_SCHEMA};

#[test]
fn default_carries_the_schema_and_nothing_else() {
    let document = Wfc2dSnapshot::default();
    assert_eq!(document.schema, WFC_2D_DOCUMENT_SCHEMA);
    assert_eq!(document.seed, 0);
    assert!(document.slots.is_empty() && document.edges.is_empty() && document.tiles.is_empty() && document.rules.is_empty());
}

#[test]
fn addressing_finds_authored_rows() {
    let document = crate::examples::two_room_corridor::document();
    assert_eq!(slot_index(&document, "room-a"), Some(1));
    assert_eq!(slot_index(&document, "nope"), None);
    assert_eq!(tile_index(&document, "room"), Some(1));
}

#[test]
fn relations_are_distinct_and_ascending() {
    let document = crate::examples::wall_roof_facade_strip::document();
    assert_eq!(relations(&document), vec!["above".to_string(), "beside".to_string()]);
    assert_eq!(relations(&crate::examples::two_room_corridor::document()), vec![WFC_2D_DEFAULT_RELATION.to_string()]);
}

/// 🎒️ The native pack envelope round-trips every bundled example byte for byte.
#[test]
fn pack_round_trips_every_example() {
    for document in crate::examples::documents() {
        let bytes = <Wfc2dSnapshot as store::ArtifactPack>::encode_pack(&document);
        let decoded = <Wfc2dSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("pack decodes");
        assert_eq!(decoded, document);
    }
}
