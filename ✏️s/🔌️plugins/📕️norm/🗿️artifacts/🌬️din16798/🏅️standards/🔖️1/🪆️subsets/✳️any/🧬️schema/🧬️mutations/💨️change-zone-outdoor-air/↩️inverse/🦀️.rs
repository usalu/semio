//! ↩️ `change-zone-outdoor-air` inverse.
use crate::change_zone_outdoor_air::ChangeZoneOutdoorAir;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeZoneOutdoorAir, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_outdoor_air::ChangeZoneOutdoorAir> {
    Ok((|| {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Op::from(ChangeZoneOutdoorAir { zone_id: payload.zone_id.clone(), new_outdoor_air_supplied_m3_h: z.outdoor_air_supplied_m3_h })]

    })())
}
