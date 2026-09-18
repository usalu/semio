//! 🧪️ Diff facet — lane application order, the pin lane's total validation, and `absorb`'s
//! composition law.

use super::*;
use crate::schema::snapshot::pin_key;
use crate::schema::snapshot::{decode_base64, encode_base64, BitmapColor, BitmapInput};
use protocol::MutationDiff;

fn scene() -> BitmapSnapshot {
    BitmapSnapshot {
        input: BitmapInput { width: 3, height: 2, palette: vec![BitmapColor::opaque(0, 0, 0), BitmapColor::opaque(255, 255, 255)], pixels: encode_base64(&[0, 1, 0, 1, 0, 1]) },
        output: BitmapOutputSpec { width: 4, height: 4, periodic: false },
        pinned: vec![BitmapPinnedPixel { x: 1, y: 1, color: 1 }],
        ..BitmapSnapshot::default()
    }
}

#[test]
fn an_empty_diff_is_the_identity() {
    let base = scene();
    assert_eq!(BitmapDiff::default().apply(&base).expect("the empty diff applies"), base);
}

#[test]
fn a_resize_pads_before_a_region_write_lands() {
    let diff = BitmapDiff {
        input_width: Some(4),
        input_height: Some(3),
        input_regions: vec![BitmapPixelRegion { x: 3, y: 2, width: 1, height: 1, pixels: encode_base64(&[1]) }],
        ..Default::default()
    };
    let next = diff.apply(&scene()).expect("resize then write");
    assert_eq!((next.input.width, next.input.height), (4, 3));
    let indices = next.input.indices().expect("the resized buffer decodes");
    assert_eq!(indices.len(), 12);
    assert_eq!(indices[11], 1, "the region landed inside the grown extent");
    assert_eq!(indices[0], 0, "the overlap is preserved");
}

#[test]
fn a_region_outside_the_extent_is_a_refusal_not_a_clip() {
    let diff = BitmapDiff { input_regions: vec![BitmapPixelRegion { x: 2, y: 0, width: 2, height: 1, pixels: encode_base64(&[1, 1]) }], ..Default::default() };
    assert!(diff.apply(&scene()).is_err());
}

#[test]
fn an_empty_palette_is_refused() {
    let diff = BitmapDiff { palette: Some(Vec::new()), ..Default::default() };
    assert!(diff.apply(&scene()).is_err());
}

#[test]
fn the_pin_lane_refuses_a_removal_of_something_that_is_not_there() {
    let diff = BitmapDiff { pinned_removed: vec![pin_key(3, 3)], ..Default::default() };
    assert!(diff.apply(&scene()).is_err());
}

#[test]
fn the_pin_lane_refuses_a_replacement_at_the_wrong_index() {
    let diff = BitmapDiff { pinned_upserted: vec![(1, BitmapPinnedPixel { x: 1, y: 1, color: 0 })], ..Default::default() };
    assert!(diff.apply(&scene()).is_err(), "an existing pin may only be replaced at its own index");
}

#[test]
fn the_pin_lane_refuses_removing_and_upserting_the_same_cell() {
    let key = pin_key(1, 1);
    let diff = BitmapDiff { pinned_removed: vec![key], pinned_upserted: vec![(0, BitmapPinnedPixel { x: 1, y: 1, color: 0 })], ..Default::default() };
    assert!(diff.apply(&scene()).is_err());
}

#[test]
fn absorb_lets_a_later_whole_buffer_write_supersede_earlier_regions() {
    let mut first = BitmapDiff { input_regions: vec![BitmapPixelRegion { x: 0, y: 0, width: 1, height: 1, pixels: encode_base64(&[1]) }], ..Default::default() };
    let second = BitmapDiff { input_pixels: Some(encode_base64(&[1, 1, 1, 1, 1, 1])), ..Default::default() };
    first.absorb(second);
    assert!(first.input_regions.is_empty(), "a whole-buffer rewrite drops the regions it would have overwritten");
    let next = first.apply(&scene()).expect("the merged diff applies");
    assert_eq!(decode_base64(&next.input.pixels).expect("pixels decode"), vec![1, 1, 1, 1, 1, 1]);
}

#[test]
fn absorb_concatenates_ordinary_region_writes() {
    let mut first = BitmapDiff { input_regions: vec![BitmapPixelRegion { x: 0, y: 0, width: 1, height: 1, pixels: encode_base64(&[1]) }], ..Default::default() };
    first.absorb(BitmapDiff { input_regions: vec![BitmapPixelRegion { x: 2, y: 1, width: 1, height: 1, pixels: encode_base64(&[0]) }], ..Default::default() });
    assert_eq!(first.input_regions.len(), 2);
    let next = first.apply(&scene()).expect("the merged diff applies");
    let indices = next.input.indices().expect("pixels decode");
    assert_eq!((indices[0], indices[5]), (1, 0));
}

#[test]
fn absorb_lets_a_later_pin_removal_win_over_an_earlier_upsert() {
    let key = pin_key(2, 2);
    let mut first = BitmapDiff { pinned_upserted: vec![(1, BitmapPinnedPixel { x: 2, y: 2, color: 0 })], ..Default::default() };
    first.absorb(BitmapDiff { pinned_removed: vec![key.clone()], ..Default::default() });
    assert!(first.pinned_upserted.is_empty());
    assert_eq!(first.pinned_removed, vec![key]);
}
