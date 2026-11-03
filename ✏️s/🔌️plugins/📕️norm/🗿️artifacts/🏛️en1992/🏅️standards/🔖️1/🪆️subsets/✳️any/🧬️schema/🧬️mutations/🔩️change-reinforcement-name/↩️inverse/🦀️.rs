//! ↩️ `change-reinforcement-name` inverse: restores the previous `name`.

use crate::change_reinforcement_name::ChangeReinforcementName;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeReinforcementName, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_reinforcement_name::ChangeReinforcementName> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeReinforcementName, base: &En1992Snapshot) -> Option<ChangeReinforcementName> {
    let r0 = base.reinforcement_grades.iter().find(|row| row.id == payload.grade_id)?;
    Some(ChangeReinforcementName { grade_id: payload.grade_id.clone(), new_name: r0.name.clone() })
}
