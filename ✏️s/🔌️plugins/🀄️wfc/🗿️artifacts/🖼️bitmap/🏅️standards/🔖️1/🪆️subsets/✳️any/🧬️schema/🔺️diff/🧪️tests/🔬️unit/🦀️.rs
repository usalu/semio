//! 🧪️ `BitmapDiff` — the input patch, the positional palette delta, the pin rows, `absorb` coalescing and `inverse`.

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

fn region(x: u32, y: u32, width: u32, height: u32, pixels: Vec<u8>) -> BitmapInputWrite {
    BitmapInputWrite::Region { region: BitmapPixelRegion { x, y, width, height, pixels } }
}

fn resized(width: u32, height: u32, writes: Vec<BitmapInputWrite>) -> BitmapDiff {
    BitmapDiff { input: BitmapInputPatch { size: Some(BitmapSize { width, height }), writes }, ..Default::default() }
}

fn writing(writes: Vec<BitmapInputWrite>) -> BitmapDiff {
    BitmapDiff { input: BitmapInputPatch { size: None, writes }, ..Default::default() }
}

fn palette(delta: BitmapPaletteDelta) -> BitmapDiff {
    BitmapDiff { palette: delta, ..Default::default() }
}

/// 🕳️ The identity delta leaves the document untouched.
#[test]
fn an_empty_diff_is_the_identity() {
    let base = scene();
    assert_eq!(protocol::apply_diff(&BitmapDiff::default(), &base).expect("the empty diff applies"), base);
    assert!(DiffAlgebra::<BitmapSnapshot>::is_empty(&BitmapDiff::default()));
}

/// 📐️ The resize runs first: it pads before a region write lands in the grown extent.
#[test]
fn a_resize_pads_before_a_region_write_lands() {
    let diff = resized(4, 3, vec![region(3, 2, 1, 1, vec![1])]);
    let next = protocol::apply_diff(&diff, &scene()).expect("resize then write");
    assert_eq!((next.input.width, next.input.height), (4, 3));
    assert_eq!(next.input.pixels.len(), 12);
    assert_eq!((next.input.pixels[11], next.input.pixels[0]), (1, 0));
}

/// 🚫️ A region outside the extent is a refusal, not a clip; so is an emptied palette or an out-of-range palette edit.
#[test]
fn invalid_edits_are_refused() {
    assert!(protocol::apply_diff(&writing(vec![region(2, 0, 2, 1, vec![1, 1])]), &scene()).is_err());
    let emptied = palette(BitmapPaletteDelta { removed: vec![0, 1], ..Default::default() });
    assert!(protocol::apply_diff(&emptied, &scene()).is_err());
    let past_the_end = palette(BitmapPaletteDelta::recoloring(5, BitmapColor::opaque(1, 2, 3)));
    assert!(protocol::apply_diff(&past_the_end, &scene()).is_err());
}

/// 🔀 A palette entry inserted and removed again cancels; recolours of one entry coalesce; a removed entry drops its recolour.
#[test]
fn palette_deltas_coalesce_by_index_arithmetic() {
    let base = scene();
    let insert = palette(BitmapPaletteDelta::insertion(1, BitmapColor::opaque(9, 9, 9)));
    let remove = palette(BitmapPaletteDelta::removal(1));
    assert!(assert_absorb_law(&base, &insert, &remove).palette.is_empty());
    let recolor = |color| palette(BitmapPaletteDelta::recoloring(0, color));
    let sigma = assert_absorb_law(&base, &recolor(BitmapColor::opaque(1, 1, 1)), &recolor(BitmapColor::opaque(2, 2, 2)));
    assert_eq!(sigma.palette.recolored, vec![BitmapPaletteRecolor { index: 0, color: BitmapColor::opaque(2, 2, 2) }]);
    let sigma = assert_absorb_law(&base, &recolor(BitmapColor::opaque(1, 1, 1)), &palette(BitmapPaletteDelta::removal(0)));
    assert_eq!((sigma.palette.removed, sigma.palette.recolored.len()), (vec![0], 0));
    let sigma = assert_absorb_law(&base, &insert, &palette(BitmapPaletteDelta::recoloring(1, BitmapColor::opaque(7, 7, 7))));
    assert_eq!(sigma.palette.inserted, vec![BitmapPaletteInsertion { index: 1, color: BitmapColor::opaque(7, 7, 7) }]);
}

/// 🔀 Writes of adjacent diffs layer in order, the later value winning per pixel.
#[test]
fn writes_layer_in_order() {
    let base = scene();
    let cells = |rows: &[(u32, u32, u32)]| writing(vec![BitmapInputWrite::Cells { cells: rows.iter().map(|(x, y, value)| BitmapPixelCell { x: *x, y: *y, value: *value }).collect() }]);
    let sigma = assert_absorb_law(&base, &cells(&[(0, 0, 1), (2, 1, 0)]), &cells(&[(0, 0, 0), (1, 0, 1)]));
    assert_eq!(sigma.input.writes.len(), 2);
}

