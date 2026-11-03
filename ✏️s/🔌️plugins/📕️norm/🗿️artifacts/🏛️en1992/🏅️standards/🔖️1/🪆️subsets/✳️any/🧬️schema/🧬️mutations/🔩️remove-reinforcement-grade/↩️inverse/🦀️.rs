//! ↩️ `remove-reinforcement-grade` inverse: inserts the removed row back at its position.

use crate::remove_reinforcement_grade::RemoveReinforcementGrade;
use crate::insert_reinforcement_grade::InsertReinforcementGrade;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &RemoveReinforcementGrade, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_reinforcement_grade::InsertReinforcementGrade> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveReinforcementGrade, base: &En1992Snapshot) -> Option<InsertReinforcementGrade> {

    let index = payload.index;
    let row = base.reinforcement_grades.get(index)?;
    Some(InsertReinforcementGrade { index, reinforcement_grade: row.clone() })
}
