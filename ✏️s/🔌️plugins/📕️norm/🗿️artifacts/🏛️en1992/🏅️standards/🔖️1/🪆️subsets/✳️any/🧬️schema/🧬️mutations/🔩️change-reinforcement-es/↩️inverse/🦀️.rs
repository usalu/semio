//! ↩️ `change-reinforcement-es` inverse: restores the previous `e_s`.

use crate::change_reinforcement_e_s::ChangeReinforcementES;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeReinforcementES, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_reinforcement_e_s::ChangeReinforcementES> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeReinforcementES, base: &En1992Snapshot) -> Option<ChangeReinforcementES> {
    let r0 = base.reinforcement_grades.iter().find(|row| row.id == payload.grade_id)?;
    Some(ChangeReinforcementES { grade_id: payload.grade_id.clone(), new_e_s: r0.e_s })
}
