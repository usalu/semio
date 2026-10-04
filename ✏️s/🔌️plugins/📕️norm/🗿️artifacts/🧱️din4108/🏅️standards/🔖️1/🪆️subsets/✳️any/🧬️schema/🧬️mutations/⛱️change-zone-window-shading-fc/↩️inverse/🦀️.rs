//! ↩️ `change-zone-window-shading-fc` inverse — restores the window's `shading_fc`, computed from BASE state; a missing target yields no step.

use super::ChangeZoneWindowShadingFc;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &ChangeZoneWindowShadingFc, base: &Din4108Snapshot) -> Result<Vec<Din4108Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.zones.iter().find(|zone| zone.id == payload.zone_id).and_then(|zone| zone.windows.iter().find(|window| window.id == payload.window_id)).map(|window| vec![Din4108Mutation::ChangeZoneWindowShadingFc(ChangeZoneWindowShadingFc { zone_id: payload.zone_id.clone(), window_id: payload.window_id.clone(), new_shading_fc: window.shading_fc })]).unwrap_or_default()

    })())
}
