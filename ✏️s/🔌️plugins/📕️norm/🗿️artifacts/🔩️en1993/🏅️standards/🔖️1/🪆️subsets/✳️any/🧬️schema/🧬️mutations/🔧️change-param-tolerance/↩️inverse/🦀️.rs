//! ↩️ `change-param-tolerance` inverse: restores the previous `tolerance`.

use crate::change_param_tolerance::ChangeParamTolerance;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeParamTolerance, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_param_tolerance::ChangeParamTolerance> {
    Ok(match base.parametric_sections.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeParamTolerance { id: payload.id.clone(), new_tolerance: row.tolerance })],
        None => Vec::new(),
    })
}
