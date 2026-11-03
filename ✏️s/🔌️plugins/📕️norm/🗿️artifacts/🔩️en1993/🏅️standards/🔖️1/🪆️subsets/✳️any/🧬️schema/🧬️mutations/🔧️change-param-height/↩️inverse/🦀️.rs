//! ↩️ `change-param-height` inverse: restores the previous `height`.

use crate::change_param_height::ChangeParamHeight;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeParamHeight, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_param_height::ChangeParamHeight> {
    Ok(match base.parametric_sections.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeParamHeight { id: payload.id.clone(), new_height: row.height })],
        None => Vec::new(),
    })
}
