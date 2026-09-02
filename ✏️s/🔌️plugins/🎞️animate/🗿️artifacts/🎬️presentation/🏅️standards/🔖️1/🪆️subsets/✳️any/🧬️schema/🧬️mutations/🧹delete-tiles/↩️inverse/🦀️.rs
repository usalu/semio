//! ↩️ Inverse reconstruction for `delete-tiles` — reads the BASE tiles, never the diff.
use super::DeleteTiles;
use crate::artifacts::presentation::mutations::create_tile::CreateTile;
use crate::artifacts::presentation::mutations::PresentationMutation;
use crate::artifacts::presentation::PresentationSnapshot;
use std::collections::HashSet;

//#region 🔹Inverse
/// ↩️ Undo re-creates every removed tile at its pre-deletion index, captured from `base`, in the
/// order it originally held — ids already absent from `base` contribute nothing, matching the
/// taxonomy's rule for a mutation with nothing to undo.
pub fn inverse(payload: &DeleteTiles, base: &PresentationSnapshot) -> Vec<PresentationMutation> {
    let (_, tiles) = crate::artifacts::presentation::presentation_working_scene(base);
    let targets: HashSet<&str> = payload.ids.iter().map(String::as_str).collect();
    tiles.iter().enumerate().filter(|(_, tile)| targets.contains(tile.id.as_str())).map(|(index, tile)| PresentationMutation::CreateTile(CreateTile { index, tile: tile.clone() })).collect()
}
//#endregion 🔹Inverse
