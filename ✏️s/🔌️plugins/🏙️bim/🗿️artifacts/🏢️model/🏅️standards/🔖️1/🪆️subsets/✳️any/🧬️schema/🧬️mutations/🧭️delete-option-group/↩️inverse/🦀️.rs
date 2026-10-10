//! ↩️ Concrete delete-option-group inverse mutations.
use super::DeleteOptionGroup;
use crate::*;
pub fn inverse(payload: &DeleteOptionGroup, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.option_groups.get(&payload.id) else { return Vec::new(); };
    let mut result = Vec::new();
    result.push(ModelMutation::CreateOptionGroup(super::super::create_option_group::CreateOptionGroup { id: payload.id.clone(), option_group: record.clone() }));
    result
}
