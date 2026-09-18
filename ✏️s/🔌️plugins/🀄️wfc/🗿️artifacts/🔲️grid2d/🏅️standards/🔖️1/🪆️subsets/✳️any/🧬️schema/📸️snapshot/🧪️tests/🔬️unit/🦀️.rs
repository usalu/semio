//! 🧪️ `Grid2dSnapshot` laws — the document's own addressing, geometry and palette-stream helpers.

use super::*;

fn scene() -> Grid2dSnapshot {
    Grid2dSnapshot { width: 3, height: 2, cell_width: 4.0, cell_height: 5.0, ..Default::default() }
}

#[test]
fn the_default_document_states_its_own_schema() {
    assert_eq!(Grid2dSnapshot::default().schema, WFC_GRID2D_DOCUMENT_SCHEMA);
}

#[test]
fn bounds_follow_the_authored_extent() {
    let document = scene();
    assert!(in_bounds(&document, 2, 1));
    assert!(!in_bounds(&document, 3, 1));
    assert!(!in_bounds(&document, 2, 2));
}

#[test]
fn a_cell_rect_is_the_authored_cell_size_at_its_own_origin() {
    assert_eq!(cell_rect(&scene(), 2, 1), (8.0, 5.0, 4.0, 5.0));
}

#[test]
fn cells_sort_row_major() {
    assert!(cell_key(0, 1) > cell_key(2, 0));
}

#[test]
fn every_direction_pairs_with_its_own_opposite() {
    for direction in WfcDirection2d::ALL {
        assert_eq!(direction.opposite().opposite(), direction);
        let (dx, dy) = direction.offset();
        let (ix, iy) = direction.opposite().offset();
        assert_eq!((dx + ix, dy + iy), (0, 0), "{direction:?} and its opposite must cancel");
    }
}

#[test]
fn the_palette_stream_round_trips() {
    for indices in [vec![], vec![0u8], vec![0, 1], vec![0, 1, 2], vec![3, 2, 1, 0, 1]] {
        assert_eq!(decode_palette_indices(&encode_palette_indices(&indices)), indices);
    }
}

#[test]
fn a_malformed_palette_stream_yields_the_prefix_it_could_read() {
    let stream = format!("{}!!!!", encode_palette_indices(&[7, 7, 7]));
    assert_eq!(decode_palette_indices(&stream), vec![7, 7, 7]);
}

#[test]
fn addressing_finds_every_collection_by_its_own_key() {
    let mut document = scene();
    document.tiles.push(WfcTile2d { id: "a".into(), weight: 1.0, ..Default::default() });
    document.rules.push(WfcAdjacencyRule2d { id: "r".into(), ..Default::default() });
    document.pinned.push(WfcPinnedCell2d { x: 1, y: 1, tile_id: "a".into() });
    document.masked.push(WfcCell2d { x: 2, y: 0 });
    assert_eq!(tile_index(&document, "a"), Some(0));
    assert_eq!(rule_index(&document, "r"), Some(0));
    assert_eq!(pinned_index(&document, 1, 1), Some(0));
    assert_eq!(masked_index(&document, 2, 0), Some(0));
    assert_eq!(tile_index(&document, "missing"), None);
}
