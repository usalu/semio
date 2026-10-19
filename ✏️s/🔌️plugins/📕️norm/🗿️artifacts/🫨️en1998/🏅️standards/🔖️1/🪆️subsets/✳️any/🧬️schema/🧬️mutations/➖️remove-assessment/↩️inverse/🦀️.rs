//! ↩️ Inverse for `remove-assessment`.
use crate::remove_assessment::RemoveAssessment;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;
use crate::insert_assessment;

pub fn inverse<Op>(payload: &RemoveAssessment, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_assessment::InsertAssessment> {
    Ok((|| {
    match base.assessments.get(payload.index) {
        Some(item) => vec![Op::from(insert_assessment::InsertAssessment { index: Some(payload.index), assessment: item.clone() })],
        None => Vec::new(),
    }

    })())
}
