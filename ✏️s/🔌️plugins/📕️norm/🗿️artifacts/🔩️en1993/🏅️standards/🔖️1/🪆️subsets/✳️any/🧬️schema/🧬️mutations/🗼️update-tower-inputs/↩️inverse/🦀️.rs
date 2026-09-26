//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use super::UpdateTowerInputs;
use crate::mutations::remove_tower_leg;
use crate::{En1993Mutation, En1993Snapshot};
pub fn inverse(payload: &UpdateTowerInputs, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    if let Some(prior) = base.tower_legs.iter().find(|x| x.id == payload.tower_leg.id) {
        vec![En1993Mutation::UpdateTowerInputs(UpdateTowerInputs { tower_leg: prior.clone() })]
    } else {
        vec![En1993Mutation::RemoveTowerLeg(remove_tower_leg::RemoveTowerLeg { index: base.tower_legs.len() })]
    }
}
