use crate::change_unit_material::ChangeUnitMaterial;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeUnitMaterial, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_unit_material::ChangeUnitMaterial> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![Op::from(ChangeUnitMaterial { index: payload.index, new_unit_material: base.walls[payload.index].unit_material })] }

    })())
}
