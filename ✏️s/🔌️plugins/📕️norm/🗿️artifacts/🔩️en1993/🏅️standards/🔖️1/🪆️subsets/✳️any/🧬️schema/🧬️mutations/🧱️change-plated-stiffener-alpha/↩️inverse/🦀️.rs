//! ↩️ `change-plated-stiffener-alpha` inverse: restores the previous `stiffener_alpha`.

use crate::change_plated_stiffener_alpha::ChangePlatedStiffenerAlpha;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedStiffenerAlpha, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_stiffener_alpha::ChangePlatedStiffenerAlpha> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedStiffenerAlpha { id: payload.id.clone(), new_stiffener_alpha: row.stiffener_alpha })],
        None => Vec::new(),
    })
}
