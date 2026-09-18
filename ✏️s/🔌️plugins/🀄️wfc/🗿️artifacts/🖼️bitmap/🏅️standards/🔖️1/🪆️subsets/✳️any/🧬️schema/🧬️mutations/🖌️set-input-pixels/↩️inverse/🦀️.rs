//! ↩️ Inverse for `SetInputPixels` — the SAME rectangle carrying the bytes it covered before, read
//! out of the base. Bounded by the rectangle, never by the whole bitmap.

use crate::mutations::{set_input_pixels, BitmapMutation};
use crate::schema::snapshot::{encode_base64, read_region, BitmapSnapshot};

pub fn inverse(payload: &super::SetInputPixels, base: &BitmapSnapshot) -> Vec<BitmapMutation> {
    let Some(buffer) = base.input.indices() else { return Vec::new() };
    let Some(prior) = read_region(&buffer, base.input.width, base.input.height, payload.x, payload.y, payload.width, payload.height) else { return Vec::new() };
    vec![set_input_pixels(payload.x, payload.y, payload.width, payload.height, encode_base64(&prior))]
}
