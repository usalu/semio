//! ↩️ `change-zone-illuminance` inverse.
use super::ChangeZoneIlluminance;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeZoneIlluminance, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeZoneIlluminance(ChangeZoneIlluminance { zone_id: payload.zone_id.clone(), new_illuminance_lx: z.illuminance_lx })]
}
