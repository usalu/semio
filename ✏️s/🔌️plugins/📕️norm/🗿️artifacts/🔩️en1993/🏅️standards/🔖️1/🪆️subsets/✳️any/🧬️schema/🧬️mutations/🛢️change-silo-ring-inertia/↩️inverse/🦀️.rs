//! ↩️ `change-silo-ring-inertia` inverse: restores the previous `ring_inertia`.

use crate::change_silo_ring_inertia::ChangeSiloRingInertia;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloRingInertia, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_ring_inertia::ChangeSiloRingInertia> {
    Ok(match base.silo_shells.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSiloRingInertia { id: payload.id.clone(), new_ring_inertia: row.ring_inertia })],
        None => Vec::new(),
    })
}
