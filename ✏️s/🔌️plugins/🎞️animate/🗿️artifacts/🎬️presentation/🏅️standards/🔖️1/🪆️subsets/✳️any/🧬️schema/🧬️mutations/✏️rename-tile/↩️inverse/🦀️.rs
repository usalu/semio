//! ↩️ Inverse reconstruction for `rename-tile` — reads the BASE name, never the diff.
use super::RenameTile;
use crate::artifacts::presentation::mutations::PresentationMutation;
use crate::artifacts::presentation::PresentationSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores the tile's pre-rename name, captured from `base` — missing target returns
/// `Vec::new()`.
pub fn inverse(payload: &RenameTile, base: &PresentationSnapshot) -> Vec<PresentationMutation> {
    let (_, tiles) = crate::artifacts::presentation::presentation_working_scene(base);
    let Some(tile) = tiles.iter().find(|tile| tile.id == payload.id) else {
        return Vec::new();
    };
    vec![PresentationMutation::RenameTile(RenameTile { id: payload.id.clone(), new_name: tile.name.clone() })]
}
//#endregion 🔹Inverse
