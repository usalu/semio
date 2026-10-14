//! ↩️ `change-element-kind` inverse — restores the element's `kind`, computed from BASE state; a missing target yields no step.

use crate::change_element_kind::ChangeElementKind;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;

pub fn inverse<Op>(payload: &ChangeElementKind, base: &Din4108Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_element_kind::ChangeElementKind> {
    Ok((|| {
    base.elements.iter().find(|element| element.id == payload.element_id).map(|element| vec![Op::from(ChangeElementKind { element_id: payload.element_id.clone(), new_kind: element.kind.clone() })]).unwrap_or_default()

    })())
}
