//! 🔺️ Sparse diff builder for `ChangePaletteColor` — one palette lane write.

use crate::diff::BitmapDiff;
use crate::schema::snapshot::BitmapSnapshot;

pub fn diff(payload: &super::ChangePaletteColor, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
    let Some(existing) = base.input.palette.get(payload.index) else {
        return protocol::MutationOutcome::fatal("mutation.missing-target", format!("The palette has no colour at index {}.", payload.index), ["palette".to_string()]);
    };
    if *existing == payload.color {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Palette colour {} already holds that value.", payload.index));
    }
    let mut palette = base.input.palette.clone();
    palette[payload.index] = payload.color;
    protocol::MutationOutcome::new(BitmapDiff { palette: Some(palette), ..Default::default() })
}
