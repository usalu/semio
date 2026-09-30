//! ↩️ `change-zone-floor-area` inverse — restores the zone's `floor_area_m2`, computed from BASE state; a missing target yields no step.

use super::ChangeZoneFloorArea;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &ChangeZoneFloorArea, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    base.zones.iter().find(|zone| zone.id == payload.zone_id).map(|zone| vec![Din4108Mutation::ChangeZoneFloorArea(ChangeZoneFloorArea { zone_id: payload.zone_id.clone(), new_floor_area_m2: zone.floor_area_m2 })]).unwrap_or_default()
}
