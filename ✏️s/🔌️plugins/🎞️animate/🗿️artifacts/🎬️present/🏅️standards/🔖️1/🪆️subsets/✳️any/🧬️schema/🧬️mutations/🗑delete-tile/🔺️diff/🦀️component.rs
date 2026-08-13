//! 🔺️ Sparse diff construction for `delete-tile`.
use super::mutation::DeleteTile;
use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Diff
/// 🔺️ Reads the working-scene `(source, tiles)` off `base.presentation`, removes the addressed
/// tile, and mints a new content-addressed `presentation` handle for the result — real handcrafted
/// construction from `(payload, base)`, never apply-then-capture.
pub fn diff(payload: &DeleteTile, base: &PresentSnapshot) -> PresentDiff {
    let (source, mut tiles) = crate::artifacts::present::present_working_scene(base);
    tiles.retain(|tile| tile.id != payload.id);
    crate::artifacts::present::diff::diff_set_presentation(&source, &tiles)
}
//#endregion 🔹Diff
