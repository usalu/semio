//! 🔺️ Sparse diff builder for `AddPaletteColor` — the new palette, plus the renumbered pixel buffer
//! and pin rows ONLY when the insert actually renumbers something (an append never does).

use crate::diff::{BitmapDiff, BitmapInputPatch, BitmapInputWrite, BitmapPaletteDelta, BitmapPinnedPatch, BitmapPixelCell, BitmapPinnedDelta, BitmapPinnedModification};
use crate::schema::snapshot::{pin_key, BitmapSnapshot, BITMAP_MAX_PALETTE};

pub fn diff(payload: &super::AddPaletteColor, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
    if payload.index > base.input.palette.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Palette index {} is past the end of a {}-entry palette.", payload.index, base.input.palette.len()), ["palette".to_string()]);
    }
    if base.input.palette.len() >= BITMAP_MAX_PALETTE {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A palette may hold at most {BITMAP_MAX_PALETTE} colours."), ["palette".to_string()]);
    }
    let Some(buffer) = base.input.indices() else {
        return protocol::MutationOutcome::fatal("mutation.apply.invalid-base", "The base input pixel buffer does not decode.".to_string(), ["input".to_string()]);
    };
    let boundary = payload.index as u32;
    let width = base.input.width as usize;
    let cells: Vec<BitmapPixelCell> = buffer.iter().enumerate().filter(|(_, value)| u32::from(**value) >= boundary).map(|(at, value)| BitmapPixelCell { x: (at % width) as u32, y: (at / width) as u32, value: u32::from(*value) + 1 }).collect();
    let patched: Vec<BitmapPinnedModification> = base.pinned.iter().filter(|pin| pin.color >= boundary).map(|pin| BitmapPinnedModification { id: pin_key(pin.x, pin.y), patch: BitmapPinnedPatch { color: Some(pin.color + 1) } }).collect();
    let writes = if cells.is_empty() { Vec::new() } else { vec![BitmapInputWrite::Cells { cells }] };
    protocol::MutationOutcome::new(BitmapDiff { input: BitmapInputPatch { writes, ..Default::default() }, palette: BitmapPaletteDelta::insertion(boundary, payload.color), pinned: BitmapPinnedDelta { modified: patched, ..Default::default() }, ..Default::default() })
}
