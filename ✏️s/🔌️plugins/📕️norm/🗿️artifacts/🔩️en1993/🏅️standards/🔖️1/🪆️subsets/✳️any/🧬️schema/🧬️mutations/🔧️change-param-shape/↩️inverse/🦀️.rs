//! ↩️ `change-param-shape` inverse: restores the previous `shape`.

use crate::change_param_shape::ChangeParamShape;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeParamShape, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_param_shape::ChangeParamShape> {
    Ok(match base.parametric_sections.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeParamShape { id: payload.id.clone(), new_shape: row.shape })],
        None => Vec::new(),
    })
}
