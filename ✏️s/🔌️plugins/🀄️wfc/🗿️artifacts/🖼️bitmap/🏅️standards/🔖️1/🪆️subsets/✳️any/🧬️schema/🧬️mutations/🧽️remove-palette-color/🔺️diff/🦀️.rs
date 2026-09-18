//! 🔺️ Sparse diff builder for `RemovePaletteColor` — the shortened palette plus the renumbered
//! pixel buffer and pin rows.

use crate::diff::BitmapDiff;
use crate::schema::snapshot::{encode_base64, used_palette_indices, BitmapSnapshot};

pub fn diff(payload: &super::RemovePaletteColor, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
    if payload.index >= base.input.palette.len() {
        return protocol::MutationOutcome::fatal("mutation.missing-target", format!("The palette has no colour at index {}.", payload.index), [payload.index.to_string()]);
    }
    if base.input.palette.len() == 1 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "A bitmap keeps at least one palette colour.".to_string(), [payload.index.to_string()]);
    }
    if used_palette_indices(base).contains(&(payload.index as u32)) {
        return protocol::MutationOutcome::fatal("mutation.colour-in-use", format!("Palette colour {} is still painted or pinned.", payload.index), [payload.index.to_string()]);
    }
    let Some(buffer) = base.input.indices() else {
        return protocol::MutationOutcome::fatal("mutation.malformed-payload", "The base input pixel buffer does not decode.".to_string(), ["input".to_string()]);
    };
    let mut palette = base.input.palette.clone();
    palette.remove(payload.index);
    let boundary = payload.index as u8;
    let renumbered: Vec<u8> = buffer.iter().map(|index| if *index > boundary { index - 1 } else { *index }).collect();
    let pinned_upserted = base
        .pinned
        .iter()
        .enumerate()
        .filter(|(_, pin)| pin.color > payload.index as u32)
        .map(|(at, pin)| {
            let mut moved = *pin;
            moved.color -= 1;
            (at, moved)
        })
        .collect::<Vec<_>>();
    let input_pixels = (renumbered != buffer).then(|| encode_base64(&renumbered));
    protocol::MutationOutcome::new(BitmapDiff { input_pixels, palette: Some(palette), pinned_upserted, ..Default::default() })
}
