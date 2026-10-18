//! ↩️ Inverse of `SetWallType`: an absolute `SetWallType` restoring the base value of exactly the fields the forward really changes, none when the wall type is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_wall_type::SetWallType;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot, diff::Patch};

pub fn inverse<Op>(payload: &SetWallType, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_wall_type::SetWallType>,
{
    let Some(record) = base.wall_types.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetWallType::from_patch(payload.id.clone(), restore))]
}
