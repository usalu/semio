//! ↩️ `remove-zone-window` inverse — re-inserts the removed window at its position, computed from BASE state; a missing target yields no step.

use crate::remove_zone_window::RemoveZoneWindow;
use crate::insert_zone_window::InsertZoneWindow;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &RemoveZoneWindow, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_zone_window::InsertZoneWindow> {
    Ok((|| {
    base.zones.iter().find(|zone| zone.id == payload.zone_id).and_then(|zone| zone.windows.get(payload.index)).map(|window| vec![Op::from(InsertZoneWindow { zone_id: payload.zone_id.clone(), index: Some(payload.index), window: window.clone() })]).unwrap_or_default()

    })())
}
