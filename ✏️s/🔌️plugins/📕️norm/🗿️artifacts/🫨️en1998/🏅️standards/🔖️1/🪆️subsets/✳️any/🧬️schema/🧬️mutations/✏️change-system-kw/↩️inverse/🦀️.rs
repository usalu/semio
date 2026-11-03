//! ↩️ `change-system-kw` inverse: restores the previous `k_w`.

use crate::change_system_k_w::ChangeSystemKW;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSystemKW, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_system_k_w::ChangeSystemKW> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSystemKW, base: &En1998Snapshot) -> Option<ChangeSystemKW> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.systems.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSystemKW { building_id: payload.building_id.clone(), id: payload.id.clone(), new_k_w: r1.k_w })
}
