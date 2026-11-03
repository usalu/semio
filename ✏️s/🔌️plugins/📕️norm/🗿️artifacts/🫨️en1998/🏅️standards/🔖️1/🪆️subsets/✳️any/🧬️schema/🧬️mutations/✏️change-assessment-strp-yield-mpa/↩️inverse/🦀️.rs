//! ↩️ `change-assessment-strp-yield-mpa` inverse: restores the previous `stirrup_yield_mpa`.

use crate::change_assessment_strp_yield_mpa::ChangeAssessmentStrpYieldMpa;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentStrpYieldMpa, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_strp_yield_mpa::ChangeAssessmentStrpYieldMpa> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentStrpYieldMpa, base: &En1998Snapshot) -> Option<ChangeAssessmentStrpYieldMpa> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentStrpYieldMpa { id: payload.id.clone(), new_stirrup_yield_mpa: r0.stirrup_yield_mpa })
}
