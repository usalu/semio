//! ↩️ `change-element-g-value` inverse.

use crate::change_element_g_value::ChangeElementGValue;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &ChangeElementGValue, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_element_g_value::ChangeElementGValue> {
    Ok(base.elements.iter().find(|row| row.id == payload.element_id).map(|row| vec![Op::from(ChangeElementGValue { element_id: payload.element_id.clone(), new_g_value: row.g_value })]).unwrap_or_default())
}
