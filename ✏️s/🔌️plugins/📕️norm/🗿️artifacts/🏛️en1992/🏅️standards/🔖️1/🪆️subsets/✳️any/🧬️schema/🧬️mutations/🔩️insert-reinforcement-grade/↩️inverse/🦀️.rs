//! ↩️ `insert-reinforcement-grade` inverse: removes the inserted row again.

use crate::insert_reinforcement_grade::InsertReinforcementGrade;
use crate::remove_reinforcement_grade::RemoveReinforcementGrade;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &InsertReinforcementGrade, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_reinforcement_grade::RemoveReinforcementGrade> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertReinforcementGrade, base: &En1992Snapshot) -> Option<RemoveReinforcementGrade> {
    let index = payload.index;
    Some(RemoveReinforcementGrade { index })
}
