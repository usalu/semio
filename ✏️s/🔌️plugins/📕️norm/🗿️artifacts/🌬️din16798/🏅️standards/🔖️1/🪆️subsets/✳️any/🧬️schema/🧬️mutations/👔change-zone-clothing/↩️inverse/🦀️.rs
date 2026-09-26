//! ↩️ `change-zone-clothing` inverse.
use super::ChangeZoneClothing;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeZoneClothing, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeZoneClothing(ChangeZoneClothing { zone_id: payload.zone_id.clone(), new_clothing_clo: z.clothing_clo })]
}
