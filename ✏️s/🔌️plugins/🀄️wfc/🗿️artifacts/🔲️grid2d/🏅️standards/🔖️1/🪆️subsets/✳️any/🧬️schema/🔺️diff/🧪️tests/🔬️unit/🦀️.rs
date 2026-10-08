//! 🧪️ `Grid2dDiff` — apply, absorb and inverse laws over the id-keyed tile rows.

use super::*;
use protocol::{DiffAlgebra, MutationDiff};

fn tile(id: &str, weight: f64) -> WfcTile2d {
    WfcTile2d { id: id.into(), weight, ..Default::default() }
}

fn base() -> Grid2dSnapshot {
    Grid2dSnapshot { tiles: vec![tile("b", 1.0), tile("d", 2.0)], ..Default::default() }
}

fn rows(delta: Grid2dTilesDelta) -> Grid2dDiff {
    Grid2dDiff { tiles: delta, ..Default::default() }
}

fn reweigh(id: &str, weight: f64) -> Grid2dTilesModification {
    Grid2dTilesModification { id: id.into(), patch: Grid2dTilePatch { weight: Some(weight), ..Default::default() } }
}

fn absorbed(first: Grid2dDiff, second: Grid2dDiff) -> Grid2dDiff {
    let mut sigma = first;
    MutationDiff::<Grid2dSnapshot>::absorb(&mut sigma, second);
    sigma
}

fn assert_absorb_law(base: &Grid2dSnapshot, first: &Grid2dDiff, second: &Grid2dDiff) -> Grid2dDiff {
    let sigma = absorbed(first.clone(), second.clone());
    let mid = protocol::apply_diff(first, base).expect("first applies");
    let sequential = protocol::apply_diff(second, &mid).expect("second applies");
    assert_eq!(protocol::apply_diff(&sigma, base).expect("absorbed applies"), sequential, "absorb must equal sequential application");
    sigma
}

/// 🕳️ The identity delta leaves the document untouched.
#[test]
fn empty_diff_is_the_identity() {
    let base = base();
    assert_eq!(protocol::apply_diff(&Grid2dDiff::default(), &base).expect("identity applies"), base);
    assert!(DiffAlgebra::<Grid2dSnapshot>::is_empty(&Grid2dDiff::default()));
}

/// 📍 Inserted rows take their stated after-list indices; the canonical position of a new row is where `insert_at` puts it.
#[test]
fn inserted_rows_take_their_after_indices() {
    assert_eq!(Grid2dRow::insert_at(&base().tiles, &tile("c", 3.0)), 1);
    let diff = rows(Grid2dTilesDelta { inserted: vec![Grid2dTilesInsertion { index: 0, row: tile("a", 4.0) }, Grid2dTilesInsertion { index: 2, row: tile("c", 3.0) }], ..Default::default() });
    let after = protocol::apply_diff(&diff, &base()).expect("inserts apply");
    let ids: Vec<&str> = after.tiles.iter().map(|tile| tile.id.as_str()).collect();
    assert_eq!(ids, ["a", "b", "c", "d"]);
}

/// 🔀 A row created and then deleted cancels out: nothing remains of it.
#[test]
fn create_then_delete_cancels() {
    let base = base();
    let created = tile("c", 3.0);
    let sigma = assert_absorb_law(&base, &rows(Grid2dTilesDelta::insertion(1, created.clone())), &rows(Grid2dTilesDelta::removal(&[tile("b", 1.0), created, tile("d", 2.0)], 1)));
    assert!(DiffAlgebra::<Grid2dSnapshot>::is_empty(&sigma));
}

/// 🔀 A base row patched and then deleted leaves only the deletion at its base index.
#[test]
fn patch_then_delete_leaves_the_deletion() {
    let base = base();
    let sigma = assert_absorb_law(&base, &rows(Grid2dTilesDelta { modified: vec![reweigh("b", 9.0)], ..Default::default() }), &rows(Grid2dTilesDelta::removal(&[tile("b", 9.0), tile("d", 2.0)], 0)));
    assert_eq!(sigma.tiles.removed, vec![Grid2dTilesRemoval { id: "b".into(), index: 0 }]);
    assert!(sigma.tiles.modified.is_empty() && sigma.tiles.inserted.is_empty());
}

/// 🔀 Two patches of one row coalesce into one patch holding the later values.
#[test]
fn patch_then_patch_coalesces() {
    let base = base();
    let sigma = assert_absorb_law(&base, &rows(Grid2dTilesDelta { modified: vec![reweigh("b", 8.0)], ..Default::default() }), &rows(Grid2dTilesDelta { modified: vec![reweigh("b", 9.0)], ..Default::default() }));
    assert_eq!(sigma.tiles.modified.len(), 1);
    assert_eq!(sigma.tiles.modified[0].patch.weight, Some(9.0));
}

