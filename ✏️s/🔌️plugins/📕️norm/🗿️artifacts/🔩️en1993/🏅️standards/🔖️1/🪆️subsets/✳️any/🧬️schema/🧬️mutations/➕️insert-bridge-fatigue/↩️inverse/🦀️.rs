use super::InsertBridgeFatigue;
use crate::mutations::{remove_bridge_fatigue, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &InsertBridgeFatigue, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    let at = payload.index.min(base.bridge_fatigue.len());
    vec![En1993Mutation::RemoveBridgeFatigue(remove_bridge_fatigue::RemoveBridgeFatigue { index: at })]
}
