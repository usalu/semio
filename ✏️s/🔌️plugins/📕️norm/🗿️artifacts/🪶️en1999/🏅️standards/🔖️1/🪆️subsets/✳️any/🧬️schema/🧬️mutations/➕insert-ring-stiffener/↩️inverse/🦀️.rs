//! ↩️ `insert-ring-stiffener` inverse.

use crate::insert_ring_stiffener::InsertRingStiffener;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::remove_ring_stiffener;

pub fn inverse<Op>(payload: &InsertRingStiffener, _base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_ring_stiffener::RemoveRingStiffener> {
    Ok((|| {
    vec![Op::from(remove_ring_stiffener::RemoveRingStiffener { id: payload.ring_stiffener.id.clone() })]

    })())
}
