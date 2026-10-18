//! ↩️ Inverse of `SetSlabType`: an absolute `SetSlabType` restoring the base value of exactly the fields the forward really changes, none when the slab type is absent or nothing changes.

use crate::standards::v1::subsets::any::schema::mutations::set_slab_type::SetSlabType;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot, diff::Patch};

pub fn inverse<Op>(payload: &SetSlabType, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_slab_type::SetSlabType>,
{
    let Some(record) = base.slab_types.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![Op::from(SetSlabType::from_patch(payload.id.clone(), restore))]
}
