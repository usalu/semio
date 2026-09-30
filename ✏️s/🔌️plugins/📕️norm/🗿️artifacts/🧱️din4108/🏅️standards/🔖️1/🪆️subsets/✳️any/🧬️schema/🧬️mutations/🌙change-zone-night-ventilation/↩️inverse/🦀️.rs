//! ↩️ `change-zone-night-ventilation` inverse — restores the zone's `night_ventilation`, computed from BASE state; a missing target yields no step.

use super::ChangeZoneNightVentilation;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &ChangeZoneNightVentilation, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
    base.zones.iter().find(|zone| zone.id == payload.zone_id).map(|zone| vec![Din4108Mutation::ChangeZoneNightVentilation(ChangeZoneNightVentilation { zone_id: payload.zone_id.clone(), new_night_ventilation: zone.night_ventilation.clone() })]).unwrap_or_default()
}
