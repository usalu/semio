//! ↩️ Inverse for `PinPixel` — re-pin the prior colour when the cell was already pinned, else
//! unpin it outright.

use crate::standards::v1::subsets::any::schema::mutations::pin_pixel::pin_pixel;
use crate::standards::v1::subsets::any::schema::mutations::unpin_pixel::unpin_pixel;
use crate::schema::snapshot::{pin_index, BitmapSnapshot};

pub fn inverse<Op>(payload: &super::PinPixel, base: &BitmapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::pin_pixel::PinPixel> + From<crate::standards::v1::subsets::any::schema::mutations::unpin_pixel::UnpinPixel>,
{
    Ok((|| {
    match pin_index(base, payload.x, payload.y) {
        Some(at) if base.pinned[at].color == payload.color => Vec::new(),
        Some(at) => vec![pin_pixel(payload.x, payload.y, base.pinned[at].color)],
        None => vec![unpin_pixel(payload.x, payload.y)],
    }

    })())
}