/// 🔀 A row deleted and created again is one replacement: removed at its base index and inserted at its after index.
#[test]
fn delete_then_create_replaces() {
    let base = base();
    let sigma = assert_absorb_law(&base, &rows(Grid2dTilesDelta::removal(&base.tiles, 0)), &rows(Grid2dTilesDelta::insertion(0, tile("b", 7.0))));
    assert_eq!((sigma.tiles.removed, sigma.tiles.inserted), (vec![Grid2dTilesRemoval { id: "b".into(), index: 0 }], vec![Grid2dTilesInsertion { index: 0, row: tile("b", 7.0) }]));
}

/// 🔀 A row created and then patched is created already patched.
#[test]
fn create_then_patch_folds_into_the_row() {
    let base = base();
    let sigma = assert_absorb_law(&base, &rows(Grid2dTilesDelta::insertion(1, tile("c", 3.0))), &rows(Grid2dTilesDelta { modified: vec![reweigh("c", 4.0)], ..Default::default() }));
    assert_eq!(sigma.tiles.inserted, vec![Grid2dTilesInsertion { index: 1, row: tile("c", 4.0) }]);
    assert!(sigma.tiles.modified.is_empty());
}

/// 🔁️ The inverse diff restores the base exactly — rows, positions and fields.
#[test]
fn inverse_restores_the_base() {
    let base = base();
    let diff = rows(Grid2dTilesDelta { removed: vec![Grid2dTilesRemoval { id: "b".into(), index: 0 }], inserted: vec![Grid2dTilesInsertion { index: 0, row: tile("a", 5.0) }], modified: vec![reweigh("d", 6.0)], ..Default::default() });
    let after = protocol::apply_diff(&diff, &base).expect("diff applies");
    let inverse = DiffAlgebra::<Grid2dSnapshot>::inverse(&diff, &base);
    assert_eq!(protocol::apply_diff(&inverse, &after).expect("inverse applies"), base);
}

/// 🎯️ Removing, inserting and moving a MIDDLE row are each undone by the inverse diff at the row's original position.
#[test]
fn inverse_restores_middle_rows() {
    let mut base = base();
    base.tiles.insert(1, tile("c", 1.5));
    base.tiles.push(tile("f", 3.0));
    for delta in [
        Grid2dTilesDelta::removal(&base.tiles, 1),
        Grid2dTilesDelta::insertion(Grid2dRow::insert_at(&base.tiles, &tile("e", 4.0)), tile("e", 4.0)),
        Grid2dTilesDelta { moved: vec![Grid2dTilesRelocation { id: "c".into(), from: 1, to: 3 }], ..Default::default() },
    ] {
        let diff = rows(delta);
        let after = protocol::apply_diff(&diff, &base).expect("diff applies");
        let inverse = DiffAlgebra::<Grid2dSnapshot>::inverse(&diff, &base);
        assert_eq!(protocol::apply_diff(&inverse, &after).expect("inverse applies"), base);
    }
}

/// 🚫️ Removing a row that is not at its stated base index is refused, not silently ignored.
#[test]
fn apply_refuses_a_missing_removal() {
    assert!(protocol::apply_diff(&rows(Grid2dTilesDelta { removed: vec![Grid2dTilesRemoval { id: "ghost".into(), index: 0 }], ..Default::default() }), &base()).is_err());
}

/// 📌️ Cells are keyed by their own coordinates and inserted at their stated row-major indices.
#[test]
fn cells_are_keyed_by_their_coordinates_and_land_row_major() {
    assert_eq!(cell_id(3, 7), "3,7");
    let pin = |x: u32, y: u32| WfcPinnedCell2d { x, y, tile_id: "b".into() };
    let diff = Grid2dDiff { pinned: Grid2dPinnedDelta { inserted: vec![Grid2dPinnedInsertion { index: 2, row: pin(2, 1) }, Grid2dPinnedInsertion { index: 0, row: pin(0, 0) }, Grid2dPinnedInsertion { index: 1, row: pin(1, 1) }], ..Default::default() }, ..Default::default() };
    let next = protocol::apply_diff(&diff, &base()).expect("pins apply");
    let order: Vec<(u32, u32)> = next.pinned.iter().map(|cell| (cell.x, cell.y)).collect();
    assert_eq!(order, [(0, 0), (1, 1), (2, 1)]);
    let removal = Grid2dDiff { pinned: Grid2dPinnedDelta::removal(&next.pinned, 1), ..Default::default() };
    assert_eq!(protocol::apply_diff(&removal, &next).expect("unpin applies").pinned.len(), 2);
}
