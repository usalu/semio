//! ↩️ `change-assessment-diag-reinf-ratio` inverse: restores the previous `diagonal_reinforcement_ratio`.

use crate::change_assessment_diag_reinf_ratio::ChangeAssessmentDiagReinfRatio;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentDiagReinfRatio, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_diag_reinf_ratio::ChangeAssessmentDiagReinfRatio> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentDiagReinfRatio, base: &En1998Snapshot) -> Option<ChangeAssessmentDiagReinfRatio> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentDiagReinfRatio { id: payload.id.clone(), new_diagonal_reinforcement_ratio: r0.diagonal_reinforcement_ratio })
}
