use crate::change_unit_group::ChangeUnitGroup;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeUnitGroup, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_unit_group::ChangeUnitGroup> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![Op::from(ChangeUnitGroup { index: payload.index, new_unit_group: base.walls[payload.index].unit_group })] }

    })())
}
