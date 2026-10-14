use crate::change_unit_length::ChangeUnitLength;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeUnitLength, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_unit_length::ChangeUnitLength> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![Op::from(ChangeUnitLength { index: payload.index, new_unit_length_m: base.walls[payload.index].unit_length_m })] }

    })())
}
