//! ↩️ Concrete set-element-workset inverse mutations.
use super::SetElementWorkset;
use crate::*;
pub fn inverse(payload: &SetElementWorkset, base: &ModelSnapshot) -> Vec<ModelMutation> {
    vec![ModelMutation::SetElementWorkset(SetElementWorkset { id: payload.id.clone(), workset: base.element_worksets.get(&payload.id).map(|membership| membership.target.clone()) })]
}
