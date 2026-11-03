//! ↩️ `remove-concrete-grade` inverse: inserts the removed row back at its position.

use crate::remove_concrete_grade::RemoveConcreteGrade;
use crate::insert_concrete_grade::InsertConcreteGrade;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &RemoveConcreteGrade, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_concrete_grade::InsertConcreteGrade> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveConcreteGrade, base: &En1992Snapshot) -> Option<InsertConcreteGrade> {

    let index = payload.index;
    let row = base.concrete_grades.get(index)?;
    Some(InsertConcreteGrade { index, concrete_grade: row.clone() })
}
