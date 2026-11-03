//! ↩️ `change-plated-flange-thickness` inverse: restores the previous `flange_thickness`.

use crate::change_plated_flange_thickness::ChangePlatedFlangeThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangePlatedFlangeThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_plated_flange_thickness::ChangePlatedFlangeThickness> {
    Ok(match base.plated_panels.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePlatedFlangeThickness { id: payload.id.clone(), new_flange_thickness: row.flange_thickness })],
        None => Vec::new(),
    })
}
