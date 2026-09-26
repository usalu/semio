//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use super::UpdateCraneInputs;
use crate::mutations::remove_crane_runway;
use crate::{En1993Mutation, En1993Snapshot};
pub fn inverse(payload: &UpdateCraneInputs, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    if let Some(prior) = base.crane_runways.iter().find(|x| x.id == payload.crane_runway.id) {
        vec![En1993Mutation::UpdateCraneInputs(UpdateCraneInputs { crane_runway: prior.clone() })]
    } else {
        vec![En1993Mutation::RemoveCraneRunway(remove_crane_runway::RemoveCraneRunway { index: base.crane_runways.len() })]
    }
}
