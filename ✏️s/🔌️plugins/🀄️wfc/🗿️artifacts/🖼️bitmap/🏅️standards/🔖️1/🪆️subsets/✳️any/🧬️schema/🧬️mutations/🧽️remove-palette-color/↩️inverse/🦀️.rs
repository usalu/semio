//! ↩️ Inverse for `RemovePaletteColor` — re-insert the removed colour AT its own index, which is
//! what restores the renumbering as well as the entry.

use crate::standards::v1::subsets::any::schema::mutations::add_palette_color::add_palette_color;
use crate::schema::snapshot::BitmapSnapshot;

pub fn inverse<Op>(payload: &super::RemovePaletteColor, base: &BitmapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::add_palette_color::AddPaletteColor>,
{
    Ok((|| {
    let Some(color) = base.input.palette.get(payload.index) else { return Vec::new() };
    vec![add_palette_color(payload.index, *color)]

    })())
}
