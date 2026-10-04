//! ↩️ `change-element-kind` inverse — restores the element's `kind`, computed from BASE state; a missing target yields no step.

use super::ChangeElementKind;
use crate::{Din4108Mutation, Din4108Snapshot};

pub fn inverse(payload: &ChangeElementKind, base: &Din4108Snapshot) -> Result<Vec<Din4108Mutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.elements.iter().find(|element| element.id == payload.element_id).map(|element| vec![Din4108Mutation::ChangeElementKind(ChangeElementKind { element_id: payload.element_id.clone(), new_kind: element.kind.clone() })]).unwrap_or_default()

    })())
}
