//! ↩️ `change-zone-heaviness` inverse — restores the zone's `heaviness`, computed from BASE state; a missing target yields no step.

use crate::change_zone_heaviness::ChangeZoneHeaviness;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &ChangeZoneHeaviness, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_heaviness::ChangeZoneHeaviness> {
    Ok((|| {
    base.zones.iter().find(|zone| zone.id == payload.zone_id).map(|zone| vec![Op::from(ChangeZoneHeaviness { zone_id: payload.zone_id.clone(), new_heaviness: zone.heaviness.clone() })]).unwrap_or_default()

    })())
}
