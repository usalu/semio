//! ↩️ `change-zone-window-inclination-deg` inverse — restores the window's `inclination_deg`, computed from BASE state; a missing target yields no step.

use crate::change_zone_window_inclination_deg::ChangeZoneWindowInclinationDeg;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &ChangeZoneWindowInclinationDeg, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_window_inclination_deg::ChangeZoneWindowInclinationDeg> {
    Ok((|| {
    base.zones.iter().find(|zone| zone.id == payload.zone_id).and_then(|zone| zone.windows.iter().find(|window| window.id == payload.window_id)).map(|window| vec![Op::from(ChangeZoneWindowInclinationDeg { zone_id: payload.zone_id.clone(), window_id: payload.window_id.clone(), new_inclination_deg: window.inclination_deg })]).unwrap_or_default()

    })())
}
