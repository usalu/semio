//! ↩️ Inverse reconstruction for `create-tile` — undo deletes the created tile.
use super::CreateTile;
use crate::artifacts::presentation::mutations::delete_tile::DeleteTile;
use crate::artifacts::presentation::mutations::PresentationMutation;
use crate::artifacts::presentation::PresentationSnapshot;

//#region 🔹Inverse
/// ↩️ Undo removes the tile this mutation created, addressed by its own id (captured from the
/// payload itself — a `create` has nothing to look up in pre-state).
pub fn inverse(payload: &CreateTile, _base: &PresentationSnapshot) -> Vec<PresentationMutation> {
    vec![PresentationMutation::DeleteTile(DeleteTile { id: payload.tile.id.clone() })]
}
//#endregion 🔹Inverse
