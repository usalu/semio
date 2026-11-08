//! ↩️ `change-element-label-en` inverse.

use crate::change_element_label_en::ChangeElementLabelEn;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &ChangeElementLabelEn, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_element_label_en::ChangeElementLabelEn> {
    Ok(base.elements.iter().find(|row| row.id == payload.element_id).map(|row| vec![Op::from(ChangeElementLabelEn { element_id: payload.element_id.clone(), new_label_en: row.label_en.clone() })]).unwrap_or_default())
}
