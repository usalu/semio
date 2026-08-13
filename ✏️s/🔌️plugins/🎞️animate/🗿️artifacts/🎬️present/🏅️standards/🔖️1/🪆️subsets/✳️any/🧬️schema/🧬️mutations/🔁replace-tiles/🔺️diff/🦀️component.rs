//! 🔺️ Sparse diff construction for `replace-tiles`.
use super::mutation::ReplaceTiles;
use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Diff
/// 🔺️ Reads the working-scene `source` off `base.presentation` (unchanged by this mutation) and
/// mints a new content-addressed `presentation` handle for `(source, payload.new_tiles)` — real
/// handcrafted construction from `(payload, base)`, never apply-then-capture.
pub fn diff(payload: &ReplaceTiles, base: &PresentSnapshot) -> PresentDiff {
    let (source, _) = crate::artifacts::present::present_working_scene(base);
    crate::artifacts::present::diff::diff_set_presentation(&source, &payload.new_tiles)
}
//#endregion 🔹Diff
