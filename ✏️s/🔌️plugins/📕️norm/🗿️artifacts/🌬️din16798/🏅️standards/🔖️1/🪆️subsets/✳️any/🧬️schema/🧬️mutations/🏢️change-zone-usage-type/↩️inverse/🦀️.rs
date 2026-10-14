//! ↩️ `change-zone-usage-type` inverse.
use crate::change_zone_usage_type::ChangeZoneUsageType;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeZoneUsageType, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_usage_type::ChangeZoneUsageType> {
    Ok((|| {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Op::from(ChangeZoneUsageType { zone_id: payload.zone_id.clone(), new_usage_type: z.usage_type.clone() })]

    })())
}
