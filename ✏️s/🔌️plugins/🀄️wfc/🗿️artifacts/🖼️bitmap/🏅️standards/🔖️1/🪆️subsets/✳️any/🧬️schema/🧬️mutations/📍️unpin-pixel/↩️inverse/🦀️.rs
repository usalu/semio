//! ↩️ Inverse for `UnpinPixel` — re-pin the exact colour the base carried; `pin-pixel` re-inserts
//! at the canonical row-major position, which is the position the removal vacated.

use crate::mutations::{pin_pixel, BitmapMutation};
use crate::schema::snapshot::{pin_index, BitmapSnapshot};

pub fn inverse(payload: &super::UnpinPixel, base: &BitmapSnapshot) -> Vec<BitmapMutation> {
    let Some(at) = pin_index(base, payload.x, payload.y) else { return Vec::new() };
    vec![pin_pixel(payload.x, payload.y, base.pinned[at].color)]
}
