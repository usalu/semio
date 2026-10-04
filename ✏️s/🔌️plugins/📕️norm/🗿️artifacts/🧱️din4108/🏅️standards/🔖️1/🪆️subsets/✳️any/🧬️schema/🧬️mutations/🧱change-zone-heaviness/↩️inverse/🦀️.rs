//! ↩️ `change-zone-heaviness` inverse — restores the zone's `heaviness`, computed from BASE state; a missing target yields no step.

use super::ChangeZoneHeaviness;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &ChangeZoneHeaviness, base: &Din4108Snapshot) -> Result<Vec<Din4108Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.zones.iter().find(|zone| zone.id == payload.zone_id).map(|zone| vec![Din4108Mutation::ChangeZoneHeaviness(ChangeZoneHeaviness { zone_id: payload.zone_id.clone(), new_heaviness: zone.heaviness.clone() })]).unwrap_or_default()

    })())
}
