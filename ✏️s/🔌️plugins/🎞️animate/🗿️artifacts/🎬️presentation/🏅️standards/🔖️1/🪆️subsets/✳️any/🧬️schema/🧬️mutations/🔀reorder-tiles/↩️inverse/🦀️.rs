//! ↩️ Inverse reconstruction for `reorder-tiles` — reads the BASE position, never the diff.
use super::ReorderTiles;
use crate::artifacts::presentation::mutations::PresentationMutation;
use crate::artifacts::presentation::PresentationSnapshot;

//#region 🔹Inverse
/// ↩️ Undo moves the tile back to its pre-reorder index, captured from `base` — missing target
/// returns `Vec::new()`.
pub fn inverse(payload: &ReorderTiles, base: &PresentationSnapshot) -> Vec<PresentationMutation> {
    let (_, tiles) = crate::artifacts::presentation::presentation_working_scene(base);
    let Some(current_index) = tiles.iter().position(|tile| tile.id == payload.id) else {
        return Vec::new();
    };
    vec![PresentationMutation::ReorderTiles(ReorderTiles { id: payload.id.clone(), to_index: current_index })]
}
//#endregion 🔹Inverse
