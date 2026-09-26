//! ↩️ `change-zone-metabolic-rate` inverse.
use super::ChangeZoneMetabolicRate;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeZoneMetabolicRate, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeZoneMetabolicRate(ChangeZoneMetabolicRate { zone_id: payload.zone_id.clone(), new_metabolic_rate_met: z.metabolic_rate_met })]
}
