//! ↩️ `insert-zone` inverse — removes the inserted zone at its landing position, computed from BASE state; a missing target yields no step.

use crate::insert_zone::InsertZone;
use crate::remove_zone::RemoveZone;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &InsertZone, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_zone::RemoveZone> {
    Ok((|| {
    vec![Op::from(RemoveZone { index: payload.index.unwrap_or(usize::MAX).min(base.zones.len()) })]

    })())
}
