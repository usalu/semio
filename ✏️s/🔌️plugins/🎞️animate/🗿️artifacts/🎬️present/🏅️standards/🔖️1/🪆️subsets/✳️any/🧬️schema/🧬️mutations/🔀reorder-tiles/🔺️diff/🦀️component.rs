//! 🔺️ Sparse diff construction for `reorder-tiles`.
use super::mutation::ReorderTiles;
use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Diff
/// 🔺️ Reads the working-scene `(source, tiles)` off `base.presentation`, relocates the addressed
/// tile to `to_index` (clamped), and mints a new content-addressed `presentation` handle for the
/// result — real handcrafted construction from `(payload, base)`, never apply-then-capture.
pub fn diff(payload: &ReorderTiles, base: &PresentSnapshot) -> PresentDiff {
    let (source, mut tiles) = crate::artifacts::present::present_working_scene(base);
    if let Some(from) = tiles.iter().position(|tile| tile.id == payload.id) {
        let item = tiles.remove(from);
        let to = payload.to_index.min(tiles.len());
        tiles.insert(to, item);
    }
    crate::artifacts::present::diff::diff_set_presentation(&source, &tiles)
}
//#endregion 🔹Diff
