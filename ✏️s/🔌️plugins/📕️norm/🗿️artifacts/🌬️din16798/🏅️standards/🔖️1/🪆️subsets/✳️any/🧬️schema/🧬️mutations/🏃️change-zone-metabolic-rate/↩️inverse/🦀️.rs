//! ↩️ `change-zone-metabolic-rate` inverse.
use crate::change_zone_metabolic_rate::ChangeZoneMetabolicRate;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeZoneMetabolicRate, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_metabolic_rate::ChangeZoneMetabolicRate> {
    Ok((|| {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Op::from(ChangeZoneMetabolicRate { zone_id: payload.zone_id.clone(), new_metabolic_rate_met: z.metabolic_rate_met })]

    })())
}
