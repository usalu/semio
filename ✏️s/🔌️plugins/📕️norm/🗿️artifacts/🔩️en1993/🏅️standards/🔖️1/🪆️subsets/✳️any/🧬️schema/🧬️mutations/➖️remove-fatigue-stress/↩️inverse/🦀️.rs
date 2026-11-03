use crate::remove_fatigue_stress::RemoveFatigueStress;
use crate::insert_fatigue_stress;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveFatigueStress, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_fatigue_stress::InsertFatigueStress> {
    Ok((|| {
    if payload.index >= base.fatigue_stresses.len() { return Vec::new(); }
    vec![Op::from(insert_fatigue_stress::InsertFatigueStress { index: Some(payload.index), fatigue_stress: base.fatigue_stresses[payload.index].clone() })]

    })())
}
