//! ↩️ Inverse of `SetElementClassification`: the absolute `SetElementClassification` back to the base code of the system, or a `RemoveElementClassification` of the system when the holder had none there; none when the holder
//! is absent.

use super::super::elements;
use super::super::remove_element_classification::RemoveElementClassification;
use super::SetElementClassification;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetElementClassification, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if !elements::holds_data(base, &payload.id) {
        return Vec::new();
    }
    match base.classifications.get(&payload.id).and_then(|set| set.get(&payload.system)) {
        Some(code) => vec![ModelMutation::SetElementClassification(SetElementClassification { id: payload.id.clone(), system: payload.system.clone(), code: code.clone() })],
        None => vec![ModelMutation::RemoveElementClassification(RemoveElementClassification { id: payload.id.clone(), system: payload.system.clone() })],
    }
}
