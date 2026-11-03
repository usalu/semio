//! ↩️ `change-assessment-steel-yield-mpa` inverse: restores the previous `steel_yield_mpa`.

use crate::change_assessment_steel_yield_mpa::ChangeAssessmentSteelYieldMpa;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentSteelYieldMpa, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_steel_yield_mpa::ChangeAssessmentSteelYieldMpa> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentSteelYieldMpa, base: &En1998Snapshot) -> Option<ChangeAssessmentSteelYieldMpa> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentSteelYieldMpa { id: payload.id.clone(), new_steel_yield_mpa: r0.steel_yield_mpa })
}
