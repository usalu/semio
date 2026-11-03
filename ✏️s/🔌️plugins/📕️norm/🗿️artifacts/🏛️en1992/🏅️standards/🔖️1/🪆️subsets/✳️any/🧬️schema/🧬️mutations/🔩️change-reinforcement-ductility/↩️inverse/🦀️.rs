//! ↩️ `change-reinforcement-ductility` inverse: restores the previous `ductility`.

use crate::change_reinforcement_ductility::ChangeReinforcementDuctility;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(payload: &ChangeReinforcementDuctility, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_reinforcement_ductility::ChangeReinforcementDuctility> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeReinforcementDuctility, base: &En1992Snapshot) -> Option<ChangeReinforcementDuctility> {
    let r0 = base.reinforcement_grades.iter().find(|row| row.id == payload.grade_id)?;
    Some(ChangeReinforcementDuctility { grade_id: payload.grade_id.clone(), new_ductility: r0.ductility })
}
