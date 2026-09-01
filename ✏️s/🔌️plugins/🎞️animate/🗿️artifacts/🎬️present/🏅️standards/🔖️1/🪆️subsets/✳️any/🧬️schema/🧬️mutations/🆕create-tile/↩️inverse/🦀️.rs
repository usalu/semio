//! ↩️ Inverse reconstruction for `create-tile` — undo deletes the created tile.
use super::CreateTile;
use crate::artifacts::present::mutations::delete_tile::DeleteTile;
use crate::artifacts::present::mutations::PresentMutation;
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Inverse
/// ↩️ Undo removes the tile this mutation created, addressed by its own id (captured from the
/// payload itself — a `create` has nothing to look up in pre-state).
pub fn inverse(payload: &CreateTile, _base: &PresentSnapshot) -> Vec<PresentMutation> {
    vec![PresentMutation::DeleteTile(DeleteTile { id: payload.tile.id.clone() })]
}
//#endregion 🔹Inverse
