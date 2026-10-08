//! 🔺️ Sparse diff builder for `SetInputPixels` — one real region entry on the `inputRegions` lane,
//! never a whole-buffer capture.

use crate::diff::{BitmapDiff, BitmapInputPatch, BitmapInputWrite, BitmapPixelRegion};
use crate::schema::snapshot::{read_region, BitmapSnapshot};

pub fn diff(payload: &super::SetInputPixels, base: &BitmapSnapshot) -> protocol::MutationOutcome<BitmapDiff> {
    let region = &payload.pixels;
    if payload.width == 0 || payload.height == 0 || region.len() != (payload.width as usize) * (payload.height as usize) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("The region payload holds {} bytes, not {}×{}.", region.len(), payload.width, payload.height), ["pixels".to_string()]);
    }
    if payload.x.saturating_add(payload.width) > base.input.width || payload.y.saturating_add(payload.height) > base.input.height {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("The region falls outside the {}×{} input bitmap.", base.input.width, base.input.height), ["pixels".to_string()]);
    }
    if let Some(index) = region.iter().find(|index| usize::from(**index) >= base.input.palette.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Palette index {index} is not in this document's palette."), ["pixels".to_string()]);
    }
    let Some(buffer) = base.input.indices() else {
        return protocol::MutationOutcome::fatal("mutation.apply.invalid-base", "The base input pixel buffer does not decode.".to_string(), ["input".to_string()]);
    };
    if read_region(&buffer, base.input.width, base.input.height, payload.x, payload.y, payload.width, payload.height).as_deref() == Some(region.as_slice()) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The region already holds these pixels.".to_string());
    }
    let entry = BitmapPixelRegion { x: payload.x, y: payload.y, width: payload.width, height: payload.height, pixels: payload.pixels.clone() };
    protocol::MutationOutcome::new(BitmapDiff { input: BitmapInputPatch { writes: vec![BitmapInputWrite::Region { region: entry }], ..Default::default() }, ..Default::default() })
}
