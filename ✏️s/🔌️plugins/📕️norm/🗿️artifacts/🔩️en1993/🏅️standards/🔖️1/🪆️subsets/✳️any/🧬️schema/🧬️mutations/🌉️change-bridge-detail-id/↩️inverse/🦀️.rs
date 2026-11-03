//! ↩️ `change-bridge-detail-id` inverse: restores the previous `detail_id`.

use crate::change_bridge_detail_id::ChangeBridgeDetailId;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeBridgeDetailId, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bridge_detail_id::ChangeBridgeDetailId> {
    Ok(match base.bridge_fatigue.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeBridgeDetailId { id: payload.id.clone(), new_detail_id: row.detail_id.clone() })],
        None => Vec::new(),
    })
}
