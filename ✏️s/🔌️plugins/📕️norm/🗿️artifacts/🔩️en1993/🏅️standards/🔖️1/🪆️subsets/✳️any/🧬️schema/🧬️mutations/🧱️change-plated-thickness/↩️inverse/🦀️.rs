//! ↩️ `change-plated-thickness` inverse: restores the previous `thickness`.

use crate::change_plated_thickness::ChangePlatedThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_thickness::ChangePlatedThickness> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedThickness { id: payload.id.clone(), new_thickness: row.thickness })],
        None => Vec::new(),
    })
}
