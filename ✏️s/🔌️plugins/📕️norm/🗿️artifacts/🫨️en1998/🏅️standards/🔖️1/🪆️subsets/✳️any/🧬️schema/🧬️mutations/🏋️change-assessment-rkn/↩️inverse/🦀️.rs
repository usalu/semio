//! ↩️ `change-assessment-rkn` inverse: restores the previous `r_k_n`.

use crate::change_assessment_rkn::ChangeAssessmentRKN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentRKN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_rkn::ChangeAssessmentRKN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeAssessmentRKN, base: &En1998Snapshot) -> Option<ChangeAssessmentRKN> {
    let r0 = base.assessments.iter().find(|row| row.id == payload.id)?;
    Some(ChangeAssessmentRKN { id: payload.id.clone(), new_r_k_n: r0.r_k_n })
}
