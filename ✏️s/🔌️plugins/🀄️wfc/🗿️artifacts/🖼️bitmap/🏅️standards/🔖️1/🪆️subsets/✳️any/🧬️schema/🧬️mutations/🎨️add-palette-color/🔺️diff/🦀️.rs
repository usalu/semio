//! 🔺️ Sparse diff builder for `AddPaletteColor` — the new palette, plus the renumbered pixel buffer
//! and pin rows ONLY when the insert actually renumbers something (an append never does).

use crate::diff::BitmapDiff;
use crate::schema::snapshot::{encode_base64, BitmapSnapshot, BITMAP_MAX_PALETTE};

pub fn diff(payload: &super::AddPaletteColor, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
    if payload.index > base.input.palette.len() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Palette index {} is past the end of a {}-entry palette.", payload.index, base.input.palette.len()), ["palette".to_string()]);
    }
    if base.input.palette.len() >= BITMAP_MAX_PALETTE {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A palette may hold at most {BITMAP_MAX_PALETTE} colours."), ["palette".to_string()]);
    }
    let Some(buffer) = base.input.indices() else {
        return protocol::MutationOutcome::fatal("mutation.malformed-payload", "The base input pixel buffer does not decode.".to_string(), ["input".to_string()]);
    };
    let mut palette = base.input.palette.clone();
    palette.insert(payload.index, payload.color);
    let boundary = payload.index as u8;
    let renumbered: Vec<u8> = buffer.iter().map(|index| if *index >= boundary { index + 1 } else { *index }).collect();
    let pinned_upserted = base
        .pinned
        .iter()
        .enumerate()
        .filter(|(_, pin)| pin.color >= payload.index as u32)
        .map(|(at, pin)| {
            let mut moved = *pin;
            moved.color += 1;
            (at, moved)
        })
        .collect::<Vec<_>>();
    let input_pixels = (renumbered != buffer).then(|| encode_base64(&renumbered));
    protocol::MutationOutcome::new(BitmapDiff { input_pixels, palette: Some(palette), pinned_upserted, ..Default::default() })
}
