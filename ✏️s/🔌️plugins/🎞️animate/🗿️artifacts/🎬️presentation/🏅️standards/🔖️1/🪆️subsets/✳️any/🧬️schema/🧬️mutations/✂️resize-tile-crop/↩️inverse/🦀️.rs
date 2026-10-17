//! ↩️ Inverse reconstruction for `resize-tile-crop` — reads the BASE crop, never the diff.
use super::ResizeTileCrop;
use crate::PresentationSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores the tile's pre-resize crop, captured from `base` — missing target returns
/// `Vec::new()`.
pub fn inverse<Op>(payload: &ResizeTileCrop, base: &PresentationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::resize_tile_crop::ResizeTileCrop>,
{
    Ok((|| {
    let (_, tiles) = crate::presentation_working_scene(base);
    let Some(tile) = tiles.iter().find(|tile| tile.id == payload.id) else {
        return Vec::new();
    };
    vec![Op::from(ResizeTileCrop { id: payload.id.clone(), new_crop: tile.crop.clone() })]

    })())
}
//#endregion 🔹Inverse
