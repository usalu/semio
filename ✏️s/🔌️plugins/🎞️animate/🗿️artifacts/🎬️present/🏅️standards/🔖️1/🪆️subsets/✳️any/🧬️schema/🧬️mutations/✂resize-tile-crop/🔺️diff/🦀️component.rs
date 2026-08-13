//! 🔺️ Sparse diff construction for `resize-tile-crop`.
use super::mutation::ResizeTileCrop;
use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Diff
/// 🔺️ Reads the working-scene `(source, tiles)` off `base.presentation`, applies the crop-only
/// patch to the addressed tile, and mints a new content-addressed `presentation` handle for the
/// result — real handcrafted construction from `(payload, base)`, never apply-then-capture.
pub fn diff(payload: &ResizeTileCrop, base: &PresentSnapshot) -> PresentDiff {
    let (source, mut tiles) = crate::artifacts::present::present_working_scene(base);
    if let Some(tile) = tiles.iter_mut().find(|tile| tile.id == payload.id) {
        tile.crop = payload.new_crop.clone();
    }
    crate::artifacts::present::diff::diff_set_presentation(&source, &tiles)
}
//#endregion 🔹Diff
