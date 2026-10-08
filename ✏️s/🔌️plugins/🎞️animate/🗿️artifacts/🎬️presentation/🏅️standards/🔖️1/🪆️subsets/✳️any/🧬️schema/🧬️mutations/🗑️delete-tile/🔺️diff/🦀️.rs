//! 🔺️ Sparse diff construction for `delete-tile`.
use super::DeleteTile;
use crate::diff::{diff_set_presentation, PresentationDiff, PresentationSourcePatch, PresentationTilePatch, PresentationTilesDelta};
use crate::PresentationSnapshot;

//#region 🔹Diff
/// 🔺️ Reads the working-scene `(source, tiles)` off `base.presentation`, removes the addressed
/// tile, and mints a new content-addressed `presentation` handle for the result — real handcrafted
/// construction from `(payload, base)`, never apply-then-capture.
pub fn diff(payload: &DeleteTile, base: &PresentationSnapshot) -> protocol::MutationOutcome<PresentationDiff> {
    if !base.tiles.iter().any(|tile| tile.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tile \"{}\" does not exist.", payload.id), ["tiles".to_string(), payload.id.clone()]);
    }
    protocol::MutationOutcome::new(diff_set_presentation(base, None, Some(PresentationTilesDelta { removed: vec![payload.id.clone()], ..Default::default() })))
}
//#endregion 🔹Diff
