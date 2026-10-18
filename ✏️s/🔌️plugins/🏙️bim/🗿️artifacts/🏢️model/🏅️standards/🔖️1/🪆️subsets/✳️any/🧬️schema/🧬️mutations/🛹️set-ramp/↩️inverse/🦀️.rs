//! ↩️ Inverse of `SetRamp`: an absolute `SetRamp` restoring the base value of exactly the fields the forward really changes, none when the ramp is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_ramp::SetRamp;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot, diff::Patch};

pub fn inverse<Op>(payload: &SetRamp, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_ramp::SetRamp>,
{
    let Some(record) = base.ramps.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetRamp::from_patch(payload.id.clone(), restore))]
}
