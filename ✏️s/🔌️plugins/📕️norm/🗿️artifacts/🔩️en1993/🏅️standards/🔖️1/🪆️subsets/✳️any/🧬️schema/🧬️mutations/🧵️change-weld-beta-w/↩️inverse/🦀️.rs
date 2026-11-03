//! ↩️ `change-weld-beta-w` inverse: restores the previous `beta_w`.

use crate::change_weld_beta_w::ChangeWeldBetaW;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeWeldBetaW, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_weld_beta_w::ChangeWeldBetaW> {
    Ok(match base.weld_groups.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeWeldBetaW { id: payload.id.clone(), new_beta_w: row.beta_w })],
        None => Vec::new(),
    })
}
