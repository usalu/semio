//! ↩️ `change-system-material` inverse: restores the previous `material`.

use crate::change_system_material::ChangeSystemMaterial;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSystemMaterial, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_system_material::ChangeSystemMaterial> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeSystemMaterial, base: &En1998Snapshot) -> Option<ChangeSystemMaterial> {
    let r0 = base.buildings.iter().find(|row| row.id == payload.building_id)?;
    let r1 = r0.systems.iter().find(|row| row.id == payload.id)?;
    Some(ChangeSystemMaterial { building_id: payload.building_id.clone(), id: payload.id.clone(), new_material: r1.material })
}
