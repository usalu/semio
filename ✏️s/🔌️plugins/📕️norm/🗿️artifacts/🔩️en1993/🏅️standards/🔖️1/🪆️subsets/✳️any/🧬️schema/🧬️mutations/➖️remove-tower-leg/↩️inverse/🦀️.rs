use super::RemoveTowerLeg;
use crate::mutations::{insert_tower_leg, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &RemoveTowerLeg, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    if payload.index >= base.tower_legs.len() { return Vec::new(); }
    vec![En1993Mutation::InsertTowerLeg(insert_tower_leg::InsertTowerLeg { index: payload.index, tower_leg: base.tower_legs[payload.index].clone() })]
}
