//! ↩️ Concrete set-option-group inverse mutations.
use crate::standards::v1::subsets::any::schema::mutations::set_option_group::SetOptionGroup;
use crate::standards::v1::subsets::any::schema::{diff::Patch, snapshot::ModelSnapshot};
pub fn inverse<Op>(payload: &SetOptionGroup, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_option_group::SetOptionGroup>,
{
    let Some(record) = base.option_groups.get(&payload.id) else { return Vec::new(); };
    let patch = payload.patch().minimal(record).restoring(record);
    if patch.is_empty() { return Vec::new(); }
    vec![Op::from(SetOptionGroup::from_patch(payload.id.clone(), patch))]
}
