//! 🔺️ Sparse diff construction for `reorder-tiles`.
use super::mutation::ReorderTiles;
use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Diff
/// 🔺️ Reads the working-scene `(source, tiles)` off `base.presentation`, relocates the addressed
/// tile to `to_index` (clamped), and mints a new content-addressed `presentation` handle for the
/// result — real handcrafted construction from `(payload, base)`, never apply-then-capture.
pub fn diff(payload: &ReorderTiles, base: &PresentSnapshot) -> protocol::MutationOutcome<PresentDiff> {
    let (source, mut tiles) = crate::artifacts::present::present_working_scene(base);
    let Some(from) = tiles.iter().position(|tile| tile.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tile \"{}\" does not exist.", payload.id), ["tiles".to_string(), payload.id.clone()]);
    };
    let item = tiles.remove(from);
    let to = payload.to_index.min(tiles.len());
    if to == from {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Tile \"{}\" is already at index {to}.", payload.id));
    }
    tiles.insert(to, item);
    protocol::MutationOutcome::new(crate::artifacts::present::diff::diff_set_presentation(&source, &tiles))
}
//#endregion 🔹Diff
