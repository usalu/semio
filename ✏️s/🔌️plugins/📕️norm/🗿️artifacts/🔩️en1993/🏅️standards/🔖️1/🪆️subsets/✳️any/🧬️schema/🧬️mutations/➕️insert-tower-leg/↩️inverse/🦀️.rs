use crate::insert_tower_leg::InsertTowerLeg;
use crate::remove_tower_leg;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertTowerLeg, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_tower_leg::RemoveTowerLeg> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.tower_legs.len());
    vec![Op::from(remove_tower_leg::RemoveTowerLeg { index: at })]

    })())
}
