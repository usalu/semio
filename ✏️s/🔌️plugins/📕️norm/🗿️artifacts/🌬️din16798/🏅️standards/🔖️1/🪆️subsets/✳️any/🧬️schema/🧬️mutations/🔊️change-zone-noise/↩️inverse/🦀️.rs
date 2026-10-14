//! ↩️ `change-zone-noise` inverse.
use crate::change_zone_noise::ChangeZoneNoise;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeZoneNoise, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_noise::ChangeZoneNoise> {
    Ok((|| {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Op::from(ChangeZoneNoise { zone_id: payload.zone_id.clone(), new_noise_db: z.noise_db })]

    })())
}
