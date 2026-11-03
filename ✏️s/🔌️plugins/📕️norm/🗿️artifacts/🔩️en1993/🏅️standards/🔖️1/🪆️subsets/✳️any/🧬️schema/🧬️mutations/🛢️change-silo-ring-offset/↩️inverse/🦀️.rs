//! ↩️ `change-silo-ring-offset` inverse: restores the previous `ring_offset`.

use crate::change_silo_ring_offset::ChangeSiloRingOffset;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloRingOffset, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_ring_offset::ChangeSiloRingOffset> {
    Ok(match base.silo_shells.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSiloRingOffset { id: payload.id.clone(), new_ring_offset: row.ring_offset })],
        None => Vec::new(),
    })
}
