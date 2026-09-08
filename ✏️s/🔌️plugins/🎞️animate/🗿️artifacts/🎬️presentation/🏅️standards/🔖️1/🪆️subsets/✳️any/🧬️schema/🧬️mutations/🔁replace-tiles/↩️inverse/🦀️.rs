//! ↩️ Inverse reconstruction for `replace-tiles` — reads the BASE tiles, never the diff.
use super::ReplaceTiles;
use crate::mutations::PresentationMutation;
use crate::PresentationSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores `base.tiles` wholesale — captured from pre-state, never from the applied diff.
pub fn inverse(_payload: &ReplaceTiles, base: &PresentationSnapshot) -> Vec<PresentationMutation> {
    let (_, tiles) = crate::presentation_working_scene(base);
    vec![PresentationMutation::ReplaceTiles(ReplaceTiles { new_tiles: tiles })]
}
//#endregion 🔹Inverse
