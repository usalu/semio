//! ↩️ Concrete create-option-group inverse mutations.
use super::CreateOptionGroup;
use crate::*;
pub fn inverse(payload: &CreateOptionGroup, base: &ModelSnapshot) -> Vec<ModelMutation> {
    vec![ModelMutation::DeleteOptionGroup(super::super::delete_option_group::DeleteOptionGroup { id: payload.id.clone() })]
}
