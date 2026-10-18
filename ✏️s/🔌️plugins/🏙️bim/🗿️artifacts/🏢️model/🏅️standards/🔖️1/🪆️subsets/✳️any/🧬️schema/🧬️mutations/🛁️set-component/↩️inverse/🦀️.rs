//! ↩️ Inverse of `SetComponent`: an absolute `SetComponent` restoring the base value of exactly the fields the forward really changes, none when the component is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_component::SetComponent;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot, diff::Patch};

pub fn inverse<Op>(payload: &SetComponent, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_component::SetComponent>,
{
    let Some(record) = base.components.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetComponent::from_patch(payload.id.clone(), restore))]
}
