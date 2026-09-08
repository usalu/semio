//! ↩️ Inverse reconstruction for `delete-tile` — reads the BASE tile, never the diff.
use super::DeleteTile;
use crate::mutations::create_tile::CreateTile;
use crate::mutations::PresentationMutation;
use crate::PresentationSnapshot;

//#region 🔹Inverse
/// ↩️ Undo re-creates the tile at its pre-deletion index, captured from `base` — missing target
/// (already absent) returns `Vec::new()` — the taxonomy's rule for a mutation with nothing to
/// undo, replacing any sentinel no-op variant.
pub fn inverse(payload: &DeleteTile, base: &PresentationSnapshot) -> Vec<PresentationMutation> {
    let (_, tiles) = crate::presentation_working_scene(base);
    let Some(index) = tiles.iter().position(|tile| tile.id == payload.id) else {
        return Vec::new();
    };
    vec![PresentationMutation::CreateTile(CreateTile { index, tile: tiles[index].clone() })]
}
//#endregion 🔹Inverse
