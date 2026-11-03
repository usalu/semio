//! ↩️ `insert-concrete-grade` inverse: removes the inserted row again.

use crate::insert_concrete_grade::InsertConcreteGrade;
use crate::remove_concrete_grade::RemoveConcreteGrade;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &InsertConcreteGrade, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_concrete_grade::RemoveConcreteGrade> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertConcreteGrade, base: &En1992Snapshot) -> Option<RemoveConcreteGrade> {
    let index = payload.index;
    Some(RemoveConcreteGrade { index })
}
