//! ↩️ Inverse reconstruction for `replace-tiles` — reads the BASE tiles, never the diff.
use super::ReplaceTiles;
use crate::artifacts::presentation::mutations::PresentationMutation;
use crate::artifacts::presentation::PresentationSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores `base.tiles` wholesale — captured from pre-state, never from the applied diff.
pub fn inverse(_payload: &ReplaceTiles, base: &PresentationSnapshot) -> Vec<PresentationMutation> {
    let (_, tiles) = crate::artifacts::presentation::presentation_working_scene(base);
    vec![PresentationMutation::ReplaceTiles(ReplaceTiles { new_tiles: tiles })]
}
//#endregion 🔹Inverse
