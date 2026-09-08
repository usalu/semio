//! ↩️ Inverse reconstruction for `resize-tile-crop` — reads the BASE crop, never the diff.
use super::ResizeTileCrop;
use crate::mutations::PresentationMutation;
use crate::PresentationSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores the tile's pre-resize crop, captured from `base` — missing target returns
/// `Vec::new()`.
pub fn inverse(payload: &ResizeTileCrop, base: &PresentationSnapshot) -> Vec<PresentationMutation> {
    let (_, tiles) = crate::presentation_working_scene(base);
    let Some(tile) = tiles.iter().find(|tile| tile.id == payload.id) else {
        return Vec::new();
    };
    vec![PresentationMutation::ResizeTileCrop(ResizeTileCrop { id: payload.id.clone(), new_crop: tile.crop.clone() })]
}
//#endregion 🔹Inverse
