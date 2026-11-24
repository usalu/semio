//! ↩️ `clear-zone-unoccupied-ventilation` inverse.
use crate::change_zone_unoccupied_ventilation::ChangeZoneUnoccupiedVentilation;
use crate::clear_zone_unoccupied_ventilation::ClearZoneUnoccupiedVentilation;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;

pub fn inverse<Op>(payload: &ClearZoneUnoccupiedVentilation, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_unoccupied_ventilation::ChangeZoneUnoccupiedVentilation> + From<crate::clear_zone_unoccupied_ventilation::ClearZoneUnoccupiedVentilation> {
    let Some(row) = base.zones.iter().find(|row| row.id == payload.zone_id) else { return Ok(Vec::new()); };
    Ok(row.unoccupied_ventilation.iter().map(|previous| Op::from(ChangeZoneUnoccupiedVentilation { zone_id: payload.zone_id.clone(), new_unoccupied_ventilation: previous.clone() })).collect())
}
