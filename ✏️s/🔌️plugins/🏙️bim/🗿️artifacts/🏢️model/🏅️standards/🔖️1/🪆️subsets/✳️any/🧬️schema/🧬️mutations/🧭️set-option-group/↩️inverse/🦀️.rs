//! ↩️ Concrete set-option-group inverse mutations.
use super::SetOptionGroup;
use crate::*;
pub fn inverse(payload: &SetOptionGroup, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.option_groups.get(&payload.id) else { return Vec::new(); };
    let patch = payload.patch().minimal(record).restoring(record);
    if patch.is_empty() { return Vec::new(); }
    vec![ModelMutation::SetOptionGroup(SetOptionGroup::from_patch(payload.id.clone(), patch))]
}
