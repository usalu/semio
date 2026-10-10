//! ↩️ Concrete delete-design-option inverse mutations.
use super::DeleteDesignOption;
use crate::*;
pub fn inverse(payload: &DeleteDesignOption, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.design_options.get(&payload.id) else { return Vec::new(); };
    let mut result = Vec::new();
    for (id, member) in &base.element_options { if member.target == payload.id { result.push(ModelMutation::SetElementOption(super::super::set_element_option::SetElementOption { id: id.clone(), option: Some(member.target.clone()) })); } }
    result.push(ModelMutation::CreateDesignOption(super::super::create_design_option::CreateDesignOption { id: payload.id.clone(), design_option: record.clone() }));
    result
}
