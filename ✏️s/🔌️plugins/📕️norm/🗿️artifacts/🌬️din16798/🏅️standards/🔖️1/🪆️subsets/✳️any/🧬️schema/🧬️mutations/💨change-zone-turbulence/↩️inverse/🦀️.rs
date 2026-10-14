//! ↩️ `change-zone-turbulence` inverse.
use crate::change_zone_turbulence::ChangeZoneTurbulence;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeZoneTurbulence, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_turbulence::ChangeZoneTurbulence> {
    Ok((|| {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Op::from(ChangeZoneTurbulence { zone_id: payload.zone_id.clone(), new_turbulence_intensity_percent: z.turbulence_intensity_percent })]

    })())
}
