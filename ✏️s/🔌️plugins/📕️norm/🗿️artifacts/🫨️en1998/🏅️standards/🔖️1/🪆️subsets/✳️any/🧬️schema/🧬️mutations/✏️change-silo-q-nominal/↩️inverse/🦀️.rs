//! ↩️ `change-silo-q-nominal` inverse: restores the previous `q_nominal`.

use crate::change_silo_q_nominal::ChangeSiloQNominal;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloQNominal, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_q_nominal::ChangeSiloQNominal> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSiloQNominal, base: &En1998Snapshot) -> Option<ChangeSiloQNominal> {
    let r0 = base.silos.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSiloQNominal { id: payload.id.clone(), new_q_nominal: r0.q_nominal })
}
