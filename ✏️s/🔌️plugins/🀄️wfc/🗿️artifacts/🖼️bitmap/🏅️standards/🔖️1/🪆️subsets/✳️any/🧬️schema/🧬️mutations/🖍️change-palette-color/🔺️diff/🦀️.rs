//! 🔺️ Sparse diff builder for `ChangePaletteColor` — one palette lane write.

use crate::diff::{BitmapDiff, BitmapPaletteDelta};
use crate::schema::snapshot::BitmapSnapshot;

pub fn diff(payload: &super::ChangePaletteColor, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
    if [payload.color.r,payload.color.g,payload.color.b,payload.color.a].into_iter().any(|channel|channel>255) {
        return protocol::MutationOutcome::fatal("mutation.invariant","Palette colour channels must be between 0 and 255.",["palette".to_string()]);
    }
    let Some(existing) = base.input.palette.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("The palette has no colour at index {}.", payload.index), ["palette".to_string()]);
    };
    if *existing == payload.color {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Palette colour {} already holds that value.", payload.index));
    }
    protocol::MutationOutcome::new(BitmapDiff { palette: BitmapPaletteDelta::recoloring(payload.index as u32, payload.color), ..Default::default() })
}
