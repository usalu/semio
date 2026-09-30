//! ↩️ `change-zone-window-g-value` inverse — restores the window's `g_value`, computed from BASE state; a missing target yields no step.

use super::ChangeZoneWindowGValue;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &ChangeZoneWindowGValue, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    base.zones.iter().find(|zone| zone.id == payload.zone_id).and_then(|zone| zone.windows.iter().find(|window| window.id == payload.window_id)).map(|window| vec![Din4108Mutation::ChangeZoneWindowGValue(ChangeZoneWindowGValue { zone_id: payload.zone_id.clone(), window_id: payload.window_id.clone(), new_g_value: window.g_value })]).unwrap_or_default()
}
