//! ↩️ `change-element-kind` inverse.

use crate::change_element_kind::ChangeElementKind;

use semio_s_artifact_norm_din18599_model::Din18599Snapshot;

pub fn inverse<Op>(payload: &ChangeElementKind, base: &Din18599Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_element_kind::ChangeElementKind> {
    Ok(base.elements.iter().find(|row| row.id == payload.element_id).map(|row| vec![Op::from(ChangeElementKind { element_id: payload.element_id.clone(), new_kind: row.kind })]).unwrap_or_default())
}
