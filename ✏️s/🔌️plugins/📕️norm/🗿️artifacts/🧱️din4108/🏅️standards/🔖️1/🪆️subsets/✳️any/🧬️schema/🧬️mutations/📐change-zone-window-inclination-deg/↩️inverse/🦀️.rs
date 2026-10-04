//! ↩️ `change-zone-window-inclination-deg` inverse — restores the window's `inclination_deg`, computed from BASE state; a missing target yields no step.

use super::ChangeZoneWindowInclinationDeg;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &ChangeZoneWindowInclinationDeg, base: &Din4108Snapshot) -> Result<Vec<Din4108Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.zones.iter().find(|zone| zone.id == payload.zone_id).and_then(|zone| zone.windows.iter().find(|window| window.id == payload.window_id)).map(|window| vec![Din4108Mutation::ChangeZoneWindowInclinationDeg(ChangeZoneWindowInclinationDeg { zone_id: payload.zone_id.clone(), window_id: payload.window_id.clone(), new_inclination_deg: window.inclination_deg })]).unwrap_or_default()

    })())
}
