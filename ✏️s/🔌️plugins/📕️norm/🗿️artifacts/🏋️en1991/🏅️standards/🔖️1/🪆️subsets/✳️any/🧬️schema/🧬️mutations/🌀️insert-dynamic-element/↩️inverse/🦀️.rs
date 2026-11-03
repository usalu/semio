//! ↩️ `insert-dynamic-element` inverse: removes the inserted row again.

use crate::insert_dynamic_element::InsertDynamicElement;
use crate::remove_dynamic_element::RemoveDynamicElement;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertDynamicElement, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_dynamic_element::RemoveDynamicElement> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertDynamicElement, base: &En1991Snapshot) -> Option<RemoveDynamicElement> {
    Some(RemoveDynamicElement { id: payload.item.id.clone() })
}
