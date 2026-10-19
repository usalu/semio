//! ↩️ Inverse for `insert-assessment`.
use crate::insert_assessment::InsertAssessment;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;
use crate::remove_assessment;

pub fn inverse<Op>(payload: &InsertAssessment, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_assessment::RemoveAssessment> {
    Ok((|| {
    vec![Op::from(remove_assessment::RemoveAssessment { index: payload.index.unwrap_or(usize::MAX).min(base.assessments.len()) })]

    })())
}
