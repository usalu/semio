//! 🔺️ Sparse diff construction for `resize-tile-crop`.
use super::ResizeTileCrop;
use crate::diff::{PresentationDiff, PresentationSourcePatch, PresentationTilePatch, PresentationTilesDelta};
use crate::PresentationSnapshot;

//#region 🔹Diff
/// 🔺️ Reads the working-scene `(source, tiles)` off `base.presentation`, applies the crop-only
/// patch to the addressed tile, and mints a new content-addressed `presentation` handle for the
/// result — real handcrafted construction from `(payload, base)`, never apply-then-capture.
pub fn diff(payload: &ResizeTileCrop, base: &PresentationSnapshot) -> protocol::MutationOutcome<PresentationDiff> {
    let Some(existing) = base.tiles.iter().find(|tile| tile.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tile \"{}\" does not exist.", payload.id), ["tiles".to_string(), payload.id.clone()]);
    };
    let crop = &payload.new_crop;
    if !crop.x.is_finite() || !crop.y.is_finite() || !crop.width.is_finite() || !crop.height.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tile \"{}\" crop must be finite, got ({}, {}, {}, {}).", payload.id, crop.x, crop.y, crop.width, crop.height), ["tiles".to_string(), payload.id.clone()]);
    }
    if crop.width <= 0.0 || crop.height <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Tile \"{}\" crop width/height must be positive, got ({}, {}).", payload.id, crop.width, crop.height), ["tiles".to_string(), payload.id.clone()]);
    }
    if existing.crop == payload.new_crop {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Tile \"{}\" crop is already unchanged.", payload.id));
    }
    protocol::MutationOutcome::new(PresentationDiff { tiles: Some(PresentationTilesDelta::modification(payload.id.clone(), PresentationTilePatch { crop: Some(payload.new_crop.clone()), ..Default::default() })), ..Default::default() })
}
//#endregion 🔹Diff
