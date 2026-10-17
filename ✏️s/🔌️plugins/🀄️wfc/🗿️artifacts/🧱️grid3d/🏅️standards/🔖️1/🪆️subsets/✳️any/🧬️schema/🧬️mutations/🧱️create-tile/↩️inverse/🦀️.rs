//! ↩️ Inverse for `CreateTile` — the mutation list that carries the applied state back to
//! `base`, restoring row POSITION as well as row value.

use crate::standards::v1::subsets::any::schema::mutations::delete_tile::delete_tile;
use crate::schema::snapshot::*;

pub fn inverse<Op>(payload: &super::CreateTile, base: &Grid3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_tile::DeleteTile>,
{
    Ok((|| {
    let _ = base;
    vec![delete_tile(payload.tile.id.clone())]

    })())
}
