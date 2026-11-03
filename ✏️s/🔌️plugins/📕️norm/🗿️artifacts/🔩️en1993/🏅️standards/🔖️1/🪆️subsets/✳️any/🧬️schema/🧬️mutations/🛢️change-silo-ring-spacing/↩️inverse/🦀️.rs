//! ↩️ `change-silo-ring-spacing` inverse: restores the previous `ring_spacing`.

use crate::change_silo_ring_spacing::ChangeSiloRingSpacing;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloRingSpacing, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_ring_spacing::ChangeSiloRingSpacing> {
    Ok(match base.silo_shells.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSiloRingSpacing { id: payload.id.clone(), new_ring_spacing: row.ring_spacing })],
        None => Vec::new(),
    })
}
