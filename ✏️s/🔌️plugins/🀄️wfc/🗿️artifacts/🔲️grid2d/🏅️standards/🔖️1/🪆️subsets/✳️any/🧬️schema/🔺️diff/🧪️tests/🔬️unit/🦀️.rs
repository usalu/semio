//! 🧪️ `Grid2dDiff` laws — sparse apply, key-addressed merge, and the position-restoring insert.

use super::*;
use crate::schema::snapshot::{WfcCell2d, WfcPinnedCell2d, WfcTile2d};
use protocol::MutationDiff;

fn tile(id: &str) -> WfcTile2d {
    WfcTile2d { id: id.into(), weight: 1.0, ..Default::default() }
}

fn scene() -> Grid2dSnapshot {
    Grid2dSnapshot { width: 4, height: 4, tiles: vec![tile("a"), tile("c")], ..Default::default() }
}

#[test]
fn an_empty_diff_is_the_identity() {
    let base = scene();
    assert_eq!(Grid2dDiff::default().apply(&base).expect("empty applies"), base);
}

#[test]
fn an_insert_lands_at_the_index_the_diff_states() {
    let diff = Grid2dDiff { tiles_upserted: vec![(1, tile("b"))], ..Default::default() };
    let next = diff.apply(&scene()).expect("insert applies");
    assert_eq!(next.tiles.iter().map(|tile| tile.id.as_str()).collect::<Vec<_>>(), ["a", "b", "c"]);
}

#[test]
fn an_insert_at_the_wrong_index_for_an_existing_row_is_refused() {
    let diff = Grid2dDiff { tiles_upserted: vec![(1, tile("a"))], ..Default::default() };
    assert!(diff.apply(&scene()).is_err(), "replacing row 0 at index 1 must be refused");
}

#[test]
fn removing_a_row_the_document_never_held_is_refused() {
    let diff = Grid2dDiff { tiles_removed: vec!["zzz".into()], ..Default::default() };
    assert!(diff.apply(&scene()).is_err());
}

#[test]
fn cells_are_keyed_by_their_own_coordinates() {
    assert_eq!(cell_id(3, 7), "3,7");
    let diff = Grid2dDiff { pinned_upserted: vec![(0, WfcPinnedCell2d { x: 1, y: 2, tile_id: "a".into() })], ..Default::default() };
    let next = diff.apply(&scene()).expect("pin applies");
    assert_eq!(next.pinned.len(), 1);
    let removal = Grid2dDiff { pinned_removed: vec![cell_id(1, 2)], ..Default::default() };
    assert!(removal.apply(&next).expect("unpin applies").pinned.is_empty());
}

#[test]
fn absorb_lets_a_later_remove_win_over_an_earlier_upsert() {
    let mut first = Grid2dDiff { tiles_upserted: vec![(1, tile("b"))], ..Default::default() };
    first.absorb(Grid2dDiff { tiles_removed: vec!["b".into()], ..Default::default() });
    assert!(first.tiles_upserted.is_empty());
    assert_eq!(first.tiles_removed, ["b".to_string()]);
}

#[test]
fn absorb_lets_a_later_upsert_clear_an_earlier_remove() {
    let mut first = Grid2dDiff { masked_removed: vec![cell_id(0, 0)], ..Default::default() };
    first.absorb(Grid2dDiff { masked_upserted: vec![(0, WfcCell2d { x: 0, y: 0 })], ..Default::default() });
    assert!(first.masked_removed.is_empty());
    assert_eq!(first.masked_upserted.len(), 1);
}

#[test]
fn every_scalar_lane_is_written_when_it_is_stated() {
    let diff = Grid2dDiff { seed: Some(9), width: Some(2), height: Some(3), cell_width: Some(1.5), cell_height: Some(2.5), periodic_x: Some(true), periodic_y: Some(true), ..Default::default() };
    let next = diff.apply(&scene()).expect("scalars apply");
    assert_eq!((next.seed, next.width, next.height, next.cell_width, next.cell_height, next.periodic_x, next.periodic_y), (9, 2, 3, 1.5, 2.5, true, true));
}
