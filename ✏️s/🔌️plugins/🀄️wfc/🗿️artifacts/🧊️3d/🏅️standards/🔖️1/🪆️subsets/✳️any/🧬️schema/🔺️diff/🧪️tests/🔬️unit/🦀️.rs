//! 🧪️ `Wfc3dDiff` — apply, absorb and inverse laws over the id-keyed tile rows.

use super::*;
use protocol::{DiffAlgebra, MutationDiff};

fn tile(id: &str, weight: f64) -> Tile {
    Tile { id: id.into(), weight, ..Default::default() }
}

fn base() -> Wfc3dSnapshot {
    Wfc3dSnapshot { tiles: vec![tile("b", 1.0), tile("d", 2.0)], ..Default::default() }
}

fn rows(delta: Wfc3dTilesDelta) -> Wfc3dDiff {
    Wfc3dDiff { tiles: delta, ..Default::default() }
}

fn reweigh(id: &str, weight: f64) -> Wfc3dTilesModification {
    Wfc3dTilesModification { id: id.into(), patch: Wfc3dTilePatch { weight: Some(weight), ..Default::default() } }
}

fn absorbed(first: Wfc3dDiff, second: Wfc3dDiff) -> Wfc3dDiff {
    let mut sigma = first;
    MutationDiff::<Wfc3dSnapshot>::absorb(&mut sigma, second);
    sigma
}

fn assert_absorb_law(base: &Wfc3dSnapshot, first: &Wfc3dDiff, second: &Wfc3dDiff) -> Wfc3dDiff {
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
    assert_eq!(protocol::apply_diff(&Wfc3dDiff::default(), &base).expect("identity applies"), base);
    assert!(DiffAlgebra::<Wfc3dSnapshot>::is_empty(&Wfc3dDiff::default()));
}

/// 📍 Inserted rows take their stated after-list indices; the canonical position of a new row is where `insert_at` puts it.
#[test]
fn inserted_rows_take_their_after_indices() {
    assert_eq!(Wfc3dRow::insert_at(&base().tiles, &tile("c", 3.0)), 1);
    let diff = rows(Wfc3dTilesDelta { inserted: vec![Wfc3dTilesInsertion { index: 0, row: tile("a", 4.0) }, Wfc3dTilesInsertion { index: 2, row: tile("c", 3.0) }], ..Default::default() });
    let after = protocol::apply_diff(&diff, &base()).expect("inserts apply");
    let ids: Vec<&str> = after.tiles.iter().map(|tile| tile.id.as_str()).collect();
    assert_eq!(ids, ["a", "b", "c", "d"]);
}

/// 🔀 A row created and then deleted cancels out: nothing remains of it.
#[test]
fn create_then_delete_cancels() {
    let base = base();
    let created = tile("c", 3.0);
    let sigma = assert_absorb_law(&base, &rows(Wfc3dTilesDelta::insertion(1, created.clone())), &rows(Wfc3dTilesDelta::removal(&[tile("b", 1.0), created, tile("d", 2.0)], 1)));
    assert!(DiffAlgebra::<Wfc3dSnapshot>::is_empty(&sigma));
}

/// 🔀 A base row patched and then deleted leaves only the deletion at its base index.
#[test]
fn patch_then_delete_leaves_the_deletion() {
    let base = base();
    let sigma = assert_absorb_law(&base, &rows(Wfc3dTilesDelta { modified: vec![reweigh("b", 9.0)], ..Default::default() }), &rows(Wfc3dTilesDelta::removal(&[tile("b", 9.0), tile("d", 2.0)], 0)));
    assert_eq!(sigma.tiles.removed, vec![Wfc3dTilesRemoval { id: "b".into(), index: 0 }]);
    assert!(sigma.tiles.modified.is_empty() && sigma.tiles.inserted.is_empty());
}

