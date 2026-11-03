//! ↩️ `change-bridge-crossbeam-spacing` inverse: restores the previous `crossbeam_spacing`.

use crate::change_bridge_crossbeam_spacing::ChangeBridgeCrossbeamSpacing;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeCrossbeamSpacing, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_crossbeam_spacing::ChangeBridgeCrossbeamSpacing> {
    Ok(match base.bridge_fatigue.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBridgeCrossbeamSpacing { id: payload.id.clone(), new_crossbeam_spacing: row.crossbeam_spacing })],
        None => Vec::new(),
    })
}
