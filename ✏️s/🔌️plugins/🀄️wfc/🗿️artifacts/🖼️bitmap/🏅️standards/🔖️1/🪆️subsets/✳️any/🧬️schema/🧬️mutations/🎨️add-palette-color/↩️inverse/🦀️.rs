//! ↩️ Inverse for `AddPaletteColor` — remove the colour that was just inserted; `remove-palette-color`
//! renumbers back down by exactly the amount this one renumbered up.

use crate::standards::v1::subsets::any::schema::mutations::remove_palette_color::remove_palette_color;
use crate::schema::snapshot::BitmapSnapshot;

pub fn inverse<Op>(payload: &super::AddPaletteColor, base: &BitmapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::remove_palette_color::RemovePaletteColor>,
{
    Ok((|| {
    if payload.index > base.input.palette.len() {
        return Vec::new();
    }
    vec![remove_palette_color(payload.index)]

    })())
}