/// 📐️ A shrink followed by a regrow keeps no stale pixel: the regrown area is zero-filled, earlier writes are clipped to the final size.
#[test]
fn resizes_compose_into_one_resize_with_a_zero_fill() {
    let base = scene();
    let shrink = resized(2, 1, vec![region(1, 0, 1, 1, vec![1])]);
    let regrow = resized(3, 2, vec![region(2, 1, 1, 1, vec![1])]);
    let sigma = assert_absorb_law(&base, &shrink, &regrow);
    assert_eq!(sigma.input.size, Some(BitmapSize { width: 3, height: 2 }));
    let shrink_more = resized(1, 1, Vec::new());
    assert_absorb_law(&base, &shrink, &shrink_more);
}

/// 🔀 Pin rows follow the positional laws: create∘delete cancels, patch∘delete leaves the deletion at its base index.
#[test]
fn pin_rows_coalesce_per_key() {
    let base = scene();
    let rows = |delta: BitmapPinnedDelta| BitmapDiff { pinned: delta, ..Default::default() };
    let sigma = assert_absorb_law(&base, &rows(BitmapPinnedDelta::insertion(0, pin(0, 0, 1))), &rows(BitmapPinnedDelta::removal(&[pin(0, 0, 1), pin(1, 1, 1)], 0)));
    assert!(DiffAlgebra::<BitmapSnapshot>::is_empty(&sigma));
    let recolor = BitmapPinnedDelta { modified: vec![BitmapPinnedModification { id: pin_key(1, 1), patch: BitmapPinnedPatch { color: Some(0) } }], ..Default::default() };
    let sigma = assert_absorb_law(&base, &rows(recolor), &rows(BitmapPinnedDelta::removal(&[pin(1, 1, 0)], 0)));
    assert_eq!((sigma.pinned.removed, sigma.pinned.modified.len()), (vec![BitmapPinnedRemoval { id: pin_key(1, 1), index: 0 }], 0));
}

/// 🔁️ The inverse diff restores resized, painted and re-paletted documents exactly.
#[test]
fn inverse_restores_the_base() {
    let base = scene();
    let diff = BitmapDiff {
        input: BitmapInputPatch { size: Some(BitmapSize { width: 2, height: 1 }), writes: vec![region(0, 0, 1, 1, vec![1])] },
        palette: BitmapPaletteDelta { inserted: vec![BitmapPaletteInsertion { index: 1, color: BitmapColor::opaque(9, 9, 9) }], recolored: vec![BitmapPaletteRecolor { index: 0, color: BitmapColor::opaque(5, 5, 5) }], ..Default::default() },
        pinned: BitmapPinnedDelta::insertion(0, pin(0, 0, 0)),
        ..Default::default()
    };
    let after = protocol::apply_diff(&diff, &base).expect("diff applies");
    let inverse = DiffAlgebra::<BitmapSnapshot>::inverse(&diff, &base);
    assert_eq!(protocol::apply_diff(&inverse, &after).expect("inverse applies"), base);
}

/// 🎯️ Removing, inserting and recolouring a MIDDLE palette entry are each undone at the entry's original position.
#[test]
fn inverse_restores_middle_palette_entries() {
    let base = BitmapSnapshot {
        input: BitmapInput { width: 2, height: 1, palette: vec![BitmapColor::opaque(1, 0, 0), BitmapColor::opaque(2, 0, 0), BitmapColor::opaque(3, 0, 0)], pixels: vec![0, 2] },
        ..BitmapSnapshot::default()
    };
    for delta in [
        BitmapPaletteDelta::removal(1),
        BitmapPaletteDelta::insertion(1, BitmapColor::opaque(9, 9, 9)),
        BitmapPaletteDelta { removed: vec![0], inserted: vec![BitmapPaletteInsertion { index: 2, color: BitmapColor::opaque(8, 8, 8) }], recolored: vec![BitmapPaletteRecolor { index: 2, color: BitmapColor::opaque(7, 7, 7) }] },
    ] {
        let diff = palette(delta);
        let after = protocol::apply_diff(&diff, &base).expect("diff applies");
        let inverse = DiffAlgebra::<BitmapSnapshot>::inverse(&diff, &base);
        assert_eq!(protocol::apply_diff(&inverse, &after).expect("inverse applies"), base);
    }
}

/// 🚫️ The pin rows refuse removing something that is not there.
#[test]
fn the_pin_rows_refuse_a_missing_removal() {
    let diff = BitmapDiff { pinned: BitmapPinnedDelta { removed: vec![BitmapPinnedRemoval { id: pin_key(3, 3), index: 0 }], ..Default::default() }, ..Default::default() };
    assert!(protocol::apply_diff(&diff, &scene()).is_err());
}
