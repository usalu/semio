//! 🔺️ Sparse diff construction for `reorder-tiles`.
use super::ReorderTiles;
use crate::diff::{PresentationDiff, PresentationSourcePatch, PresentationTilePatch, PresentationTilesDelta};
use crate::PresentationSnapshot;

//#region 🔹Diff
/// 🔺️ Reads the working-scene `(source, tiles)` off `base.presentation`, relocates the addressed
/// tile to `to_index` (clamped), and mints a new content-addressed `presentation` handle for the
/// result — real handcrafted construction from `(payload, base)`, never apply-then-capture.
pub fn diff(payload: &ReorderTiles, base: &PresentationSnapshot) -> protocol::MutationOutcome<PresentationDiff> {
    let Some(from) = base.tiles.iter().position(|tile| tile.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tile \"{}\" does not exist.", payload.id), ["tiles".to_string(), payload.id.clone()]);
    };
    let others: Vec<&str> = base.tiles.iter().map(|tile| tile.id.as_str()).filter(|id| *id != payload.id).collect();
    let to = payload.to_index.min(others.len());
    if to == from {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Tile \"{}\" is already at index {to}.", payload.id));
    }
    protocol::MutationOutcome::new(PresentationDiff { tiles: Some(PresentationTilesDelta::relocation(&base.tiles, from, to)), ..Default::default() })
}
//#endregion 🔹Diff
