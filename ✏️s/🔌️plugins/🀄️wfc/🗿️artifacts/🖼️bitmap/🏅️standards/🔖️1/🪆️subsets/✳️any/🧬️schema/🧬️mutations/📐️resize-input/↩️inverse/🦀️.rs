//! ↩️ Inverse for `ResizeInput` — resize back AND write the whole prior buffer, because a shrink
//! discards the pixels outside the new extent and a resize alone cannot bring them back.

use crate::mutations::{resize_input, set_input_pixels, BitmapMutation};
use crate::schema::snapshot::BitmapSnapshot;

pub fn inverse(payload: &super::ResizeInput, base: &BitmapSnapshot) -> Vec<BitmapMutation> {
    if base.input.width == payload.width && base.input.height == payload.height {
        return Vec::new();
    }
    vec![resize_input(base.input.width, base.input.height), set_input_pixels(0, 0, base.input.width, base.input.height, base.input.pixels.clone())]
}
