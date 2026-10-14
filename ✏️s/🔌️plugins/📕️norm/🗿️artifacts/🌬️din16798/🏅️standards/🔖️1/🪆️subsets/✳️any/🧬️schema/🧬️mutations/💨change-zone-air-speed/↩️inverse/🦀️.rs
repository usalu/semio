//! ↩️ `change-zone-air-speed` inverse.
use crate::change_zone_air_speed::ChangeZoneAirSpeed;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeZoneAirSpeed, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_air_speed::ChangeZoneAirSpeed> {
    Ok((|| {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Op::from(ChangeZoneAirSpeed { zone_id: payload.zone_id.clone(), new_air_speed_m_s: z.air_speed_m_s })]

    })())
}
