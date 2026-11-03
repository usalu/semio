//! ↩️ `change-ring-internal` inverse: restores the previous `internal`.

use crate::change_ring_internal::ChangeRingInternal;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeRingInternal, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_ring_internal::ChangeRingInternal> {
    Ok(match base.ring_stiffeners.iter().find(|row| row.id == payload.ring_stiffener_id) {
        Some(row) => vec![Op::from(ChangeRingInternal { ring_stiffener_id: payload.ring_stiffener_id.clone(), new_internal: row.internal })],
        None => Vec::new(),
    })
}
