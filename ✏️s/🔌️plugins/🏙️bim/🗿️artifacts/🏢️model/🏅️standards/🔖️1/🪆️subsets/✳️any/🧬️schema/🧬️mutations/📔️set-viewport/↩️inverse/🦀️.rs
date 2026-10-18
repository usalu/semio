//! ↩️ Inverse of `SetViewport`: an absolute `SetViewport` restoring the base value of exactly the fields the forward really changes, none when the viewport is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_viewport::SetViewport;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot, diff::Patch};

pub fn inverse<Op>(payload: &SetViewport, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_viewport::SetViewport>,
{
    let Some(record) = base.viewports.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetViewport::from_patch(payload.id.clone(), restore))]
}
