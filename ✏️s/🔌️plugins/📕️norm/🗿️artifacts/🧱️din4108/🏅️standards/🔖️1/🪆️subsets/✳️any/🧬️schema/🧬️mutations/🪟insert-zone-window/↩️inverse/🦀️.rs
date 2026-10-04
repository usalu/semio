//! ↩️ `insert-zone-window` inverse — removes the inserted window at its landing position, computed from BASE state; a missing target yields no step.

use super::InsertZoneWindow;
use crate::mutations::remove_zone_window::RemoveZoneWindow;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &InsertZoneWindow, base: &Din4108Snapshot) -> Result<Vec<Din4108Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.zones.iter().find(|zone| zone.id == payload.zone_id).map(|zone| vec![Din4108Mutation::RemoveZoneWindow(RemoveZoneWindow { zone_id: payload.zone_id.clone(), index: payload.index.min(zone.windows.len()) })]).unwrap_or_default()

    })())
}
