//! ↩️ `change-zone-window-g-value` inverse — restores the window's `g_value`, computed from BASE state; a missing target yields no step.

use crate::change_zone_window_g_value::ChangeZoneWindowGValue;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &ChangeZoneWindowGValue, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_window_g_value::ChangeZoneWindowGValue> {
    Ok((|| {
    base.zones.iter().find(|zone| zone.id == payload.zone_id).and_then(|zone| zone.windows.iter().find(|window| window.id == payload.window_id)).map(|window| vec![Op::from(ChangeZoneWindowGValue { zone_id: payload.zone_id.clone(), window_id: payload.window_id.clone(), new_g_value: window.g_value })]).unwrap_or_default()

    })())
}
