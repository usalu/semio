//! ↩️ `remove-ring-stiffener` inverse.

use crate::remove_ring_stiffener::RemoveRingStiffener;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use crate::insert_ring_stiffener;

pub fn inverse<Op>(payload: &RemoveRingStiffener, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_ring_stiffener::InsertRingStiffener> {
    Ok((|| {
    if let Some((index, row)) = base.ring_stiffeners.iter().enumerate().find(|(_, row)| row.id == payload.id) {
        vec![Op::from(insert_ring_stiffener::InsertRingStiffener { index: index as u32, ring_stiffener: row.clone() })]
    } else { Vec::new() }

    })())
}
