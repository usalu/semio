//! ↩️ Inverse for `ChangePaletteColor` — write the prior colour back at the same index.

use crate::mutations::{change_palette_color, BitmapMutation};
use crate::schema::snapshot::BitmapSnapshot;

pub fn inverse(payload: &super::ChangePaletteColor, base: &BitmapSnapshot) -> Vec<BitmapMutation> {
    let Some(existing) = base.input.palette.get(payload.index) else { return Vec::new() };
    vec![change_palette_color(payload.index, *existing)]
}
