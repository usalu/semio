//! ↩️ Inverse of `SetSpace`: an absolute `SetSpace` restoring the base value of exactly the fields the forward really changes, none when the space is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_space::SetSpace;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot, diff::Patch};

pub fn inverse<Op>(payload: &SetSpace, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_space::SetSpace>,
{
    let Some(record) = base.spaces.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetSpace::from_patch(payload.id.clone(), restore))]
}
