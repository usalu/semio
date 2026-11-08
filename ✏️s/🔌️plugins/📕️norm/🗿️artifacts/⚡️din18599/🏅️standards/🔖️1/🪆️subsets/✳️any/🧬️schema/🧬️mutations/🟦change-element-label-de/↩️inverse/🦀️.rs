//! ↩️ `change-element-label-de` inverse.

use crate::change_element_label_de::ChangeElementLabelDe;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &ChangeElementLabelDe, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_element_label_de::ChangeElementLabelDe> {
    Ok(base.elements.iter().find(|row| row.id == payload.element_id).map(|row| vec![Op::from(ChangeElementLabelDe { element_id: payload.element_id.clone(), new_label_de: row.label_de.clone() })]).unwrap_or_default())
}
