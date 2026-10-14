use crate::change_is_basement::ChangeIsBasement;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeIsBasement, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_is_basement::ChangeIsBasement> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else {
        vec![Op::from(ChangeIsBasement { index: payload.index, new_is_basement: base.walls[payload.index].is_basement })]
    }

    })())
}
