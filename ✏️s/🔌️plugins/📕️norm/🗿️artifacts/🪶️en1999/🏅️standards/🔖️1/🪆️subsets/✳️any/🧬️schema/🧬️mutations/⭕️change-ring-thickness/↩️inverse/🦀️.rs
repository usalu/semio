//! ↩️ `change-ring-thickness` inverse: restores the previous `thickness`.

use crate::change_ring_thickness::ChangeRingThickness;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeRingThickness, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_ring_thickness::ChangeRingThickness> {
    Ok(match base.ring_stiffeners.iter().find(|row| row.id == payload.ring_stiffener_id) {
        Some(row) => vec![Op::from(ChangeRingThickness { ring_stiffener_id: payload.ring_stiffener_id.clone(), new_thickness: row.thickness })],
        None => Vec::new(),
    })
}
