//! ↩️ Inverse reconstruction for `rename-tile` — reads the BASE name, never the diff.
use super::RenameTile;
use crate::PresentationSnapshot;

//#region 🔹Inverse
/// ↩️ Undo restores the tile's pre-rename name, captured from `base` — missing target returns
/// `Vec::new()`.
pub fn inverse<Op>(payload: &RenameTile, base: &PresentationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::rename_tile::RenameTile>,
{
    Ok((|| {
    let (_, tiles) = crate::presentation_working_scene(base);
    let Some(tile) = tiles.iter().find(|tile| tile.id == payload.id) else {
        return Vec::new();
    };
    vec![Op::from(RenameTile { id: payload.id.clone(), new_name: tile.name.clone() })]

    })())
}
//#endregion 🔹Inverse
