//! ↩️ `change-assessment-limit-state` inverse: restores the previous `limit_state`.

use crate::change_assessment_limit_state::ChangeAssessmentLimitState;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentLimitState, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_limit_state::ChangeAssessmentLimitState> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentLimitState, base: &En1998Snapshot) -> Option<ChangeAssessmentLimitState> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentLimitState { id: payload.id.clone(), new_limit_state: r0.limit_state })
}
