//! ↩️ Concrete set-design-option inverse mutations.
use super::SetDesignOption;
use crate::*;
pub fn inverse(payload: &SetDesignOption, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.design_options.get(&payload.id) else { return Vec::new(); };
    let patch = payload.patch().minimal(record).restoring(record);
    if patch.is_empty() { return Vec::new(); }
    vec![ModelMutation::SetDesignOption(SetDesignOption::from_patch(payload.id.clone(), patch))]
}
