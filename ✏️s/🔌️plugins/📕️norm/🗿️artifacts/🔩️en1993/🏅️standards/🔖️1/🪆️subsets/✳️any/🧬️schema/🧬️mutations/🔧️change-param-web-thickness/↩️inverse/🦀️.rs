//! ↩️ `change-param-web-thickness` inverse: restores the previous `web_thickness`.

use crate::change_param_web_thickness::ChangeParamWebThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeParamWebThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_param_web_thickness::ChangeParamWebThickness> {
    Ok(match base.parametric_sections.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeParamWebThickness { id: payload.id.clone(), new_web_thickness: row.web_thickness })],
        None => Vec::new(),
    })
}
