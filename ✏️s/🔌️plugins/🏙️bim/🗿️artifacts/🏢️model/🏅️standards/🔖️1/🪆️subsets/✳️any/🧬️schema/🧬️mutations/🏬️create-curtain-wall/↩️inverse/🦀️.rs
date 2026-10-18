//! ↩️ Inverse of `CreateCurtainWall`: the concrete `DeleteCurtainWall` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_curtain_wall::DeleteCurtainWall;
use crate::standards::v1::subsets::any::schema::mutations::create_curtain_wall::CreateCurtainWall;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateCurtainWall, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_curtain_wall::DeleteCurtainWall>,
{
    if base.curtain_walls.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteCurtainWall { id: payload.id.clone() })]
}
