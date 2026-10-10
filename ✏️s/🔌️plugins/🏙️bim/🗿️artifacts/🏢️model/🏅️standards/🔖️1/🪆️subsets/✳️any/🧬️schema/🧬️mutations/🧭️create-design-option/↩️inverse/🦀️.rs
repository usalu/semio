//! ↩️ Concrete create-design-option inverse mutations.
use super::CreateDesignOption;
use crate::*;
pub fn inverse(payload: &CreateDesignOption, base: &ModelSnapshot) -> Vec<ModelMutation> {
    vec![ModelMutation::DeleteDesignOption(super::super::delete_design_option::DeleteDesignOption { id: payload.id.clone() })]
}
