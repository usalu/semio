//! ↩️ `change-isolation-system-yield-disp-m` inverse: restores the previous `yield_displacement_m`.

use crate::change_isolation_system_yield_disp_m::ChangeIsolationSystemYieldDisplacementM;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeIsolationSystemYieldDisplacementM, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_isolation_system_yield_disp_m::ChangeIsolationSystemYieldDisplacementM> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeIsolationSystemYieldDisplacementM, base: &En1998Snapshot) -> Option<ChangeIsolationSystemYieldDisplacementM> {
    let r0 = base.isolation_systems.iter().find(|row| row.id == payload.id)?;
    Some(ChangeIsolationSystemYieldDisplacementM { id: payload.id.clone(), new_yield_displacement_m: r0.yield_displacement_m })
}
