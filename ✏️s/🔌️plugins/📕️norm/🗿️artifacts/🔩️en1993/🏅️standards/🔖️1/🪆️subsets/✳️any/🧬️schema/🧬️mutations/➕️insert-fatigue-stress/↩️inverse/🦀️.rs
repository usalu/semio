use crate::insert_fatigue_stress::InsertFatigueStress;
use crate::remove_fatigue_stress;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertFatigueStress, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_fatigue_stress::RemoveFatigueStress> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.fatigue_stresses.len());
    vec![Op::from(remove_fatigue_stress::RemoveFatigueStress { index: at })]

    })())
}
