//! 🔺️ Sparse diff construction for `resize-tile-crop`.
use super::mutation::ResizeTileCrop;
use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Diff
/// 🔺️ Reads the working-scene `(source, tiles)` off `base.presentation`, applies the crop-only
/// patch to the addressed tile, and mints a new content-addressed `presentation` handle for the
/// result — real handcrafted construction from `(payload, base)`, never apply-then-capture.
pub async fn diff(payload: &ResizeTileCrop, base: &PresentSnapshot) -> protocol::MutationOutcome<PresentDiff> {
    let (source, mut tiles) = crate::artifacts::present::present_working_scene(base);
    let Some(existing) = tiles.iter().find(|tile| tile.id == payload.id) else {
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
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Tile \"{}\" crop is already unchanged.", payload.id));
    }
    if let Some(tile) = tiles.iter_mut().find(|tile| tile.id == payload.id) {
        tile.crop = payload.new_crop.clone();
    }
    protocol::MutationOutcome::new(crate::artifacts::present::diff::diff_set_presentation(&source, &tiles))
}
//#endregion 🔹Diff
