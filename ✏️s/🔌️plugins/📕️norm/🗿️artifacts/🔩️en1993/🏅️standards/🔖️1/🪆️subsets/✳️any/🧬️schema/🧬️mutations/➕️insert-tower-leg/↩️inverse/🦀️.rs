use super::InsertTowerLeg;
use crate::mutations::{remove_tower_leg, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &InsertTowerLeg, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    let at = payload.index.min(base.tower_legs.len());
    vec![En1993Mutation::RemoveTowerLeg(remove_tower_leg::RemoveTowerLeg { index: at })]
}
