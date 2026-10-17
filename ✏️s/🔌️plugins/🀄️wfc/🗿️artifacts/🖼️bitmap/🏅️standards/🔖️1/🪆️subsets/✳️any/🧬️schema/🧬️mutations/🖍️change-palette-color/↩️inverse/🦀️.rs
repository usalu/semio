//! ↩️ Inverse for `ChangePaletteColor` — write the prior colour back at the same index.

use crate::standards::v1::subsets::any::schema::mutations::change_palette_color::change_palette_color;
use crate::schema::snapshot::BitmapSnapshot;

pub fn inverse<Op>(payload: &super::ChangePaletteColor, base: &BitmapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_palette_color::ChangePaletteColor>,
{
    Ok((|| {
    let Some(existing) = base.input.palette.get(payload.index) else { return Vec::new() };
    vec![change_palette_color(payload.index, *existing)]

    })())
}
