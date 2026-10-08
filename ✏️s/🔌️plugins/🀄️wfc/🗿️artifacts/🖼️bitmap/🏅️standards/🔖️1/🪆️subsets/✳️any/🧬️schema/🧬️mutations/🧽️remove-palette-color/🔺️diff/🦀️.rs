//! 🔺️ Sparse diff builder for `RemovePaletteColor` — the shortened palette plus the renumbered
//! pixel buffer and pin rows.

use crate::diff::{BitmapDiff, BitmapInputOp, BitmapPaletteOp, BitmapPinnedPatch, BitmapPixelCell, BitmapRowPatch, BitmapRows};
use crate::schema::snapshot::{pin_key, used_palette_indices, BitmapSnapshot};

pub fn diff(payload: &super::RemovePaletteColor, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
    if payload.index >= base.input.palette.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("The palette has no colour at index {}.", payload.index), [payload.index.to_string()]);
    }
    if base.input.palette.len() == 1 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A bitmap keeps at least one palette colour.".to_string(), [payload.index.to_string()]);
    }
    if used_palette_indices(base).contains(&(payload.index as u32)) {
        return protocol::MutationOutcome::error("mutation.target-referenced", format!("Palette colour {} is still painted or pinned.", payload.index), [payload.index.to_string()]);
    }
    let Some(buffer) = base.input.indices() else {
        return protocol::MutationOutcome::fatal("mutation.apply.invalid-base", "The base input pixel buffer does not decode.".to_string(), ["input".to_string()]);
    };
    let boundary = payload.index as u32;
    let width = base.input.width as usize;
    let cells: Vec<BitmapPixelCell> = buffer.iter().enumerate().filter(|(_, value)| u32::from(**value) > boundary).map(|(at, value)| BitmapPixelCell { x: (at % width) as u32, y: (at / width) as u32, value: u32::from(*value) - 1 }).collect();
    let patched: Vec<BitmapRowPatch<BitmapPinnedPatch>> = base.pinned.iter().filter(|pin| pin.color > boundary).map(|pin| BitmapRowPatch { id: pin_key(pin.x, pin.y), patch: BitmapPinnedPatch { color: Some(pin.color - 1) } }).collect();
    let input_ops = if cells.is_empty() { Vec::new() } else { vec![BitmapInputOp::Cells { cells }] };
    protocol::MutationOutcome::new(BitmapDiff { input_ops, palette_ops: vec![BitmapPaletteOp::Remove { index: boundary }], pinned: BitmapRows { patched, ..Default::default() }, ..Default::default() })
}
