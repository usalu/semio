//! ↩️ `change-assessment-fc-mpa` inverse: restores the previous `concrete_strength_mpa`.

use crate::change_assessment_fc_mpa::ChangeAssessmentFcMpa;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentFcMpa, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_fc_mpa::ChangeAssessmentFcMpa> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentFcMpa, base: &En1998Snapshot) -> Option<ChangeAssessmentFcMpa> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentFcMpa { id: payload.id.clone(), new_concrete_strength_mpa: r0.concrete_strength_mpa })
}
