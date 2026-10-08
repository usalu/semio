//! 🔺️ Sparse diff construction for `replace-source`.
use super::ReplaceSource;
use crate::diff::{diff_set_presentation, PresentationDiff, PresentationSourcePatch, PresentationTilePatch, PresentationTilesDelta};
use crate::PresentationSnapshot;

//#region 🔹Diff
/// 🔺️ Reads the working-scene `tiles` off `base.presentation` (unchanged by this mutation) and
/// mints a new content-addressed `presentation` handle for `(payload.new_source, tiles)` — real
/// handcrafted construction from `(payload, base)`, never apply-then-capture.
pub fn diff(payload: &ReplaceSource, base: &PresentationSnapshot) -> protocol::MutationOutcome<PresentationDiff> {
    let Some(patch) = PresentationSourcePatch::replacing(&base.source, &payload.new_source) else {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Source is already unchanged.".to_string());
    };
    protocol::MutationOutcome::new(diff_set_presentation(base, Some(patch), None))
}
//#endregion 🔹Diff
