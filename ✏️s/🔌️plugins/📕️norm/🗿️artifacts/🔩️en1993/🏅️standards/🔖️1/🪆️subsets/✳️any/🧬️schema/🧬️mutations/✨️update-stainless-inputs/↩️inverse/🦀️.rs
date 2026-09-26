//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use super::UpdateStainlessInputs;
use crate::mutations::remove_material;
use crate::{En1993Mutation, En1993Snapshot};
pub fn inverse(payload: &UpdateStainlessInputs, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    if let Some(prior) = base.materials.iter().find(|x| x.id == payload.material.id) {
        vec![En1993Mutation::UpdateStainlessInputs(UpdateStainlessInputs { material: prior.clone() })]
    } else {
        vec![En1993Mutation::RemoveMaterial(remove_material::RemoveMaterial { index: base.materials.len() })]
    }
}
