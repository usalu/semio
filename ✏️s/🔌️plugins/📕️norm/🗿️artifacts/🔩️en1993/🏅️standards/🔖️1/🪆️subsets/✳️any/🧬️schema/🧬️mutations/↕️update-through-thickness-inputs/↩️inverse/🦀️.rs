//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use super::UpdateThroughThicknessInputs;
use crate::mutations::remove_section;
use crate::{En1993Mutation, En1993Snapshot};
pub fn inverse(payload: &UpdateThroughThicknessInputs, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    if let Some(prior) = base.sections.iter().find(|x| x.id == payload.section.id) {
        vec![En1993Mutation::UpdateThroughThicknessInputs(UpdateThroughThicknessInputs { section: prior.clone() })]
    } else {
        vec![En1993Mutation::RemoveSection(remove_section::RemoveSection { index: base.sections.len() })]
    }
}
