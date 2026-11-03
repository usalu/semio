//! ↩️ `change-bridge-trough-thickness` inverse: restores the previous `trough_thickness`.

use crate::change_bridge_trough_thickness::ChangeBridgeTroughThickness;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeTroughThickness, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_trough_thickness::ChangeBridgeTroughThickness> {
    Ok(match base.bridge_fatigue.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBridgeTroughThickness { id: payload.id.clone(), new_trough_thickness: row.trough_thickness })],
        None => Vec::new(),
    })
}
