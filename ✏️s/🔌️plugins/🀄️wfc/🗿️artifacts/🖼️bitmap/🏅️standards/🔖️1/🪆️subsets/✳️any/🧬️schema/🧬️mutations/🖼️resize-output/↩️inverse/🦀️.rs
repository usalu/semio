//! ↩️ Inverse for `ResizeOutput` — restore the prior output spec, then re-pin every pixel the
//! shrink cascaded away, in the canonical row-major pin order so both index and value come back.

use crate::mutations::{pin_pixel, resize_output, BitmapMutation};
use crate::schema::snapshot::BitmapSnapshot;

pub fn inverse(payload: &super::ResizeOutput, base: &BitmapSnapshot) -> Vec<BitmapMutation> {
    if base.output.width == payload.width && base.output.height == payload.height && base.output.periodic == payload.periodic {
        return Vec::new();
    }
    let mut steps = vec![resize_output(base.output.width, base.output.height, base.output.periodic)];
    steps.extend(base.pinned.iter().filter(|pin| pin.x >= payload.width || pin.y >= payload.height).map(|pin| pin_pixel(pin.x, pin.y, pin.color)));
    steps
}
