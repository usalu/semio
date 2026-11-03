//! ↩️ `remove-self-weight-element` inverse: inserts the removed row back at its position.

use crate::remove_self_weight_element::RemoveSelfWeightElement;
use crate::insert_self_weight_element::InsertSelfWeightElement;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &RemoveSelfWeightElement, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_self_weight_element::InsertSelfWeightElement> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveSelfWeightElement, base: &En1991Snapshot) -> Option<InsertSelfWeightElement> {
    let (index, row) = base.self_weight_elements.iter().enumerate().find(|(_, row)| row.id == payload.id)?;
    Some(InsertSelfWeightElement { index, item: row.clone() })
}
