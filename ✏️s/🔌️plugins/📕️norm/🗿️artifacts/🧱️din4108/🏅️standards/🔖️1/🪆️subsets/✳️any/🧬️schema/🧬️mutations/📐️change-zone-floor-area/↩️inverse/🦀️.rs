//! ↩️ `change-zone-floor-area` inverse — restores the zone's `floor_area_m2`, computed from BASE state; a missing target yields no step.

use crate::change_zone_floor_area::ChangeZoneFloorArea;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &ChangeZoneFloorArea, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_floor_area::ChangeZoneFloorArea> {
    Ok((|| {
    base.zones.iter().find(|zone| zone.id == payload.zone_id).map(|zone| vec![Op::from(ChangeZoneFloorArea { zone_id: payload.zone_id.clone(), new_floor_area_m2: zone.floor_area_m2 })]).unwrap_or_default()

    })())
}
