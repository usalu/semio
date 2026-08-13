//! 🔺️ Sparse diff construction for `rename-tile`.
use super::mutation::RenameTile;
use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Diff
/// 🔺️ Reads the working-scene `(source, tiles)` off `base.presentation`, applies the name-only
/// patch to the addressed tile, and mints a new content-addressed `presentation` handle for the
/// result — real handcrafted construction from `(payload, base)`, never apply-then-capture.
pub fn diff(payload: &RenameTile, base: &PresentSnapshot) -> PresentDiff {
    let (source, mut tiles) = crate::artifacts::present::present_working_scene(base);
    if let Some(tile) = tiles.iter_mut().find(|tile| tile.id == payload.id) {
        tile.name = payload.new_name.clone();
    }
    crate::artifacts::present::diff::diff_set_presentation(&source, &tiles)
}
//#endregion 🔹Diff
