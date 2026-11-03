//! ↩️ `change-tower-q-nominal` inverse: restores the previous `q_nominal`.

use crate::change_tower_q_nominal::ChangeTowerQNominal;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerQNominal, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_q_nominal::ChangeTowerQNominal> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTowerQNominal, base: &En1998Snapshot) -> Option<ChangeTowerQNominal> {
    let r0 = base.towers.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTowerQNominal { id: payload.id.clone(), new_q_nominal: r0.q_nominal })
}
