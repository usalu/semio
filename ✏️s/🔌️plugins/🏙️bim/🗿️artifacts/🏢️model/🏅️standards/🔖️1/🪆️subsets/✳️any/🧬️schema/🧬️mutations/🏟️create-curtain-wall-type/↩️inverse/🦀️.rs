//! ↩️ Inverse of `CreateCurtainWallType`: the concrete `DeleteCurtainWallType` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_curtain_wall_type::DeleteCurtainWallType;
use crate::standards::v1::subsets::any::schema::mutations::create_curtain_wall_type::CreateCurtainWallType;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateCurtainWallType, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_curtain_wall_type::DeleteCurtainWallType>,
{
    if base.curtain_wall_types.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteCurtainWallType { id: payload.id.clone() })]
}
