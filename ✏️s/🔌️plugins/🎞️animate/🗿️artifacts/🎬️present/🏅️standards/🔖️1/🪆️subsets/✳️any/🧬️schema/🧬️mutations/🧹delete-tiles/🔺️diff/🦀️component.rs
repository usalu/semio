//! 🔺️ Sparse diff construction for `delete-tiles`.
use super::mutation::DeleteTiles;
use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Diff
/// 🔺️ Reads the working-scene `(source, tiles)` off `base.presentation`, removes every addressed
/// tile, and mints a new content-addressed `presentation` handle for the result — real handcrafted
/// construction from `(payload, base)`, never apply-then-capture.
pub fn diff(payload: &DeleteTiles, base: &PresentSnapshot) -> PresentDiff {
    let (source, mut tiles) = crate::artifacts::present::present_working_scene(base);
    let targets: std::collections::HashSet<&str> = payload.ids.iter().map(String::as_str).collect();
    tiles.retain(|tile| !targets.contains(tile.id.as_str()));
    crate::artifacts::present::diff::diff_set_presentation(&source, &tiles)
}
//#endregion 🔹Diff
