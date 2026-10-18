//! ↩️ Inverse of `SetRoofType`: an absolute `SetRoofType` restoring the base value of exactly the fields the forward really changes, none when the roof type is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_roof_type::SetRoofType;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot, diff::Patch};

pub fn inverse<Op>(payload: &SetRoofType, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_roof_type::SetRoofType>,
{
    let Some(record) = base.roof_types.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetRoofType::from_patch(payload.id.clone(), restore))]
}
