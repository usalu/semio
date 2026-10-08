//! 🔺️ Sparse diff construction for `delete-tile`.
use super::DeleteTile;
use crate::diff::{PresentationDiff, PresentationSourcePatch, PresentationTilePatch, PresentationTilesDelta};
use crate::PresentationSnapshot;

//#region 🔹Diff
/// 🔺️ Reads the working-scene `(source, tiles)` off `base.presentation`, removes the addressed
/// tile, and mints a new content-addressed `presentation` handle for the result — real handcrafted
/// construction from `(payload, base)`, never apply-then-capture.
pub fn diff(payload: &DeleteTile, base: &PresentationSnapshot) -> protocol::MutationOutcome<PresentationDiff> {
    let Some(index) = base.tiles.iter().position(|tile| tile.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tile \"{}\" does not exist.", payload.id), ["tiles".to_string(), payload.id.clone()]);
    };
    protocol::MutationOutcome::new(PresentationDiff { tiles: Some(PresentationTilesDelta::removal(&base.tiles, index)), ..Default::default() })
}
//#endregion 🔹Diff
