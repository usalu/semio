//! ↩️ `change-tower-ring-mode` inverse: restores the previous `ring_mode`.

use crate::change_tower_ring_mode::ChangeTowerRingMode;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeTowerRingMode, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_tower_ring_mode::ChangeTowerRingMode> {
    Ok(match base.tower_legs.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeTowerRingMode { id: payload.id.clone(), new_ring_mode: row.ring_mode })],
        None => Vec::new(),
    })
}
