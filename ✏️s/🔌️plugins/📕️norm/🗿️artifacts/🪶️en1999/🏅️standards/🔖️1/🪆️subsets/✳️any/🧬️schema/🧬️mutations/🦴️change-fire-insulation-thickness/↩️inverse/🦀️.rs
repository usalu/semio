//! ↩️ `change-fire-insulation-thickness` inverse: restores the previous `thickness`.

use crate::change_fire_insulation_thickness::ChangeFireInsulationThickness;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeFireInsulationThickness, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_insulation_thickness::ChangeFireInsulationThickness> {
    Ok(match base.fire_insulations.iter().find(|row| row.id == payload.fire_insulation_id) {
        Some(row) => vec![Op::from(ChangeFireInsulationThickness { fire_insulation_id: payload.fire_insulation_id.clone(), new_thickness: row.thickness })],
        None => Vec::new(),
    })
}
