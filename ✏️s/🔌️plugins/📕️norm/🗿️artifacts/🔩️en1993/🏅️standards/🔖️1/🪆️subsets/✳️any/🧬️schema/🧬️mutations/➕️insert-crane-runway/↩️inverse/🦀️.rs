use super::InsertCraneRunway;
use crate::mutations::{remove_crane_runway, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &InsertCraneRunway, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    let at = payload.index.min(base.crane_runways.len());
    vec![En1993Mutation::RemoveCraneRunway(remove_crane_runway::RemoveCraneRunway { index: at })]
}
