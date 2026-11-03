//! ↩️ `change-param-flange-thickness` inverse: restores the previous `flange_thickness`.

use crate::change_param_flange_thickness::ChangeParamFlangeThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeParamFlangeThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_param_flange_thickness::ChangeParamFlangeThickness> {
    Ok(match base.parametric_sections.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeParamFlangeThickness { id: payload.id.clone(), new_flange_thickness: row.flange_thickness })],
        None => Vec::new(),
    })
}
