//! ↩️ Inverse for `ResizeOutput` — restore the prior output spec, then re-pin every pixel the
//! shrink cascaded away, in the canonical row-major pin order so both index and value come back.

use crate::standards::v1::subsets::any::schema::mutations::pin_pixel::pin_pixel;
use crate::standards::v1::subsets::any::schema::mutations::resize_output::resize_output;
use crate::schema::snapshot::BitmapSnapshot;

pub fn inverse<Op>(payload: &super::ResizeOutput, base: &BitmapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::pin_pixel::PinPixel> + From<crate::standards::v1::subsets::any::schema::mutations::resize_output::ResizeOutput>,
{
    Ok((|| {
    if base.output.width == payload.width && base.output.height == payload.height && base.output.periodic == payload.periodic {
        return Vec::new();
    }
    let mut steps = vec![resize_output(base.output.width, base.output.height, base.output.periodic)];
    steps.extend(base.pinned.iter().filter(|pin| pin.x >= payload.width || pin.y >= payload.height).map(|pin| pin_pixel(pin.x, pin.y, pin.color)));
    steps.reverse();
    steps

    })())
}
