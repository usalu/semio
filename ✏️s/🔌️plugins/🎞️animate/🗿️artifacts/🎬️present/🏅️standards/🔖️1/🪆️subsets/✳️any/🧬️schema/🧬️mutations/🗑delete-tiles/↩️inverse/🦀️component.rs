//! ↩️ Inverse reconstruction for `delete-tiles` — reads the BASE tiles, never the diff.
use super::mutation::DeleteTiles;
use crate::artifacts::present::mutations::create_tile::mutation::CreateTile;
use crate::artifacts::present::mutations::PresentMutation;
use crate::artifacts::present::PresentSnapshot;
use std::collections::HashSet;

//#region 🔹Inverse
/// ↩️ Undo re-creates every removed tile at its pre-deletion index, captured from `base`, in the
/// order it originally held — ids already absent from `base` contribute nothing (the taxonomy's
/// replacement for the banned `NoMutation` sentinel).
pub fn inverse(payload: &DeleteTiles, base: &PresentSnapshot) -> Vec<PresentMutation> {
    let targets: HashSet<&str> = payload.ids.iter().map(String::as_str).collect();
    base.tiles
        .iter()
        .enumerate()
        .filter(|(_, tile)| targets.contains(tile.id.as_str()))
        .map(|(index, tile)| PresentMutation::CreateTile(CreateTile { index, tile: tile.clone() }))
        .collect()
}
//#endregion 🔹Inverse