/// 🔀 Two patches of one row coalesce into one patch holding the later values.
#[test]
fn patch_then_patch_coalesces() {
    let base = base();
    let sigma = assert_absorb_law(&base, &rows(Wfc3dTilesDelta { modified: vec![reweigh("b", 8.0)], ..Default::default() }), &rows(Wfc3dTilesDelta { modified: vec![reweigh("b", 9.0)], ..Default::default() }));
    assert_eq!(sigma.tiles.modified.len(), 1);
    assert_eq!(sigma.tiles.modified[0].patch.weight, Some(9.0));
}

/// 🔀 A row deleted and created again is one replacement: removed at its base index and inserted at its after index.
#[test]
fn delete_then_create_replaces() {
    let base = base();
    let sigma = assert_absorb_law(&base, &rows(Wfc3dTilesDelta::removal(&base.tiles, 0)), &rows(Wfc3dTilesDelta::insertion(0, tile("b", 7.0))));
    assert_eq!((sigma.tiles.removed, sigma.tiles.inserted), (vec![Wfc3dTilesRemoval { id: "b".into(), index: 0 }], vec![Wfc3dTilesInsertion { index: 0, row: tile("b", 7.0) }]));
}

/// 🔀 A row created and then patched is created already patched.
#[test]
fn create_then_patch_folds_into_the_row() {
    let base = base();
    let sigma = assert_absorb_law(&base, &rows(Wfc3dTilesDelta::insertion(1, tile("c", 3.0))), &rows(Wfc3dTilesDelta { modified: vec![reweigh("c", 4.0)], ..Default::default() }));
    assert_eq!(sigma.tiles.inserted, vec![Wfc3dTilesInsertion { index: 1, row: tile("c", 4.0) }]);
    assert!(sigma.tiles.modified.is_empty());
}

/// 🔁️ The inverse diff restores the base exactly — rows, positions and fields.
#[test]
fn inverse_restores_the_base() {
    let base = base();
    let diff = rows(Wfc3dTilesDelta { removed: vec![Wfc3dTilesRemoval { id: "b".into(), index: 0 }], inserted: vec![Wfc3dTilesInsertion { index: 0, row: tile("a", 5.0) }], modified: vec![reweigh("d", 6.0)], ..Default::default() });
    let after = protocol::apply_diff(&diff, &base).expect("diff applies");
    let inverse = DiffAlgebra::<Wfc3dSnapshot>::inverse(&diff, &base);
    assert_eq!(protocol::apply_diff(&inverse, &after).expect("inverse applies"), base);
}

/// 🎯️ Removing, inserting and moving a MIDDLE row are each undone by the inverse diff at the row's original position.
#[test]
fn inverse_restores_middle_rows() {
    let mut base = base();
    base.tiles.insert(1, tile("c", 1.5));
    base.tiles.push(tile("f", 3.0));
    for delta in [
        Wfc3dTilesDelta::removal(&base.tiles, 1),
        Wfc3dTilesDelta::insertion(Wfc3dRow::insert_at(&base.tiles, &tile("e", 4.0)), tile("e", 4.0)),
        Wfc3dTilesDelta { moved: vec![Wfc3dTilesRelocation { id: "c".into(), from: 1, to: 3 }], ..Default::default() },
    ] {
        let diff = rows(delta);
        let after = protocol::apply_diff(&diff, &base).expect("diff applies");
        let inverse = DiffAlgebra::<Wfc3dSnapshot>::inverse(&diff, &base);
        assert_eq!(protocol::apply_diff(&inverse, &after).expect("inverse applies"), base);
    }
}

/// 🚫️ Removing a row that is not at its stated base index is refused, not silently ignored.
#[test]
fn apply_refuses_a_missing_removal() {
    assert!(protocol::apply_diff(&rows(Wfc3dTilesDelta { removed: vec![Wfc3dTilesRemoval { id: "ghost".into(), index: 0 }], ..Default::default() }), &base()).is_err());
}
