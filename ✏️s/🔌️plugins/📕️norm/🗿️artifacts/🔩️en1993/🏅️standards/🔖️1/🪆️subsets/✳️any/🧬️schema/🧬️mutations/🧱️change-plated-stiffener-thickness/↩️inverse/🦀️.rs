//! ↩️ `change-plated-stiffener-thickness` inverse: restores the previous `stiffener_thickness`.

use crate::change_plated_stiffener_thickness::ChangePlatedStiffenerThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedStiffenerThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_stiffener_thickness::ChangePlatedStiffenerThickness> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedStiffenerThickness { id: payload.id.clone(), new_stiffener_thickness: row.stiffener_thickness })],
        None => Vec::new(),
    })
}
