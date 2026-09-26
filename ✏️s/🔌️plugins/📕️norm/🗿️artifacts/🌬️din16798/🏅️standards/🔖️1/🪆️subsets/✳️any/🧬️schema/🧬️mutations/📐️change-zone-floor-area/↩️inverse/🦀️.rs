//! ↩️ `change-zone-floor-area` inverse.
use super::ChangeZoneFloorArea;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeZoneFloorArea, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeZoneFloorArea(ChangeZoneFloorArea { zone_id: payload.zone_id.clone(), new_floor_area_m2: z.floor_area_m2 })]
}
