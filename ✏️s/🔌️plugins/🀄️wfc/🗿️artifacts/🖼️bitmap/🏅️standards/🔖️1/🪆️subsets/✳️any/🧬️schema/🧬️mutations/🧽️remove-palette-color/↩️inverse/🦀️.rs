//! ↩️ Inverse for `RemovePaletteColor` — re-insert the removed colour AT its own index, which is
//! what restores the renumbering as well as the entry.

use crate::mutations::{add_palette_color, BitmapMutation};
use crate::schema::snapshot::BitmapSnapshot;

pub fn inverse(payload: &super::RemovePaletteColor, base: &BitmapSnapshot) -> Vec<BitmapMutation> {
    let Some(color) = base.input.palette.get(payload.index) else { return Vec::new() };
    vec![add_palette_color(payload.index, *color)]
}
