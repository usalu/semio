//! ↩️ `change-zone-co2` inverse.
use super::ChangeZoneCo2;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeZoneCo2, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeZoneCo2(ChangeZoneCo2 { zone_id: payload.zone_id.clone(), new_co2_ppm: z.co2_ppm })]
}
