use super::InsertFireExposure;
use crate::mutations::{remove_fire_exposure, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &InsertFireExposure, base: &En1993Snapshot) -> Vec<En1993Mutation> {
    let at = payload.index.min(base.fire_exposures.len());
    vec![En1993Mutation::RemoveFireExposure(remove_fire_exposure::RemoveFireExposure { index: at })]
}
