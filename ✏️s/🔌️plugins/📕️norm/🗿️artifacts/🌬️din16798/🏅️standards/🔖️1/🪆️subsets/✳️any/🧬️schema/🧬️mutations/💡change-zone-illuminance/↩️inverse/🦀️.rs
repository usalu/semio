//! ↩️ `change-zone-illuminance` inverse.
use crate::change_zone_illuminance::ChangeZoneIlluminance;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeZoneIlluminance, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_illuminance::ChangeZoneIlluminance> {
    Ok((|| {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Op::from(ChangeZoneIlluminance { zone_id: payload.zone_id.clone(), new_illuminance_lx: z.illuminance_lx })]

    })())
}
