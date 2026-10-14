use crate::remove_tower_leg::RemoveTowerLeg;
use crate::insert_tower_leg;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveTowerLeg, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_tower_leg::InsertTowerLeg> {
    Ok((|| {
    if payload.index >= base.tower_legs.len() { return Vec::new(); }
    vec![Op::from(insert_tower_leg::InsertTowerLeg { index: Some(payload.index), tower_leg: base.tower_legs[payload.index].clone() })]

    })())
}
