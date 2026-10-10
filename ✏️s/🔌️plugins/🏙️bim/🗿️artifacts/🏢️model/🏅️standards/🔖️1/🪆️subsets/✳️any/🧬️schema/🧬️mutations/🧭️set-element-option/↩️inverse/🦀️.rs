//! ↩️ Concrete set-element-option inverse mutations.
use super::SetElementOption;
use crate::*;
pub fn inverse(payload: &SetElementOption, base: &ModelSnapshot) -> Vec<ModelMutation> {
    vec![ModelMutation::SetElementOption(SetElementOption { id: payload.id.clone(), option: base.element_options.get(&payload.id).map(|membership| membership.target.clone()) })]
}
