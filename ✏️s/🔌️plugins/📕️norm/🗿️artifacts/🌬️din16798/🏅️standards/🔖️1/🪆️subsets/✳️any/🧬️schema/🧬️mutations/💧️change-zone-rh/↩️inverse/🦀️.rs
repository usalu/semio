//! ↩️ `change-zone-rh` inverse.
use super::ChangeZoneRh;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeZoneRh, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeZoneRh(ChangeZoneRh { zone_id: payload.zone_id.clone(), new_rh_percent: z.rh_percent })]
}
