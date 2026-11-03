//! ↩️ `remove-dynamic-element` inverse: inserts the removed row back at its position.

use crate::remove_dynamic_element::RemoveDynamicElement;
use crate::insert_dynamic_element::InsertDynamicElement;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveDynamicElement, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_dynamic_element::InsertDynamicElement> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveDynamicElement, base: &En1991Snapshot) -> Option<InsertDynamicElement> {
    let (index, row) = base.dynamic_elements.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertDynamicElement { index, item: row.clone() })
}
