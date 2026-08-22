//! 🔺️ Sparse diff construction for `replace-tiles`.
use super::mutation::ReplaceTiles;
use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Diff
/// 🔺️ Reads the working-scene `source` off `base.presentation` (unchanged by this mutation) and
/// mints a new content-addressed `presentation` handle for `(source, payload.new_tiles)` — real
/// handcrafted construction from `(payload, base)`, never apply-then-capture.
pub fn diff(payload: &ReplaceTiles, base: &PresentSnapshot) -> protocol::MutationOutcome<PresentDiff> {
    let (source, tiles) = crate::artifacts::present::present_working_scene(base);
    if tiles == payload.new_tiles {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "The tiles collection is already unchanged.".to_string());
    }
    protocol::MutationOutcome::new(crate::artifacts::present::diff::diff_set_presentation(&source, &payload.new_tiles))
}
//#endregion 🔹Diff
