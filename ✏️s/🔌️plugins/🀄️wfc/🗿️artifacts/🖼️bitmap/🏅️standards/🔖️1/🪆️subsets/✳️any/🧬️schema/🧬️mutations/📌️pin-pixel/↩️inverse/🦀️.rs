//! ↩️ Inverse for `PinPixel` — re-pin the prior colour when the cell was already pinned, else
//! unpin it outright.

use crate::mutations::{pin_pixel, unpin_pixel, BitmapMutation};
use crate::schema::snapshot::{pin_index, BitmapSnapshot};

pub fn inverse(payload: &super::PinPixel, base: &BitmapSnapshot) -> Vec<BitmapMutation> {
    match pin_index(base, payload.x, payload.y) {
        Some(at) if base.pinned[at].color == payload.color => Vec::new(),
        Some(at) => vec![pin_pixel(payload.x, payload.y, base.pinned[at].color)],
        None => vec![unpin_pixel(payload.x, payload.y)],
    }
}
