//! ↩️ `change-system-system-type` inverse: restores the previous `system_type`.

use crate::change_system_system_type::ChangeSystemSystemType;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSystemSystemType, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_system_system_type::ChangeSystemSystemType> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSystemSystemType, base: &En1998Snapshot) -> Option<ChangeSystemSystemType> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.systems.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSystemSystemType { building_id: payload.building_id.clone(), id: payload.id.clone(), new_system_type: r1.system_type })
}
