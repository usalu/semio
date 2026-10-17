//! ↩️ Inverse for `ResizeInput` — resize back AND write the whole prior buffer, because a shrink
//! discards the pixels outside the new extent and a resize alone cannot bring them back.

use crate::standards::v1::subsets::any::schema::mutations::resize_input::resize_input;
use crate::standards::v1::subsets::any::schema::mutations::set_input_pixels::set_input_pixels;
use crate::schema::snapshot::{read_region, BitmapSnapshot};

pub fn inverse<Op>(payload: &super::ResizeInput, base: &BitmapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::resize_input::ResizeInput> + From<crate::standards::v1::subsets::any::schema::mutations::set_input_pixels::SetInputPixels>,
{
    Ok((|| {
    if base.input.width == payload.width && base.input.height == payload.height {
        return Vec::new();
    }
    let (width, height) = (base.input.width, base.input.height);
    let Some(buffer) = base.input.indices() else { return Vec::new() };
    let mut restore = vec![resize_input(width, height)];
    if payload.width < width {
        restore.extend(read_region(&buffer, width, height, payload.width, 0, width - payload.width, height).map(|pixels| set_input_pixels(payload.width, 0, width - payload.width, height, pixels)));
    }
    if payload.height < height {
        let surviving = payload.width.min(width);
        restore.extend(read_region(&buffer, width, height, 0, payload.height, surviving, height - payload.height).map(|pixels| set_input_pixels(0, payload.height, surviving, height - payload.height, pixels)));
    }
    restore.reverse();
    restore

    })())
}
