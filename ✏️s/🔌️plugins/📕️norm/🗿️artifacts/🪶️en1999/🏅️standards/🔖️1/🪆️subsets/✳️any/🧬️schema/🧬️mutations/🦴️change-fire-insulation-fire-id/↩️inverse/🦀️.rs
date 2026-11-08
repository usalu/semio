//! ↩️ `change-fire-insulation-fire-id` inverse: restores the previous `fire_id`.

use crate::change_fire_insulation_fire_id::ChangeFireInsulationFireId;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeFireInsulationFireId, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fire_insulation_fire_id::ChangeFireInsulationFireId> {
    Ok(match base.fire_insulations.iter().find(|row| row.id == payload.fire_insulation_id) {
        Some(row) => vec![Op::from(ChangeFireInsulationFireId { fire_insulation_id: payload.fire_insulation_id.clone(), new_fire_id: row.fire_id.clone() })],
        None => Vec::new(),
    })
}
