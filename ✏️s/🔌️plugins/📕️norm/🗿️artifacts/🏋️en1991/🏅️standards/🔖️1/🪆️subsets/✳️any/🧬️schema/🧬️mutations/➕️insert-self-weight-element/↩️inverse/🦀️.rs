//! ↩️ `insert-self-weight-element` inverse: removes the inserted row again.

use crate::insert_self_weight_element::InsertSelfWeightElement;
use crate::remove_self_weight_element::RemoveSelfWeightElement;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn inverse<Op>(payload: &InsertSelfWeightElement, base: &En1991Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_self_weight_element::RemoveSelfWeightElement> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertSelfWeightElement, base: &En1991Snapshot) -> Option<RemoveSelfWeightElement> {
    Some(RemoveSelfWeightElement { id: payload.item.id.clone() })
}
