//! ↩️ `change-reinforcement-k` inverse: restores the previous `k`.

use crate::change_reinforcement_k::ChangeReinforcementK;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeReinforcementK, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_reinforcement_k::ChangeReinforcementK> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeReinforcementK, base: &En1992Snapshot) -> Option<ChangeReinforcementK> {
    let r0 = base.reinforcement_grades.iter().find(|row| row.id == payload.grade_id)?;
    Some(ChangeReinforcementK { grade_id: payload.grade_id.clone(), new_k: r0.k })
}
