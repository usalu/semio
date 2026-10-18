//! ↩️ Inverse of `CreateWallType`: the concrete `DeleteWallType` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_wall_type::DeleteWallType;
use crate::standards::v1::subsets::any::schema::mutations::create_wall_type::CreateWallType;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateWallType, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_wall_type::DeleteWallType>,
{
    if base.wall_types.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteWallType { id: payload.id.clone() })]
}
