//! ↩️ `remove-zone` inverse — re-inserts the removed zone at its position, computed from BASE state; a missing target yields no step.

use crate::remove_zone::RemoveZone;
use crate::insert_zone::InsertZone;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &RemoveZone, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_zone::InsertZone> {
    Ok((|| {
    base.zones.get(payload.index).map(|zone| vec![Op::from(InsertZone { index: Some(payload.index), zone: zone.clone() })]).unwrap_or_default()

    })())
}
