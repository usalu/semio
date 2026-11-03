//! ↩️ `change-system-q0` inverse: restores the previous `q0`.

use crate::change_system_q0::ChangeSystemQ0;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSystemQ0, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_system_q0::ChangeSystemQ0> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSystemQ0, base: &En1998Snapshot) -> Option<ChangeSystemQ0> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.systems.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSystemQ0 { building_id: payload.building_id.clone(), id: payload.id.clone(), new_q0: r1.q0 })
}
