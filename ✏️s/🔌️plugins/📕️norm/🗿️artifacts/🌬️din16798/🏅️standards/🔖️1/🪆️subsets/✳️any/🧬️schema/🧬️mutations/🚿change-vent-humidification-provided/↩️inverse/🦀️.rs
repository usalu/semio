//! ↩️ `change-vent-humidification-provided` inverse.
use crate::change_vent_humidification_provided::ChangeVentHumidificationProvided;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
pub fn inverse<Op>(payload: &ChangeVentHumidificationProvided, base: &Din16798Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_vent_humidification_provided::ChangeVentHumidificationProvided> {
    Ok((|| {
    let Some(row) = base.vent_systems.iter().find(|row| row.id == payload.vent_id) else { return Vec::new(); };
    vec![Op::from(ChangeVentHumidificationProvided { vent_id: payload.vent_id.clone(), new_humidification_provided_kg_h: row.humidification_provided_kg_h })]

    })())
}
