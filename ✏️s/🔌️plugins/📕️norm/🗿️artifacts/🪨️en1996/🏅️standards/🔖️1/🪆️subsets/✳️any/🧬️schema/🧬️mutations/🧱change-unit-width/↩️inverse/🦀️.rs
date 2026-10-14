use crate::change_unit_width::ChangeUnitWidth;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeUnitWidth, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_unit_width::ChangeUnitWidth> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![Op::from(ChangeUnitWidth { index: payload.index, new_unit_width_m: base.walls[payload.index].unit_width_m })] }

    })())
}
