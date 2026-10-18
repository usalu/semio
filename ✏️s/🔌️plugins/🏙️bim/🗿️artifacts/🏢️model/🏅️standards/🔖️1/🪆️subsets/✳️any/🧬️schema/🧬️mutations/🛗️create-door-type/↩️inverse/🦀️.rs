//! ↩️ Inverse of `CreateDoorType`: the concrete `DeleteDoorType` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_door_type::DeleteDoorType;
use crate::standards::v1::subsets::any::schema::mutations::create_door_type::CreateDoorType;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateDoorType, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_door_type::DeleteDoorType>,
{
    if base.door_types.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteDoorType { id: payload.id.clone() })]
}
