//! ↩️ Inverse reconstruction for `delete-tiles` — reads the BASE tiles, never the diff.
use super::DeleteTiles;
use crate::mutations::create_tile::CreateTile;
use crate::mutations::PresentationMutation;
use crate::PresentationSnapshot;
use std::collections::HashSet;

//#region 🔹Inverse
/// ↩️ Undo re-creates every removed tile at its pre-deletion index, captured from `base`, in the
/// order it originally held (stored last-created-first, so the reversed replay re-inserts the lowest index first) — ids already absent from `base` contribute nothing, matching the
/// taxonomy's rule for a mutation with nothing to undo.
pub fn inverse(payload: &DeleteTiles, base: &PresentationSnapshot) -> Result<Vec<PresentationMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let (_, tiles) = crate::presentation_working_scene(base);
    let targets: HashSet<&str> = payload.ids.iter().map(String::as_str).collect();
    let mut steps: Vec<PresentationMutation> = tiles.iter().enumerate().filter(|(_, tile)| targets.contains(tile.id.as_str())).map(|(index, tile)| PresentationMutation::CreateTile(CreateTile { index, tile: tile.clone() })).collect();
    steps.reverse();
    steps

    })())
}
//#endregion 🔹Inverse
