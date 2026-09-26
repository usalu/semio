//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use super::UpdateFireInputs;
use crate::mutations::remove_fire_exposure;
use crate::{En1993Mutation, En1993Snapshot};
pub fn inverse(payload: &UpdateFireInputs, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    if let Some(prior) = base.fire_exposures.iter().find(|x| x.id == payload.fire_exposure.id) {
        vec![En1993Mutation::UpdateFireInputs(UpdateFireInputs { fire_exposure: prior.clone() })]
    } else {
        vec![En1993Mutation::RemoveFireExposure(remove_fire_exposure::RemoveFireExposure { index: base.fire_exposures.len() })]
    }
}
