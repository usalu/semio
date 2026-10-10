//! ↩️ Inverse of `CreateMepElement`: the concrete `DeleteMepElement` of the id it created, none when the id was already taken.

use super::super::delete_mep_element::DeleteMepElement;
use super::CreateMepElement;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &CreateMepElement, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if base.mep_elements.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![ModelMutation::DeleteMepElement(DeleteMepElement { id: payload.id.clone() })]
}
