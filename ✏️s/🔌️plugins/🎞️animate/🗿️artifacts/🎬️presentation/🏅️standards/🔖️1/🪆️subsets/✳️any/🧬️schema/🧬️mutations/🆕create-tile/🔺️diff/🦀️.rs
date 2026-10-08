//! 🔺️ Sparse diff construction for `create-tile`.
use super::CreateTile;
use crate::diff::{PresentationDiff, PresentationSourcePatch, PresentationTilePatch, PresentationTilesDelta};
use crate::PresentationSnapshot;

//#region 🔹Diff
/// 🔺️ Reads the working-scene `(source, tiles)` off `base.presentation`, inserts `payload.tile` at
/// `payload.index` (clamped, FINAL-state per the taxonomy's index-addressing law), and mints a new
/// content-addressed `presentation` handle for the result — real handcrafted construction from
/// `(payload, base)`, never apply-then-capture.
pub fn diff(payload: &CreateTile, base: &PresentationSnapshot) -> protocol::MutationOutcome<PresentationDiff> {
    if base.tiles.iter().any(|tile| tile.id == payload.tile.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A tile with id \"{}\" already exists.", payload.tile.id), ["tiles".to_string(), payload.tile.id.clone()]);
    }
    let at = payload.index.min(base.tiles.len());
    protocol::MutationOutcome::new(PresentationDiff { tiles: Some(PresentationTilesDelta::insertion(at, payload.tile.clone())), ..Default::default() })
}
//#endregion 🔹Diff
