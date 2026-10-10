//! ↩️ Inverse of `SetMepElement`: an absolute `SetMepElement` restoring the base value of exactly the fields the forward really changes, none when the element is absent or nothing changes.

use super::SetMepElement;
use crate::{ModelMutation, ModelSnapshot, Patch};

pub fn inverse(payload: &SetMepElement, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(record) = base.mep_elements.get(&payload.id) else {
        return Vec::new();
    };
    let restore = payload.patch().minimal(record).restoring(record);
    if restore.is_empty() {
        return Vec::new();
    }
    vec![ModelMutation::SetMepElement(SetMepElement::from_patch(payload.id.clone(), restore))]
}
