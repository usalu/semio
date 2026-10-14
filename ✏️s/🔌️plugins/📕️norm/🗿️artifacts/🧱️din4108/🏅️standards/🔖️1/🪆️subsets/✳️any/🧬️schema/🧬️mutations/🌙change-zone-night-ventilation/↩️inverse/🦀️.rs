//! ↩️ `change-zone-night-ventilation` inverse — restores the zone's `night_ventilation`, computed from BASE state; a missing target yields no step.

use crate::change_zone_night_ventilation::ChangeZoneNightVentilation;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &ChangeZoneNightVentilation, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_night_ventilation::ChangeZoneNightVentilation> {
    Ok((|| {
    base.zones.iter().find(|zone| zone.id == payload.zone_id).map(|zone| vec![Op::from(ChangeZoneNightVentilation { zone_id: payload.zone_id.clone(), new_night_ventilation: zone.night_ventilation.clone() })]).unwrap_or_default()

    })())
}
