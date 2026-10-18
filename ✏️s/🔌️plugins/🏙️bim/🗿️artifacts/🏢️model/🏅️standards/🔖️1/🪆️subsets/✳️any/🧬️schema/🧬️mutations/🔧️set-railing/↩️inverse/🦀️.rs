//! ↩️ Inverse of `SetRailing`: an absolute `SetRailing` restoring the base value of exactly the fields the forward really changes, none when the railing is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_railing::SetRailing;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot, diff::Patch};

pub fn inverse<Op>(payload: &SetRailing, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_railing::SetRailing>,
{
    let Some(record) = base.railings.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetRailing::from_patch(payload.id.clone(), restore))]
}
