//! ↩️ Inverse reconstruction for `rename-tile` — reads the BASE name, never the diff.
use super::mutation::RenameTile;
use crate::artifacts::present::mutations::PresentMutation;
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores the tile's pre-rename name, captured from `base` — missing target returns
/// `Vec::new()`.
pub fn inverse(payload: &RenameTile, base: &PresentSnapshot) -> Vec<PresentMutation> {
    let (_, tiles) = crate::artifacts::present::present_working_scene(base);
    let Some(tile) = tiles.iter().find(|tile| tile.id == payload.id) else {
        return Vec::new();
    };
    vec![PresentMutation::RenameTile(RenameTile { id: payload.id.clone(), new_name: tile.name.clone() })]
}
//#endregion 🔹Inverse
