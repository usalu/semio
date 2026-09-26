//! ↩️ `change-zone-noise` inverse.
use super::ChangeZoneNoise;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeZoneNoise, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeZoneNoise(ChangeZoneNoise { zone_id: payload.zone_id.clone(), new_noise_db: z.noise_db })]
}
