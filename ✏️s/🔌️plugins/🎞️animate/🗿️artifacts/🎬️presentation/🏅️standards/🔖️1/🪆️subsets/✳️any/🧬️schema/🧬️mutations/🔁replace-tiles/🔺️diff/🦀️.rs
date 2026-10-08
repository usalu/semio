//! 🔺️ Sparse diff construction for `replace-tiles`.
use super::ReplaceTiles;
use crate::diff::{diff_set_presentation, PresentationDiff, PresentationSourcePatch, PresentationTilePatch, PresentationTilesDelta};
use crate::PresentationSnapshot;

//#region 🔹Diff
/// 🔺️ Reads the working-scene `source` off `base.presentation` (unchanged by this mutation) and
/// mints a new content-addressed `presentation` handle for `(source, payload.new_tiles)` — real
/// handcrafted construction from `(payload, base)`, never apply-then-capture.
pub fn diff(payload: &ReplaceTiles, base: &PresentationSnapshot) -> protocol::MutationOutcome<PresentationDiff> {
    if base.tiles == payload.new_tiles {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The tiles collection is already unchanged.".to_string());
    }
    protocol::MutationOutcome::new(diff_set_presentation(base, None, Some(crate::diff::tiles_replacing(&base.tiles, &payload.new_tiles))))
}
//#endregion 🔹Diff
