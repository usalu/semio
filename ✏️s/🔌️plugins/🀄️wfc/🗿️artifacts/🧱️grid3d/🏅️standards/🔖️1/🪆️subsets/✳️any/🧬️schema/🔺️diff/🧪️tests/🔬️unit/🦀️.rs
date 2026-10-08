//! 🧪️ `Grid3dDiff` — apply, absorb, inverse and between laws over the id-keyed tile rows.

use super::*;
use protocol::{DiffAlgebra, MutationDiff};

fn tile(id: &str, weight: f64) -> Grid3dTile {
    Grid3dTile { id: id.into(), weight, ..Default::default() }
}

fn base() -> Grid3dSnapshot {
    Grid3dSnapshot { tiles: vec![tile("b", 1.0), tile("d", 2.0)], ..Default::default() }
}

fn rows(delta: Grid3dTilesDelta) -> Grid3dDiff {
    Grid3dDiff { tiles: delta, ..Default::default() }
}

fn reweigh(id: &str, weight: f64) -> Grid3dRowPatch<Grid3dTilePatch> {
    Grid3dRowPatch { id: id.into(), patch: Grid3dTilePatch { weight: Some(weight), ..Default::default() } }
}

fn absorbed(first: Grid3dDiff, second: Grid3dDiff) -> Grid3dDiff {
    let mut sigma = first;
    MutationDiff::<Grid3dSnapshot>::absorb(&mut sigma, second);
    sigma
}

fn assert_absorb_law(base: &Grid3dSnapshot, first: &Grid3dDiff, second: &Grid3dDiff) -> Grid3dDiff {
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
    assert_eq!(protocol::apply_diff(&Grid3dDiff::default(), &base).expect("identity applies"), base);
    assert!(DiffAlgebra::<Grid3dSnapshot>::is_empty(&Grid3dDiff::default()));
}

/// 📍 An added row lands at its canonical id position, wherever the diff was built.
#[test]
fn added_rows_land_at_their_canonical_position() {
    let after = protocol::apply_diff(&rows(Grid3dRows { added: vec![tile("c", 3.0), tile("a", 4.0)], ..Default::default() }), &base()).expect("adds apply");
    let ids: Vec<&str> = after.tiles.iter().map(|tile| tile.id.as_str()).collect();
    assert_eq!(ids, ["a", "b", "c", "d"]);
}

/// 🔀 A row created and then deleted cancels out: nothing remains of it.
#[test]
fn create_then_delete_cancels() {
    let base = base();
    let sigma = assert_absorb_law(&base, &rows(Grid3dRows { added: vec![tile("c", 3.0)], ..Default::default() }), &rows(Grid3dRows { removed: vec!["c".into()], ..Default::default() }));
    assert!(DiffAlgebra::<Grid3dSnapshot>::is_empty(&sigma));
}

/// 🔀 A base row patched and then deleted leaves only the deletion.
#[test]
fn patch_then_delete_leaves_the_deletion() {
    let base = base();
    let sigma = assert_absorb_law(&base, &rows(Grid3dRows { patched: vec![reweigh("b", 9.0)], ..Default::default() }), &rows(Grid3dRows { removed: vec!["b".into()], ..Default::default() }));
    assert_eq!(sigma.tiles.removed, vec!["b".to_string()]);
    assert!(sigma.tiles.patched.is_empty() && sigma.tiles.added.is_empty());
}

/// 🔀 Two patches of one row coalesce into one patch holding the later values.
#[test]
fn patch_then_patch_coalesces() {
    let base = base();
    let sigma = assert_absorb_law(&base, &rows(Grid3dRows { patched: vec![reweigh("b", 8.0)], ..Default::default() }), &rows(Grid3dRows { patched: vec![reweigh("b", 9.0)], ..Default::default() }));
    assert_eq!(sigma.tiles.patched.len(), 1);
    assert_eq!(sigma.tiles.patched[0].patch.weight, Some(9.0));
}

/// 🔀 A row deleted and created again is one replacement: removed and added together.
#[test]
fn delete_then_create_replaces() {
    let base = base();
    let sigma = assert_absorb_law(&base, &rows(Grid3dRows { removed: vec!["b".into()], ..Default::default() }), &rows(Grid3dRows { added: vec![tile("b", 7.0)], ..Default::default() }));
    assert_eq!((sigma.tiles.removed, sigma.tiles.added), (vec!["b".to_string()], vec![tile("b", 7.0)]));
}

