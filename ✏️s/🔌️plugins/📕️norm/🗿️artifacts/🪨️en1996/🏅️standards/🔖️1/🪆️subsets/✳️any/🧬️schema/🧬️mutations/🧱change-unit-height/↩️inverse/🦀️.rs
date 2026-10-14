use crate::change_unit_height::ChangeUnitHeight;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeUnitHeight, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_unit_height::ChangeUnitHeight> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![Op::from(ChangeUnitHeight { index: payload.index, new_unit_height_m: base.walls[payload.index].unit_height_m })] }

    })())
}
