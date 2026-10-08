use super::InsertTowerLeg;
use crate::mutations::{remove_tower_leg, En1993Mutation};
use crate::En1993Snapshot;
pub fn inverse(payload: &InsertTowerLeg, base: &En1993Snapshot) -> Result<Vec<En1993Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.tower_legs.len());
    vec![En1993Mutation::RemoveTowerLeg(remove_tower_leg::RemoveTowerLeg { index: at })]

    })())
}
