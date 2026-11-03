//! ↩️ `change-bridge-trough-spacing` inverse: restores the previous `trough_spacing`.

use crate::change_bridge_trough_spacing::ChangeBridgeTroughSpacing;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeTroughSpacing, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_trough_spacing::ChangeBridgeTroughSpacing> {
    Ok(match base.bridge_fatigue.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBridgeTroughSpacing { id: payload.id.clone(), new_trough_spacing: row.trough_spacing })],
        None => Vec::new(),
    })
}
