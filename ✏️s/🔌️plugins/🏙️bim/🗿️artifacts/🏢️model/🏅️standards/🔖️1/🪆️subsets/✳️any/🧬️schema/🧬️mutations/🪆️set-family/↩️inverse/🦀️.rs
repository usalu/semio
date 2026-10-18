//! ↩️ Inverse of `SetFamily`: an absolute `SetFamily` restoring the base value of exactly the fields the forward really changes, none when the family is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_family::SetFamily;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot, diff::Patch};

pub fn inverse<Op>(payload: &SetFamily, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_family::SetFamily>,
{
    let Some(record) = base.families.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetFamily::from_patch(payload.id.clone(), restore))]
}
