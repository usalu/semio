//! ↩️ Inverse reconstruction for `replace-tiles` — reads the BASE tiles, never the diff.
use super::mutation::ReplaceTiles;
use crate::artifacts::present::mutations::PresentMutation;
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores `base.tiles` wholesale — captured from pre-state, never from the applied diff.
pub async fn inverse(_payload: &ReplaceTiles, base: &PresentSnapshot) -> Vec<PresentMutation> {
    let (_, tiles) = crate::artifacts::present::present_working_scene(base);
    vec![PresentMutation::ReplaceTiles(ReplaceTiles { new_tiles: tiles })]
}
//#endregion 🔹Inverse
