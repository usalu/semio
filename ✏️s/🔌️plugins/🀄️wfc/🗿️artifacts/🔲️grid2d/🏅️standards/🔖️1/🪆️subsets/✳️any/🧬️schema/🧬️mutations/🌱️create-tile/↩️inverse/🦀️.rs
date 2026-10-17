//! ↩️ Inverse for `CreateTile` — deletes the id it created. A duplicate create was itself a no-op,
//! so it inverts to nothing.

use crate::standards::v1::subsets::any::schema::mutations::delete_tile::delete_tile;
use crate::schema::snapshot::Grid2dSnapshot;

pub fn inverse<Op>(payload: &super::CreateTile, base: &Grid2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_tile::DeleteTile>,
{
    Ok((|| {
    if base.tiles.iter().any(|tile| tile.id == payload.tile.id) {
        return Vec::new();
    }
    vec![delete_tile(payload.tile.id.clone())]

    })())
}
