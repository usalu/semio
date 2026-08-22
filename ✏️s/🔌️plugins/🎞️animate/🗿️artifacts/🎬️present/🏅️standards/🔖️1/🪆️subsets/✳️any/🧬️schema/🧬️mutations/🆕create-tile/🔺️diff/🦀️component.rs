//! 🔺️ Sparse diff construction for `create-tile`.
use super::mutation::CreateTile;
use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Diff
/// 🔺️ Reads the working-scene `(source, tiles)` off `base.presentation`, inserts `payload.tile` at
/// `payload.index` (clamped, FINAL-state per the taxonomy's index-addressing law), and mints a new
/// content-addressed `presentation` handle for the result — real handcrafted construction from
/// `(payload, base)`, never apply-then-capture.
pub fn diff(payload: &CreateTile, base: &PresentSnapshot) -> protocol::MutationOutcome<PresentDiff> {
    let (source, mut tiles) = crate::artifacts::present::present_working_scene(base);
    if tiles.iter().any(|tile| tile.id == payload.tile.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A tile with id \"{}\" already exists.", payload.tile.id), ["tiles".to_string(), payload.tile.id.clone()]);
    }
    let at = payload.index.min(tiles.len());
    tiles.insert(at, payload.tile.clone());
    protocol::MutationOutcome::new(crate::artifacts::present::diff::diff_set_presentation(&source, &tiles))
}
//#endregion 🔹Diff
