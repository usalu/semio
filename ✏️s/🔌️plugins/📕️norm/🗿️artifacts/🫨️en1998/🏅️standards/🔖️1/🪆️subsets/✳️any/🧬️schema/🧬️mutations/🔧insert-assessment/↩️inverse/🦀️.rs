//! ↩️ `insert-assessment` inverse: removes the inserted row again.

use crate::insert_assessment::InsertAssessment;
use crate::remove_assessment::RemoveAssessment;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &InsertAssessment, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_assessment::RemoveAssessment> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertAssessment, base: &En1998Snapshot) -> Option<RemoveAssessment> {
    Some(RemoveAssessment { id: payload.assessment.id.clone() })
}
