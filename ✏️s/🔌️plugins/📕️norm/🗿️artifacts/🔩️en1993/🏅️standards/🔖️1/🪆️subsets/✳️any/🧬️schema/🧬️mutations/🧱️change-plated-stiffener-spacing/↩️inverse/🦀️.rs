//! ↩️ `change-plated-stiffener-spacing` inverse: restores the previous `stiffener_spacing`.

use crate::change_plated_stiffener_spacing::ChangePlatedStiffenerSpacing;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedStiffenerSpacing, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_stiffener_spacing::ChangePlatedStiffenerSpacing> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedStiffenerSpacing { id: payload.id.clone(), new_stiffener_spacing: row.stiffener_spacing })],
        None => Vec::new(),
    })
}
