//! ↩️ Inverse of `SetCeilingType`: an absolute `SetCeilingType` restoring the base value of exactly the fields the forward really changes, none when the ceiling type is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_ceiling_type::SetCeilingType;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot, diff::Patch};

pub fn inverse<Op>(payload: &SetCeilingType, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_ceiling_type::SetCeilingType>,
{
    let Some(record) = base.ceiling_types.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetCeilingType::from_patch(payload.id.clone(), restore))]
}
