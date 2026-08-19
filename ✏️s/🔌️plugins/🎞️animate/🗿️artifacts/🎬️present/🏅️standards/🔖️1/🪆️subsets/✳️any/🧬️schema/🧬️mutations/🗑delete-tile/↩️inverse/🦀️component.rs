//! ↩️ Inverse reconstruction for `delete-tile` — reads the BASE tile, never the diff.
use super::mutation::DeleteTile;
use crate::artifacts::present::mutations::create_tile::mutation::CreateTile;
use crate::artifacts::present::mutations::PresentMutation;
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Inverse
/// ↩️ Undo re-creates the tile at its pre-deletion index, captured from `base` — missing target
/// (already absent) returns `Vec::new()` — the taxonomy's rule for a mutation with nothing to
/// undo, replacing any sentinel no-op variant.
pub async fn inverse(payload: &DeleteTile, base: &PresentSnapshot) -> Vec<PresentMutation> {
    let (_, tiles) = crate::artifacts::present::present_working_scene(base);
    let Some(index) = tiles.iter().position(|tile| tile.id == payload.id) else {
        return Vec::new();
    };
    vec![PresentMutation::CreateTile(CreateTile { index, tile: tiles[index].clone() })]
}
//#endregion 🔹Inverse
