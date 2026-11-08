//! ↩️ `change-element-fc` inverse.

use crate::change_element_fc::ChangeElementFc;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &ChangeElementFc, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_element_fc::ChangeElementFc> {
    Ok(base.elements.iter().find(|row| row.id == payload.element_id).map(|row| vec![Op::from(ChangeElementFc { element_id: payload.element_id.clone(), new_fc: row.fc })]).unwrap_or_default())
}