/// 🔀 A row created and then patched is created already patched.
#[test]
fn create_then_patch_folds_into_the_row() {
    let base = base();
    let sigma = assert_absorb_law(&base, &rows(Grid3dRows { added: vec![tile("c", 3.0)], ..Default::default() }), &rows(Grid3dRows { patched: vec![reweigh("c", 4.0)], ..Default::default() }));
    assert_eq!(sigma.tiles.added, vec![tile("c", 4.0)]);
    assert!(sigma.tiles.patched.is_empty());
}

/// 🔁️ The inverse diff restores the base exactly — rows, positions and fields.
#[test]
fn inverse_restores_the_base() {
    let base = base();
    let diff = rows(Grid3dRows { removed: vec!["b".into()], added: vec![tile("a", 5.0)], patched: vec![reweigh("d", 6.0)] });
    let after = protocol::apply_diff(&diff, &base).expect("diff applies");
    let inverse = DiffAlgebra::<Grid3dSnapshot>::inverse(&diff, &base);
    assert_eq!(protocol::apply_diff(&inverse, &after).expect("inverse applies"), base);
}

/// 🎯️ Removing or creating a middle row is undone by the inverse diff at the row's original position.
#[test]
fn inverse_restores_middle_rows() {
    let mut base = base();
    base.tiles.insert(1, tile("c", 1.5));
    base.tiles.push(tile("f", 3.0));
    for diff in [rows(Grid3dRows { removed: vec!["c".into()], ..Default::default() }), rows(Grid3dRows { added: vec![tile("e", 4.0)], ..Default::default() })] {
        let after = protocol::apply_diff(&diff, &base).expect("diff applies");
        let inverse = DiffAlgebra::<Grid3dSnapshot>::inverse(&diff, &base);
        assert_eq!(protocol::apply_diff(&inverse, &after).expect("inverse applies"), base);
    }
}

/// 🧭️ `between` reaches the other document, and is empty between equal documents.
#[test]
fn between_reaches_the_other_document() {
    let base = base();
    let other = Grid3dSnapshot { tiles: vec![tile("a", 5.0), tile("b", 1.5), tile("c", 3.0)], ..Default::default() };
    let delta = <Grid3dDiff as DiffAlgebra<Grid3dSnapshot>>::between(&base, &other);
    assert_eq!(protocol::apply_diff(&delta, &base).expect("between applies"), other);
    assert!(DiffAlgebra::<Grid3dSnapshot>::is_empty(&<Grid3dDiff as DiffAlgebra<Grid3dSnapshot>>::between(&base, &base)));
}

/// 🚫️ Removing a row the base never held is refused, not silently ignored.
#[test]
fn apply_refuses_a_missing_removal() {
    assert!(protocol::apply_diff(&rows(Grid3dRows { removed: vec!["ghost".into()], ..Default::default() }), &base()).is_err());
}

/// 📏 An axis patch resizes and sets cells, its inverse restores the base sizes, and later patches coalesce per cell and length.
#[test]
fn axis_patches_resize_set_restore_and_coalesce() {
    let size = |index: u32, size: f64| Grid3dAxisSize { index, size };
    let base = Grid3dSnapshot { cell_sizes_x: vec![1.0, 2.0, 3.0], ..Default::default() };
    let grow = Grid3dDiff { cell_sizes_x: Some(Grid3dAxisPatch { length: Some(4), sizes: vec![size(1, 5.0), size(3, 3.0)] }), ..Default::default() };
    let after = protocol::apply_diff(&grow, &base).expect("grow applies");
    assert_eq!(after.cell_sizes_x, vec![1.0, 5.0, 3.0, 3.0]);
    let inverse = DiffAlgebra::<Grid3dSnapshot>::inverse(&grow, &base);
    assert_eq!(protocol::apply_diff(&inverse, &after).expect("inverse applies").cell_sizes_x, base.cell_sizes_x);
    let shrink = Grid3dDiff { cell_sizes_x: Some(Grid3dAxisPatch { length: Some(2), sizes: vec![] }), ..Default::default() };
    let regrow = Grid3dDiff { cell_sizes_x: Some(Grid3dAxisPatch { length: Some(3), sizes: vec![size(2, 9.0)] }), ..Default::default() };
    let sigma = assert_absorb_law(&base, &shrink, &regrow);
    assert_eq!(sigma.cell_sizes_x, Some(Grid3dAxisPatch { length: Some(3), sizes: vec![size(2, 9.0)] }));
    let past_the_end = Grid3dDiff { cell_sizes_x: Some(Grid3dAxisPatch { length: None, sizes: vec![size(7, 1.0)] }), ..Default::default() };
    assert!(protocol::apply_diff(&past_the_end, &base).is_err());
}
