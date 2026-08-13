//! ↩️ Inverse reconstruction for `reorder-tiles` — reads the BASE position, never the diff.
use super::mutation::ReorderTiles;
use crate::artifacts::present::mutations::PresentMutation;
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Inverse
/// ↩️ Undo moves the tile back to its pre-reorder index, captured from `base` — missing target
/// returns `Vec::new()`.
pub fn inverse(payload: &ReorderTiles, base: &PresentSnapshot) -> Vec<PresentMutation> {
    let (_, tiles) = crate::artifacts::present::present_working_scene(base);
    let Some(current_index) = tiles.iter().position(|tile| tile.id == payload.id) else {
        return Vec::new();
    };
    vec![PresentMutation::ReorderTiles(ReorderTiles { id: payload.id.clone(), to_index: current_index })]
}
//#endregion 🔹Inverse
