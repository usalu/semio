//! ↩️ Inverse for `UnpinPixel` — re-pin the exact colour the base carried; `pin-pixel` re-inserts
//! at the canonical row-major position, which is the position the removal vacated.

use crate::standards::v1::subsets::any::schema::mutations::pin_pixel::pin_pixel;
use crate::schema::snapshot::{pin_index, BitmapSnapshot};

pub fn inverse<Op>(payload: &super::UnpinPixel, base: &BitmapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::pin_pixel::PinPixel>,
{
    Ok((|| {
    let Some(at) = pin_index(base, payload.x, payload.y) else { return Vec::new() };
    vec![pin_pixel(payload.x, payload.y, base.pinned[at].color)]

    })())
}
