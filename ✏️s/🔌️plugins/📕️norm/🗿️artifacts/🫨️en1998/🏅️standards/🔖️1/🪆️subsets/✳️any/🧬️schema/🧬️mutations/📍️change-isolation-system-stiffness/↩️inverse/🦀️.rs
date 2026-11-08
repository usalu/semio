//! ↩️ `change-isolation-system-stiffness` inverse: restores the previous `stiffness_n_per_m`.

use crate::change_isolation_system_stiffness::ChangeIsolationSystemStiffnessNPerM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeIsolationSystemStiffnessNPerM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_isolation_system_stiffness::ChangeIsolationSystemStiffnessNPerM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeIsolationSystemStiffnessNPerM, base: &En1998Snapshot) -> Option<ChangeIsolationSystemStiffnessNPerM> {
    let r0 = base.isolation_systems.iter().find(|row| row.id == payload.id)?;
    Some(ChangeIsolationSystemStiffnessNPerM { id: payload.id.clone(), new_stiffness_n_per_m: r0.stiffness_n_per_m })
}
