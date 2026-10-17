//! ↩️ Inverse for `CreateTile` — the `delete-tile` of the id it created.

use crate::standards::v1::subsets::any::schema::mutations::delete_tile::delete_tile;
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse<Op>(payload: &super::CreateTile, _base: &Wfc3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_tile::DeleteTile>,
{
    Ok((|| {
    vec![delete_tile(payload.tile.id.clone())]

    })())
}
