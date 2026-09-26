//! ↩️ `change-zone-occupants` inverse.
use super::ChangeZoneOccupants;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeZoneOccupants, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeZoneOccupants(ChangeZoneOccupants { zone_id: payload.zone_id.clone(), new_occupants: z.occupants })]
}
