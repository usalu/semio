//! ↩️ Inverse reconstruction for `create-tile` — undo deletes the created tile.
use super::CreateTile;
use crate::standards::v1::subsets::any::schema::mutations::delete_tile::DeleteTile;
use crate::PresentationSnapshot;

//#region 🔹Inverse
/// ↩️ Undo removes the tile this mutation created, addressed by its own id (captured from the
/// payload itself — a `create` has nothing to look up in pre-state).
pub fn inverse<Op>(payload: &CreateTile, _base: &PresentationSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_tile::DeleteTile>,
{
    Ok((|| {
    vec![Op::from(DeleteTile { id: payload.tile.id.clone() })]

    })())
}
//#endregion 🔹Inverse
