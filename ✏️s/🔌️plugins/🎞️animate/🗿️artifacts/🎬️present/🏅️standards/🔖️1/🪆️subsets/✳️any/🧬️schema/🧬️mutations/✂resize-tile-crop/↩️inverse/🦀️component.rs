//! ↩️ Inverse reconstruction for `resize-tile-crop` — reads the BASE crop, never the diff.
use super::mutation::ResizeTileCrop;
use crate::artifacts::present::mutations::PresentMutation;
use crate::artifacts::present::PresentSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores the tile's pre-resize crop, captured from `base` — missing target returns
/// `Vec::new()`.
pub fn inverse(payload: &ResizeTileCrop, base: &PresentSnapshot) -> Vec<PresentMutation> {
    let Some(tile) = base.tiles.iter().find(|tile| tile.id == payload.id) else {
        return Vec::new();
    };
    vec![PresentMutation::ResizeTileCrop(ResizeTileCrop { id: payload.id.clone(), new_crop: tile.crop.clone() })]
}
//#endregion 🔹Inverse
