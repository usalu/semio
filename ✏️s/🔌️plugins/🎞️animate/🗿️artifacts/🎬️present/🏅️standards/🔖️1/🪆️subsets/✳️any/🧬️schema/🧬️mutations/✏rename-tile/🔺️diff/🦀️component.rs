//! 🔺️ Sparse diff construction for `rename-tile`.
use super::mutation::RenameTile;
use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Diff
/// 🔺️ Reads the working-scene `(source, tiles)` off `base.presentation`, applies the name-only
/// patch to the addressed tile, and mints a new content-addressed `presentation` handle for the
/// result — real handcrafted construction from `(payload, base)`, never apply-then-capture.
pub fn diff(payload: &RenameTile, base: &PresentSnapshot) -> protocol::MutationOutcome<PresentDiff> {
    let (source, mut tiles) = crate::artifacts::present::present_working_scene(base);
    let Some(existing) = tiles.iter().find(|tile| tile.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Tile \"{}\" does not exist.", payload.id), ["tiles".to_string(), payload.id.clone()]);
    };
    if existing.name == payload.new_name {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Tile \"{}\" is already named \"{}\".", payload.id, payload.new_name));
    }
    if let Some(tile) = tiles.iter_mut().find(|tile| tile.id == payload.id) {
        tile.name = payload.new_name.clone();
    }
    protocol::MutationOutcome::new(crate::artifacts::present::diff::diff_set_presentation(&source, &tiles))
}
//#endregion 🔹Diff
