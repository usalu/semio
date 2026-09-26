//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use super::UpdateTensionComponentInputs;
use crate::mutations::remove_tension_component;
use crate::{En1993Mutation, En1993Snapshot};
pub fn inverse(payload: &UpdateTensionComponentInputs, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    if let Some(prior) = base.tension_components.iter().find(|x| x.id == payload.tension_component.id) {
        vec![En1993Mutation::UpdateTensionComponentInputs(UpdateTensionComponentInputs { tension_component: prior.clone() })]
    } else {
        vec![En1993Mutation::RemoveTensionComponent(remove_tension_component::RemoveTensionComponent { index: base.tension_components.len() })]
    }
}
