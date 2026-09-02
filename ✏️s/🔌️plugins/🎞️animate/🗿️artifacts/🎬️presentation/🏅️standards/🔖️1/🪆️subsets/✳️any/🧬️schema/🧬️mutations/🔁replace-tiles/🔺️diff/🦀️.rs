//! 🔺️ Sparse diff construction for `replace-tiles`.
use super::ReplaceTiles;
use crate::artifacts::presentation::diff::PresentationDiff;
use crate::artifacts::presentation::PresentationSnapshot;

//#region 🔹Diff
/// 🔺️ Reads the working-scene `source` off `base.presentation` (unchanged by this mutation) and
/// mints a new content-addressed `presentation` handle for `(source, payload.new_tiles)` — real
/// handcrafted construction from `(payload, base)`, never apply-then-capture.
pub fn diff(payload: &ReplaceTiles, base: &PresentationSnapshot) -> protocol::MutationOutcome<PresentationDiff> {
    let (source, tiles) = crate::artifacts::presentation::presentation_working_scene(base);
    if tiles == payload.new_tiles {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "The tiles collection is already unchanged.".to_string());
    }
    protocol::MutationOutcome::new(crate::artifacts::presentation::diff::diff_set_presentation(&source, &payload.new_tiles))
}
//#endregion 🔹Diff
