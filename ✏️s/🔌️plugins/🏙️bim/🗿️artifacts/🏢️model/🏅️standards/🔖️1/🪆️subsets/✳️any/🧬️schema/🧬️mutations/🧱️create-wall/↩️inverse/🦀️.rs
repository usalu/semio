//! ↩️ Inverse of `CreateWall`: the concrete `DeleteWall` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_wall::DeleteWall;
use crate::standards::v1::subsets::any::schema::mutations::create_wall::CreateWall;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateWall, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_wall::DeleteWall>,
{
    if base.walls.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteWall { id: payload.id.clone() })]
}
