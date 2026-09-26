//! ↩️ `change-zone-vent-method` inverse.
use super::ChangeZoneVentMethod;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeZoneVentMethod, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeZoneVentMethod(ChangeZoneVentMethod { zone_id: payload.zone_id.clone(), new_vent_method: z.vent_method.clone() })]
}
