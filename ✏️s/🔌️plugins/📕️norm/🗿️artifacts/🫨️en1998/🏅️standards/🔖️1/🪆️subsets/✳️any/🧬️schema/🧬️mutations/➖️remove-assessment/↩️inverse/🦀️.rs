//! ↩️ `remove-assessment` inverse: inserts the removed row back at its position.

use crate::remove_assessment::RemoveAssessment;
use crate::insert_assessment::InsertAssessment;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &RemoveAssessment, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_assessment::InsertAssessment> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveAssessment, base: &En1998Snapshot) -> Option<InsertAssessment> {
    let (index, row) = base.assessments.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertAssessment { index: Some(index), assessment: row.clone() })
}
