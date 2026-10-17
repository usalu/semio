//! ↩️ Inverse reconstruction for `reorder-tiles` — reads the BASE position, never the diff.
use super::ReorderTiles;
use crate::PresentationSnapshot;

//#region 🔹Inverse
/// ↩️ Undo moves the tile back to its pre-reorder index, captured from `base` — missing target
/// returns `Vec::new()`.
pub fn inverse<Op>(payload: &ReorderTiles, base: &PresentationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::reorder_tiles::ReorderTiles>,
{
    Ok((|| {
    let (_, tiles) = crate::presentation_working_scene(base);
    let Some(current_index) = tiles.iter().position(|tile| tile.id == payload.id) else {
        return Vec::new();
    };
    vec![Op::from(ReorderTiles { id: payload.id.clone(), to_index: current_index })]

    })())
}
//#endregion 🔹Inverse
