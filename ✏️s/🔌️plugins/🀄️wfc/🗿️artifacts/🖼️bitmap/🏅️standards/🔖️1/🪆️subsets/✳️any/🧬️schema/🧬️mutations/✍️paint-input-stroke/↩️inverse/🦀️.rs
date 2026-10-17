//! ↩️ Inverse for `PaintInputStroke` — one `set-input-pixels` over the stroke's bounding box carrying the
//! bytes it covered before, read out of the base. Bounded by the stroke, never by the whole bitmap.

use super::stroke_extent;
use crate::standards::v1::subsets::any::schema::mutations::set_input_pixels::set_input_pixels;
use crate::schema::snapshot::{read_region, BitmapSnapshot};

pub fn inverse<Op>(payload: &super::PaintInputStroke, base: &BitmapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_input_pixels::SetInputPixels>,
{
    Ok((|| {
    let Some(buffer) = base.input.indices() else { return Vec::new() };
    let Some(((x, y, width, height), _)) = stroke_extent(&payload.points, base.input.width, base.input.height) else { return Vec::new() };
    let Some(prior) = read_region(&buffer, base.input.width, base.input.height, x, y, width, height) else { return Vec::new() };
    vec![set_input_pixels(x, y, width, height, prior.to_vec())]

    })())
}
