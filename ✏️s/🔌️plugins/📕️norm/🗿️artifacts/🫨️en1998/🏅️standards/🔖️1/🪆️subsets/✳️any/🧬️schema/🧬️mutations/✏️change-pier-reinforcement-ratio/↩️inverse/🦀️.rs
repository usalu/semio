//! ↩️ `change-pier-reinforcement-ratio` inverse: restores the previous `reinforcement_ratio`.

use crate::change_pier_reinforcement_ratio::ChangePierReinforcementRatio;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangePierReinforcementRatio, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pier_reinforcement_ratio::ChangePierReinforcementRatio> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangePierReinforcementRatio, base: &En1998Snapshot) -> Option<ChangePierReinforcementRatio> {
    let r0 = base.piers.iter().find(|row| row.id == payload.id)?;
    Some(ChangePierReinforcementRatio { id: payload.id.clone(), new_reinforcement_ratio: r0.reinforcement_ratio })
}
