use crate::change_exposure::ChangeExposure;

use semio_s_artifact_norm_en1996_model::En1996Snapshot;
pub fn inverse<Op>(payload: &ChangeExposure, base: &En1996Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_exposure::ChangeExposure> {
    Ok((|| {
    if payload.index >= base.walls.len() { Vec::new() } else { vec![Op::from(ChangeExposure { index: payload.index, new_exposure: base.walls[payload.index].exposure })] }

    })())
}
