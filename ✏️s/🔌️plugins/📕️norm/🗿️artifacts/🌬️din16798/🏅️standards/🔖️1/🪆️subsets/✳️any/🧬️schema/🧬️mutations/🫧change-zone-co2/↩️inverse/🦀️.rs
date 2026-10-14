//! ↩️ `change-zone-co2` inverse.
use crate::change_zone_co2::ChangeZoneCo2;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeZoneCo2, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_co2::ChangeZoneCo2> {
    Ok((|| {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Op::from(ChangeZoneCo2 { zone_id: payload.zone_id.clone(), new_co2_ppm: z.co2_ppm })]

    })())
}
