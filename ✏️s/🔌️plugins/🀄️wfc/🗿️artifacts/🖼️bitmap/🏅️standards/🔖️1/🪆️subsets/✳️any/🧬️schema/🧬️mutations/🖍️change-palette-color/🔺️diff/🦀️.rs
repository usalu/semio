//! 🔺️ Sparse diff builder for `ChangePaletteColor` — one palette lane write.

use crate::diff::{BitmapDiff, BitmapPaletteOp};
use crate::schema::snapshot::BitmapSnapshot;

pub fn diff(payload: &super::ChangePaletteColor, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
    let Some(existing) = base.input.palette.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("The palette has no colour at index {}.", payload.index), ["palette".to_string()]);
    };
    if *existing == payload.color {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Palette colour {} already holds that value.", payload.index));
    }
    protocol::MutationOutcome::new(BitmapDiff { palette_ops: vec![BitmapPaletteOp::Recolor { index: payload.index as u32, color: payload.color }], ..Default::default() })
}
