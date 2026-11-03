//! ↩️ `change-reinforcement-eps-uk` inverse: restores the previous `eps_uk`.

use crate::change_reinforcement_eps_uk::ChangeReinforcementEpsUk;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeReinforcementEpsUk, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_reinforcement_eps_uk::ChangeReinforcementEpsUk> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeReinforcementEpsUk, base: &En1992Snapshot) -> Option<ChangeReinforcementEpsUk> {
    let r0 = base.reinforcement_grades.iter().find(|row| row.id == payload.grade_id)?;
    Some(ChangeReinforcementEpsUk { grade_id: payload.grade_id.clone(), new_eps_uk: r0.eps_uk })
}
