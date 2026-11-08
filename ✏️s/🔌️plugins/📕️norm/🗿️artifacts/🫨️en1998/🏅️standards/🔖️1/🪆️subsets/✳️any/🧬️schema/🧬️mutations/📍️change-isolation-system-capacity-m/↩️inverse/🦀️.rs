//! ↩️ `change-isolation-system-capacity-m` inverse: restores the previous `capacity_m`.

use crate::change_isolation_system_capacity_m::ChangeIsolationSystemCapacityM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeIsolationSystemCapacityM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_isolation_system_capacity_m::ChangeIsolationSystemCapacityM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeIsolationSystemCapacityM, base: &En1998Snapshot) -> Option<ChangeIsolationSystemCapacityM> {
    let r0 = base.isolation_systems.iter().find(|row| row.id == payload.id)?;
    Some(ChangeIsolationSystemCapacityM { id: payload.id.clone(), new_capacity_m: r0.capacity_m })
}
