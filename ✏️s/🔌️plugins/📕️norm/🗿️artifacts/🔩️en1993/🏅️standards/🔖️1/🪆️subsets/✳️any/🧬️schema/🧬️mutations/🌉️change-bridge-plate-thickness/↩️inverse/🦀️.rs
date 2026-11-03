//! ↩️ `change-bridge-plate-thickness` inverse: restores the previous `plate_thickness`.

use crate::change_bridge_plate_thickness::ChangeBridgePlateThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgePlateThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_plate_thickness::ChangeBridgePlateThickness> {
    Ok(match base.bridge_fatigue.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBridgePlateThickness { id: payload.id.clone(), new_plate_thickness: row.plate_thickness })],
        None => Vec::new(),
    })
}
