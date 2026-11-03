//! ↩️ `change-param-width` inverse: restores the previous `width`.

use crate::change_param_width::ChangeParamWidth;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeParamWidth, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_param_width::ChangeParamWidth> {
    Ok(match base.parametric_sections.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeParamWidth { id: payload.id.clone(), new_width: row.width })],
        None => Vec::new(),
    })
}
