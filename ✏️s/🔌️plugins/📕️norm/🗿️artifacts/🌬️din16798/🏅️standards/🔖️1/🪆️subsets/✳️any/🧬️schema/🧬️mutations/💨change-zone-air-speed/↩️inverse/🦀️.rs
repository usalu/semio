//! ↩️ `change-zone-air-speed` inverse.
use super::ChangeZoneAirSpeed;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeZoneAirSpeed, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeZoneAirSpeed(ChangeZoneAirSpeed { zone_id: payload.zone_id.clone(), new_air_speed_m_s: z.air_speed_m_s })]
}
