//! Inverse for `change-assessment-r-k-n`.
use crate::change_assessment_rkn::ChangeAssessmentRKN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeAssessmentRKN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_assessment_rkn::ChangeAssessmentRKN> {
    Ok((|| {
    match base.assessments.get(payload.index) {
        Some(a) => vec![Op::from(ChangeAssessmentRKN { index: payload.index, new_r_k_n: a.r_k_n })],
        None => Vec::new(),
    }

    })())
}
