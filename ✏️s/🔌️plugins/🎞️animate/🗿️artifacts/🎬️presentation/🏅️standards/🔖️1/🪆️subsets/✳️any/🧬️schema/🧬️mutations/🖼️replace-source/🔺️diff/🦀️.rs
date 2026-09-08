//! 🔺️ Sparse diff construction for `replace-source`.
use super::ReplaceSource;
use crate::diff::PresentationDiff;
use crate::PresentationSnapshot;

//#region 🔹Diff
/// 🔺️ Reads the working-scene `tiles` off `base.presentation` (unchanged by this mutation) and
/// mints a new content-addressed `presentation` handle for `(payload.new_source, tiles)` — real
/// handcrafted construction from `(payload, base)`, never apply-then-capture.
pub fn diff(payload: &ReplaceSource, base: &PresentationSnapshot) -> protocol::MutationOutcome<PresentationDiff> {
    let (source, tiles) = crate::presentation_working_scene(base);
    if source == payload.new_source {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Source is already unchanged.".to_string());
    }
    protocol::MutationOutcome::new(crate::diff::diff_set_presentation(&payload.new_source, &tiles))
}
//#endregion 🔹Diff
