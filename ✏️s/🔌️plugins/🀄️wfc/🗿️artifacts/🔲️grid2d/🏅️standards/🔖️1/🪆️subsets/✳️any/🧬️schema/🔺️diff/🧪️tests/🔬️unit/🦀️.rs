//! 🧪️ `Grid2dDiff` — apply, absorb, inverse and between laws over the id-keyed tile rows.

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

fn reweigh(id: &str, weight: f64) -> Grid2dRowPatch<Grid2dTilePatch> {
    Grid2dRowPatch { id: id.into(), patch: Grid2dTilePatch { weight: Some(weight), ..Default::default() } }
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

/// 📍 An added row lands at its canonical id position, wherever the diff was built.
#[test]
fn added_rows_land_at_their_canonical_position() {
    let after = protocol::apply_diff(&rows(Grid2dRows { added: vec![tile("c", 3.0), tile("a", 4.0)], ..Default::default() }), &base()).expect("adds apply");
    let ids: Vec<&str> = after.tiles.iter().map(|tile| tile.id.as_str()).collect();
    assert_eq!(ids, ["a", "b", "c", "d"]);
}

/// 🔀 A row created and then deleted cancels out: nothing remains of it.
#[test]
fn create_then_delete_cancels() {
    let base = base();
    let sigma = assert_absorb_law(&base, &rows(Grid2dRows { added: vec![tile("c", 3.0)], ..Default::default() }), &rows(Grid2dRows { removed: vec!["c".into()], ..Default::default() }));
    assert!(DiffAlgebra::<Grid2dSnapshot>::is_empty(&sigma));
}

/// 🔀 A base row patched and then deleted leaves only the deletion.
#[test]
fn patch_then_delete_leaves_the_deletion() {
    let base = base();
    let sigma = assert_absorb_law(&base, &rows(Grid2dRows { patched: vec![reweigh("b", 9.0)], ..Default::default() }), &rows(Grid2dRows { removed: vec!["b".into()], ..Default::default() }));
    assert_eq!(sigma.tiles.removed, vec!["b".to_string()]);
    assert!(sigma.tiles.patched.is_empty() && sigma.tiles.added.is_empty());
}

/// 🔀 Two patches of one row coalesce into one patch holding the later values.
#[test]
fn patch_then_patch_coalesces() {
    let base = base();
    let sigma = assert_absorb_law(&base, &rows(Grid2dRows { patched: vec![reweigh("b", 8.0)], ..Default::default() }), &rows(Grid2dRows { patched: vec![reweigh("b", 9.0)], ..Default::default() }));
    assert_eq!(sigma.tiles.patched.len(), 1);
    assert_eq!(sigma.tiles.patched[0].patch.weight, Some(9.0));
}

/// 🔀 A row deleted and created again is one replacement: removed and added together.
#[test]
fn delete_then_create_replaces() {
    let base = base();
    let sigma = assert_absorb_law(&base, &rows(Grid2dRows { removed: vec!["b".into()], ..Default::default() }), &rows(Grid2dRows { added: vec![tile("b", 7.0)], ..Default::default() }));
    assert_eq!((sigma.tiles.removed, sigma.tiles.added), (vec!["b".to_string()], vec![tile("b", 7.0)]));
}

/// 🔀 A row created and then patched is created already patched.
#[test]
fn create_then_patch_folds_into_the_row() {
    let base = base();
    let sigma = assert_absorb_law(&base, &rows(Grid2dRows { added: vec![tile("c", 3.0)], ..Default::default() }), &rows(Grid2dRows { patched: vec![reweigh("c", 4.0)], ..Default::default() }));
    assert_eq!(sigma.tiles.added, vec![tile("c", 4.0)]);
    assert!(sigma.tiles.patched.is_empty());
}

/// 🔁️ The inverse diff restores the base exactly — rows, positions and fields.
#[test]
fn inverse_restores_the_base() {
    let base = base();
    let diff = rows(Grid2dRows { removed: vec!["b".into()], added: vec![tile("a", 5.0)], patched: vec![reweigh("d", 6.0)] });
    let after = protocol::apply_diff(&diff, &base).expect("diff applies");
    let inverse = DiffAlgebra::<Grid2dSnapshot>::inverse(&diff, &base);
    assert_eq!(protocol::apply_diff(&inverse, &after).expect("inverse applies"), base);
}

/// 🧭️ `between` reaches the other document, and is empty between equal documents.
#[test]
fn between_reaches_the_other_document() {
    let base = base();
    let other = Grid2dSnapshot { tiles: vec![tile("a", 5.0), tile("b", 1.5), tile("c", 3.0)], ..Default::default() };
    let delta = <Grid2dDiff as DiffAlgebra<Grid2dSnapshot>>::between(&base, &other);
    assert_eq!(protocol::apply_diff(&delta, &base).expect("between applies"), other);
    assert!(DiffAlgebra::<Grid2dSnapshot>::is_empty(&<Grid2dDiff as DiffAlgebra<Grid2dSnapshot>>::between(&base, &base)));
}

/// 🚫️ Removing a row the base never held is refused, not silently ignored.
#[test]
fn apply_refuses_a_missing_removal() {
    assert!(protocol::apply_diff(&rows(Grid2dRows { removed: vec!["ghost".into()], ..Default::default() }), &base()).is_err());
}
