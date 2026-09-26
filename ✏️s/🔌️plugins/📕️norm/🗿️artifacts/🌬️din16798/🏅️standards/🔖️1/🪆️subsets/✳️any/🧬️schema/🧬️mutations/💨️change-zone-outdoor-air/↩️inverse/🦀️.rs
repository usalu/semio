//! ↩️ `change-zone-outdoor-air` inverse.
use super::ChangeZoneOutdoorAir;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeZoneOutdoorAir, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeZoneOutdoorAir(ChangeZoneOutdoorAir { zone_id: payload.zone_id.clone(), new_outdoor_air_supplied_m3_h: z.outdoor_air_supplied_m3_h })]
}
