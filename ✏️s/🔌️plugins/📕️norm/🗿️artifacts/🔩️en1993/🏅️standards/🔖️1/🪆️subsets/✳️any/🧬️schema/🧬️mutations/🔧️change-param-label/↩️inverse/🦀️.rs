//! ↩️ `change-param-label` inverse: restores the previous `label`.

use crate::change_param_label::ChangeParamLabel;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeParamLabel, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_param_label::ChangeParamLabel> {
    Ok(match base.parametric_sections.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeParamLabel { id: payload.id.clone(), new_label: row.label.clone() })],
        None => Vec::new(),
    })
}
