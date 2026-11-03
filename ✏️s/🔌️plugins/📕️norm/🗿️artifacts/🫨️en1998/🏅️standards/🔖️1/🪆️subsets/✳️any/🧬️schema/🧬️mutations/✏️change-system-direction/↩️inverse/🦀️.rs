//! ↩️ `change-system-direction` inverse: restores the previous `direction`.

use crate::change_system_direction::ChangeSystemDirection;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSystemDirection, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_system_direction::ChangeSystemDirection> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSystemDirection, base: &En1998Snapshot) -> Option<ChangeSystemDirection> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.systems.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSystemDirection { building_id: payload.building_id.clone(), id: payload.id.clone(), new_direction: r1.direction })
}
