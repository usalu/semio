//! ↩️ Inverse for `AddPaletteColor` — remove the colour that was just inserted; `remove-palette-color`
//! renumbers back down by exactly the amount this one renumbered up.

use crate::mutations::{remove_palette_color, BitmapMutation};
use crate::schema::snapshot::BitmapSnapshot;

pub fn inverse(payload: &super::AddPaletteColor, base: &BitmapSnapshot) -> Vec<BitmapMutation> {
    if payload.index > base.input.palette.len() {
        return Vec::new();
    }
    vec![remove_palette_color(payload.index)]
}
