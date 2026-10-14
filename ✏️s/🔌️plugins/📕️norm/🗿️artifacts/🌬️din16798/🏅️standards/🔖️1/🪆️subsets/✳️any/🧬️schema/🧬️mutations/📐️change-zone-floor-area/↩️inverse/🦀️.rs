//! ↩️ `change-zone-floor-area` inverse.
use crate::change_zone_floor_area::ChangeZoneFloorArea;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeZoneFloorArea, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_floor_area::ChangeZoneFloorArea> {
    Ok((|| {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Op::from(ChangeZoneFloorArea { zone_id: payload.zone_id.clone(), new_floor_area_m2: z.floor_area_m2 })]

    })())
}
