//! ↩️ `change-silo-ring-area` inverse: restores the previous `ring_area`.

use crate::change_silo_ring_area::ChangeSiloRingArea;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;

pub fn inverse<Op>(payload: &ChangeSiloRingArea, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_silo_ring_area::ChangeSiloRingArea> {
    Ok(match base.silo_shells.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangeSiloRingArea { id: payload.id.clone(), new_ring_area: row.ring_area })],
        None => Vec::new(),
    })
}
