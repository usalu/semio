//! ↩️ upsert inverse — restore prior entity or remove inserted one.
use super::UpdateBridgeInputs;
use crate::mutations::remove_bridge_fatigue;
use crate::{En1993Mutation, En1993Snapshot};
pub fn inverse(payload: &UpdateBridgeInputs, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    if let Some(prior) = base.bridge_fatigue.iter().find(|x| x.id == payload.bridge_fatigue_item.id) {
        vec![En1993Mutation::UpdateBridgeInputs(UpdateBridgeInputs { bridge_fatigue_item: prior.clone() })]
    } else {
        vec![En1993Mutation::RemoveBridgeFatigue(remove_bridge_fatigue::RemoveBridgeFatigue { index: base.bridge_fatigue.len() })]
    }
}
