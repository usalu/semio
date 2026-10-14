use crate::change_as_horizontal::ChangeAsHorizontal;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeAsHorizontal, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_as_horizontal::ChangeAsHorizontal> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![Op::from(ChangeAsHorizontal { index: payload.index, new_as_horizontal_m2: base.walls[payload.index].as_horizontal_m2 })] }

    })())
}
