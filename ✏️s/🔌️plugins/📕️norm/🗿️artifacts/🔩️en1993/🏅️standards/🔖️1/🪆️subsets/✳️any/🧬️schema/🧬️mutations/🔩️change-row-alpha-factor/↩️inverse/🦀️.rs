//! ↩️ `change-row-alpha-factor` inverse: restores the previous `alpha_factor`.

use crate::change_row_alpha_factor::ChangeRowAlphaFactor;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeRowAlphaFactor, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_row_alpha_factor::ChangeRowAlphaFactor> {
    Ok(match base.joint_rows.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeRowAlphaFactor { id: payload.id.clone(), new_alpha_factor: row.alpha_factor })],
        None => Vec::new(),
    })
}
