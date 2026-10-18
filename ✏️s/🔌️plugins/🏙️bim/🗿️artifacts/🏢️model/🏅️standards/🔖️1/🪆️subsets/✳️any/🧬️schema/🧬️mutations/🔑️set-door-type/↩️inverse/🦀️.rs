//! ↩️ Inverse of `SetDoorType`: an absolute `SetDoorType` restoring the base value of exactly the fields the forward really changes, none when the door type is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_door_type::SetDoorType;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot, diff::Patch};

pub fn inverse<Op>(payload: &SetDoorType, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_door_type::SetDoorType>,
{
    let Some(record) = base.door_types.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetDoorType::from_patch(payload.id.clone(), restore))]
}
