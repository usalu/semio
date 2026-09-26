//! ↩️ `change-zone-usage-type` inverse.
use super::ChangeZoneUsageType;
use crate::{Din16798Mutation, Din16798Snapshot};
pub fn inverse(payload: &ChangeZoneUsageType, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
    let Some(z) = base.zones.iter().find(|z| z.id == payload.zone_id) else { return Vec::new(); };
    vec![Din16798Mutation::ChangeZoneUsageType(ChangeZoneUsageType { zone_id: payload.zone_id.clone(), new_usage_type: z.usage_type.clone() })]
}
