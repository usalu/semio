//! ↩️ `change-element-area-m2` inverse.

use crate::change_element_area_m2::ChangeElementAreaM2;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &ChangeElementAreaM2, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_element_area_m2::ChangeElementAreaM2> {
    Ok(base.elements.iter().find(|row| row.id == payload.element_id).map(|row| vec![Op::from(ChangeElementAreaM2 { element_id: payload.element_id.clone(), new_area_m2: row.area_m2 })]).unwrap_or_default())
}
