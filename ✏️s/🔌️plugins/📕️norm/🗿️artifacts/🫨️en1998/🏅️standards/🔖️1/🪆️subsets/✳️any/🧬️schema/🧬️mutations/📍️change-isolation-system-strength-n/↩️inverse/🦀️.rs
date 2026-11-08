//! ↩️ `change-isolation-system-strength-n` inverse: restores the previous `strength_n`.

use crate::change_isolation_system_strength_n::ChangeIsolationSystemStrengthN;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeIsolationSystemStrengthN, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_isolation_system_strength_n::ChangeIsolationSystemStrengthN> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeIsolationSystemStrengthN, base: &En1998Snapshot) -> Option<ChangeIsolationSystemStrengthN> {
    let r0 = base.isolation_systems.iter().find(|row| row.id == payload.id)?;
    Some(ChangeIsolationSystemStrengthN { id: payload.id.clone(), new_strength_n: r0.strength_n })
}
