//! ↩️ `change-zone-unoccupied-ventilation` inverse.
use crate::change_zone_unoccupied_ventilation::ChangeZoneUnoccupiedVentilation;
use crate::clear_zone_unoccupied_ventilation::ClearZoneUnoccupiedVentilation;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;

pub fn inverse<Op>(payload: &ChangeZoneUnoccupiedVentilation, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_zone_unoccupied_ventilation::ChangeZoneUnoccupiedVentilation> + From<crate::clear_zone_unoccupied_ventilation::ClearZoneUnoccupiedVentilation> {
    let Some(row) = base.zones.iter().find(|row| row.id == payload.zone_id) else { return Ok(Vec::new()); };
    Ok(vec![match &row.unoccupied_ventilation {
        Some(previous) => Op::from(ChangeZoneUnoccupiedVentilation { zone_id: payload.zone_id.clone(), new_unoccupied_ventilation: previous.clone() }),
        None => Op::from(ClearZoneUnoccupiedVentilation { zone_id: payload.zone_id.clone() }),
    }])
}
