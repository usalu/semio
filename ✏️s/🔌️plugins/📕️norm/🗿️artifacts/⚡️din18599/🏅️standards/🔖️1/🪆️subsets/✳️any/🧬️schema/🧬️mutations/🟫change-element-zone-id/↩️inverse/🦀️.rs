//! ↩️ `change-element-zone-id` inverse.

use crate::change_element_zone_id::ChangeElementZoneId;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &ChangeElementZoneId, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_element_zone_id::ChangeElementZoneId> {
    Ok(base.elements.iter().find(|row| row.id == payload.element_id).map(|row| vec![Op::from(ChangeElementZoneId { element_id: payload.element_id.clone(), new_zone_id: row.zone_id.clone() })]).unwrap_or_default())
}
