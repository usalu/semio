//! ↩️ Inverse for `PaintInputStroke` — one `set-input-pixels` over the stroke's bounding box carrying the
//! bytes it covered before, read out of the base. Bounded by the stroke, never by the whole bitmap.

use super::stroke_extent;
use crate::mutations::{set_input_pixels, BitmapMutation};
use crate::schema::snapshot::{encode_base64, read_region, BitmapSnapshot};

pub fn inverse(payload: &super::PaintInputStroke, base: &BitmapSnapshot) -> Vec<BitmapMutation> {
    let Some(buffer) = base.input.indices() else { return Vec::new() };
    let Some(((x, y, width, height), _)) = stroke_extent(&payload.points, base.input.width, base.input.height) else { return Vec::new() };
    let Some(prior) = read_region(&buffer, base.input.width, base.input.height, x, y, width, height) else { return Vec::new() };
    vec![set_input_pixels(x, y, width, height, encode_base64(&prior))]
}
