//! 🧪️ `BitmapDiff` — ordered input and palette ops, the pin rows, `absorb` coalescing, `inverse` and `between`.

use super::*;
use crate::schema::snapshot::{BitmapColor, BitmapInput};
use protocol::DiffAlgebra;

fn scene() -> BitmapSnapshot {
    BitmapSnapshot {
        input: BitmapInput { width: 3, height: 2, palette: vec![BitmapColor::opaque(0, 0, 0), BitmapColor::opaque(255, 255, 255)], pixels: vec![0, 1, 0, 1, 0, 1] },
        output: BitmapOutputSpec { width: 4, height: 4, periodic: false },
        pinned: vec![BitmapPinnedPixel { x: 1, y: 1, color: 1 }],
        ..BitmapSnapshot::default()
    }
}

fn pin(x: u32, y: u32, color: u32) -> BitmapPinnedPixel {
    BitmapPinnedPixel { x, y, color }
}

fn absorbed(first: BitmapDiff, second: BitmapDiff) -> BitmapDiff {
    let mut sigma = first;
    protocol::MutationDiff::<BitmapSnapshot>::absorb(&mut sigma, second);
    sigma
}

fn assert_absorb_law(base: &BitmapSnapshot, first: &BitmapDiff, second: &BitmapDiff) -> BitmapDiff {
    let sigma = absorbed(first.clone(), second.clone());
    let mid = protocol::apply_diff(first, base).expect("first applies");
    let sequential = protocol::apply_diff(second, &mid).expect("second applies");
    assert_eq!(protocol::apply_diff(&sigma, base).expect("absorbed applies"), sequential, "absorb must equal sequential application");
    sigma
}

fn region(x: u32, y: u32, width: u32, height: u32, pixels: Vec<u8>) -> BitmapInputOp {
    BitmapInputOp::Region { region: BitmapPixelRegion { x, y, width, height, pixels } }
}

/// 🕳️ The identity delta leaves the document untouched.
#[test]
fn an_empty_diff_is_the_identity() {
    let base = scene();
    assert_eq!(protocol::apply_diff(&BitmapDiff::default(), &base).expect("the empty diff applies"), base);
    assert!(DiffAlgebra::<BitmapSnapshot>::is_empty(&BitmapDiff::default()));
}

/// 📐️ Ops run in order: a resize pads before a later region write lands in the grown extent.
#[test]
fn a_resize_pads_before_a_region_write_lands() {
    let diff = BitmapDiff { input_ops: vec![BitmapInputOp::Resize { width: 4, height: 3 }, region(3, 2, 1, 1, vec![1])], ..Default::default() };
    let next = protocol::apply_diff(&diff, &scene()).expect("resize then write");
    assert_eq!((next.input.width, next.input.height), (4, 3));
    assert_eq!(next.input.pixels.len(), 12);
    assert_eq!((next.input.pixels[11], next.input.pixels[0]), (1, 0));
}

/// 🚫️ A region outside the extent is a refusal, not a clip; so is an emptied palette or an out-of-range palette edit.
#[test]
fn invalid_edits_are_refused() {
    let outside = BitmapDiff { input_ops: vec![region(2, 0, 2, 1, vec![1, 1])], ..Default::default() };
    assert!(protocol::apply_diff(&outside, &scene()).is_err());
    let emptied = BitmapDiff { palette_ops: vec![BitmapPaletteOp::Remove { index: 0 }, BitmapPaletteOp::Remove { index: 0 }], ..Default::default() };
    assert!(protocol::apply_diff(&emptied, &scene()).is_err());
    let past_the_end = BitmapDiff { palette_ops: vec![BitmapPaletteOp::Recolor { index: 5, color: BitmapColor::opaque(1, 2, 3) }], ..Default::default() };
    assert!(protocol::apply_diff(&past_the_end, &scene()).is_err());
}

/// 🔀 A palette entry inserted and removed again cancels; recolours of one entry coalesce.
#[test]
fn palette_ops_coalesce() {
    let base = scene();
    let insert = BitmapDiff { palette_ops: vec![BitmapPaletteOp::Insert { index: 1, color: BitmapColor::opaque(9, 9, 9) }], ..Default::default() };
    let remove = BitmapDiff { palette_ops: vec![BitmapPaletteOp::Remove { index: 1 }], ..Default::default() };
    assert!(assert_absorb_law(&base, &insert, &remove).palette_ops.is_empty());
    let recolor = |color| BitmapDiff { palette_ops: vec![BitmapPaletteOp::Recolor { index: 0, color }], ..Default::default() };
    let sigma = assert_absorb_law(&base, &recolor(BitmapColor::opaque(1, 1, 1)), &recolor(BitmapColor::opaque(2, 2, 2)));
    assert_eq!(sigma.palette_ops, vec![BitmapPaletteOp::Recolor { index: 0, color: BitmapColor::opaque(2, 2, 2) }]);
}

