//! ↩️ `change-unit-material` inverse: restores the previous `unit_material`.

use crate::change_unit_material::ChangeUnitMaterial;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;

pub fn inverse<Op>(payload: &ChangeUnitMaterial, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_unit_material::ChangeUnitMaterial> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeUnitMaterial, base: &En1996Snapshot) -> Option<ChangeUnitMaterial> {
    let r0 = base.walls.iter().find(|row| row.id == payload.id)?;
    Some(ChangeUnitMaterial { id: payload.id.clone(), new_unit_material: r0.unit_material.clone() })
}
