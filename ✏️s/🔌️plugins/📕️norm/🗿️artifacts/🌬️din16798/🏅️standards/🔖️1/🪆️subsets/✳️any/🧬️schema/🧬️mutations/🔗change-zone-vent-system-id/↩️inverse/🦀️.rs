//! ↩️ `change-zone-vent-system-id` inverse.
use super::ChangeZoneVentSystemId;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeZoneVentSystemId, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeZoneVentSystemId(ChangeZoneVentSystemId { zone_id: payload.zone_id.clone(), new_vent_system_id: z.vent_system_id.clone() })]
}