/// 🔀 Single-pixel writes of adjacent diffs merge cell by cell, the later value winning.
#[test]
fn cell_writes_merge_per_pixel() {
    let base = scene();
    let cells = |rows: &[(u32, u32, u32)]| BitmapDiff { input_ops: vec![BitmapInputOp::Cells { cells: rows.iter().map(|(x, y, value)| BitmapPixelCell { x: *x, y: *y, value: *value }).collect() }], ..Default::default() };
    let sigma = assert_absorb_law(&base, &cells(&[(0, 0, 1), (2, 1, 0)]), &cells(&[(0, 0, 0), (1, 0, 1)]));
    assert_eq!(sigma.input_ops.len(), 1);
}

/// 🔀 Pin rows follow the id-keyed laws: create∘delete cancels, patch∘delete leaves the deletion.
#[test]
fn pin_rows_coalesce_per_key() {
    let base = scene();
    let rows = |delta: BitmapPinnedDelta| BitmapDiff { pinned: delta, ..Default::default() };
    let sigma = assert_absorb_law(&base, &rows(BitmapRows { added: vec![pin(0, 0, 1)], ..Default::default() }), &rows(BitmapRows { removed: vec![pin_key(0, 0)], ..Default::default() }));
    assert!(DiffAlgebra::<BitmapSnapshot>::is_empty(&sigma));
    let recolor = BitmapRows { patched: vec![BitmapRowPatch { id: pin_key(1, 1), patch: BitmapPinnedPatch { color: Some(0) } }], ..Default::default() };
    let sigma = assert_absorb_law(&base, &rows(recolor), &rows(BitmapRows { removed: vec![pin_key(1, 1)], ..Default::default() }));
    assert_eq!((sigma.pinned.removed, sigma.pinned.patched.len()), (vec![pin_key(1, 1)], 0));
}

/// 🔁️ The inverse diff restores resized, painted and re-paletted documents exactly.
#[test]
fn inverse_restores_the_base() {
    let base = scene();
    let diff = BitmapDiff {
        input_ops: vec![BitmapInputOp::Resize { width: 2, height: 1 }, region(0, 0, 1, 1, vec![1])],
        palette_ops: vec![BitmapPaletteOp::Insert { index: 1, color: BitmapColor::opaque(9, 9, 9) }, BitmapPaletteOp::Recolor { index: 0, color: BitmapColor::opaque(5, 5, 5) }],
        pinned: BitmapRows { added: vec![pin(0, 0, 0)], ..Default::default() },
        ..Default::default()
    };
    let after = protocol::apply_diff(&diff, &base).expect("diff applies");
    let inverse = DiffAlgebra::<BitmapSnapshot>::inverse(&diff, &base);
    assert_eq!(protocol::apply_diff(&inverse, &after).expect("inverse applies"), base);
}

/// 🧭️ `between` reaches the other document, and is empty between equal documents.
#[test]
fn between_reaches_the_other_document() {
    let base = scene();
    let mut other = scene();
    other.input = BitmapInput { width: 2, height: 2, palette: vec![BitmapColor::opaque(0, 0, 0), BitmapColor::opaque(7, 7, 7), BitmapColor::opaque(8, 8, 8)], pixels: vec![2, 1, 0, 1] };
    other.pinned = vec![pin(0, 1, 2)];
    let delta = <BitmapDiff as DiffAlgebra<BitmapSnapshot>>::between(&base, &other);
    assert_eq!(protocol::apply_diff(&delta, &base).expect("between applies"), other);
    assert!(DiffAlgebra::<BitmapSnapshot>::is_empty(&<BitmapDiff as DiffAlgebra<BitmapSnapshot>>::between(&base, &base)));
}

/// 🚫️ The pin rows refuse removing something that is not there.
#[test]
fn the_pin_rows_refuse_a_missing_removal() {
    let diff = BitmapDiff { pinned: BitmapRows { removed: vec![pin_key(3, 3)], ..Default::default() }, ..Default::default() };
    assert!(protocol::apply_diff(&diff, &scene()).is_err());
}
