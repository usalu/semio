//! ↩️ Inverse for `SetInputPixels` — the SAME rectangle carrying the bytes it covered before, read
//! out of the base. Bounded by the rectangle, never by the whole bitmap.

use crate::standards::v1::subsets::any::schema::mutations::set_input_pixels::set_input_pixels;
use crate::schema::snapshot::{read_region, BitmapSnapshot};

pub fn inverse<Op>(payload: &super::SetInputPixels, base: &BitmapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_input_pixels::SetInputPixels>,
{
    Ok((|| {
    let Some(buffer) = base.input.indices() else { return Vec::new() };
    let Some(prior) = read_region(&buffer, base.input.width, base.input.height, payload.x, payload.y, payload.width, payload.height) else { return Vec::new() };
    vec![set_input_pixels(payload.x, payload.y, payload.width, payload.height, prior.to_vec())]

    })())
}
