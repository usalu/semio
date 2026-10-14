//! ↩️ `insert-zone-window` inverse — removes the inserted window at its landing position, computed from BASE state; a missing target yields no step.

use crate::insert_zone_window::InsertZoneWindow;
use crate::remove_zone_window::RemoveZoneWindow;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &InsertZoneWindow, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_zone_window::RemoveZoneWindow> {
    Ok((|| {
    base.zones.iter().find(|zone| zone.id == payload.zone_id).map(|zone| vec![Op::from(RemoveZoneWindow { zone_id: payload.zone_id.clone(), index: payload.index.unwrap_or(usize::MAX).min(zone.windows.len()) })]).unwrap_or_default()

    })())
}
