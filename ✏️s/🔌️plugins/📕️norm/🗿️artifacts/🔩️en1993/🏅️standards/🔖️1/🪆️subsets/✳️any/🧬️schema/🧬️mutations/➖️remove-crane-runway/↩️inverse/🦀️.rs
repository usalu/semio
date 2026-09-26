use super::RemoveCraneRunway;
use crate::mutations::{insert_crane_runway, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &RemoveCraneRunway, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    if payload.index >= base.crane_runways.len() { return Vec::new(); }
    vec![En1993Mutation::InsertCraneRunway(insert_crane_runway::InsertCraneRunway { index: payload.index, crane_runway: base.crane_runways[payload.index].clone() })]
}
