//! ↩️ `change-zone-usage-profile` inverse.

use crate::change_zone_usage_profile::ChangeZoneUsageProfile;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &ChangeZoneUsageProfile, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_usage_profile::ChangeZoneUsageProfile> {
    Ok(base.zones.iter().find(|row| row.id == payload.zone_id).map(|row| vec![Op::from(ChangeZoneUsageProfile { zone_id: payload.zone_id.clone(), new_usage_profile: row.usage_profile })]).unwrap_or_default())
}
