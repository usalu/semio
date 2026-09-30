//! ↩️ `remove-zone-window` inverse — re-inserts the removed window at its position, computed from BASE state; a missing target yields no step.

use super::RemoveZoneWindow;
use crate::mutations::insert_zone_window::InsertZoneWindow;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &RemoveZoneWindow, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    base.zones.iter().find(|zone| zone.id == payload.zone_id).and_then(|zone| zone.windows.get(payload.index)).map(|window| vec![Din4108Mutation::InsertZoneWindow(InsertZoneWindow { zone_id: payload.zone_id.clone(), index: payload.index, window: window.clone() })]).unwrap_or_default()
}
