//! ↩️ `change-bridge-trough-width` inverse: restores the previous `trough_width`.

use crate::change_bridge_trough_width::ChangeBridgeTroughWidth;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeTroughWidth, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_trough_width::ChangeBridgeTroughWidth> {
    Ok(match base.bridge_fatigue.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBridgeTroughWidth { id: payload.id.clone(), new_trough_width: row.trough_width })],
        None => Vec::new(),
    })
}
