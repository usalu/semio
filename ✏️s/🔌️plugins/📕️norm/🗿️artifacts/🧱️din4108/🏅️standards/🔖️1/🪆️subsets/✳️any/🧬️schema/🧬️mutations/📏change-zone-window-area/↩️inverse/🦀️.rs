//! ↩️ `change-zone-window-area` inverse — restores the window's `area_m2`, computed from BASE state; a missing target yields no step.

use super::ChangeZoneWindowArea;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &ChangeZoneWindowArea, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    base.zones.iter().find(|zone| zone.id == payload.zone_id).and_then(|zone| zone.windows.iter().find(|window| window.id == payload.window_id)).map(|window| vec![Din4108Mutation::ChangeZoneWindowArea(ChangeZoneWindowArea { zone_id: payload.zone_id.clone(), window_id: payload.window_id.clone(), new_area_m2: window.area_m2 })]).unwrap_or_default()
}
