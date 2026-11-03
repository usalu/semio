//! ↩️ `change-plated-alpha-cr` inverse: restores the previous `alpha_cr`.

use crate::change_plated_alpha_cr::ChangePlatedAlphaCr;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedAlphaCr, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_alpha_cr::ChangePlatedAlphaCr> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedAlphaCr { id: payload.id.clone(), new_alpha_cr: row.alpha_cr })],
        None => Vec::new(),
    })
}
