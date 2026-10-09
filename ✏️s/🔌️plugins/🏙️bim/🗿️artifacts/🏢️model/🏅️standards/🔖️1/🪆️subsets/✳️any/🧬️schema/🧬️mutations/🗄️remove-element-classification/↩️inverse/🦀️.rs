//! ↩️ Inverse of `RemoveElementClassification`: the concrete `SetElementClassification` carrying the removed code of the system, none when the holder had none there.

use super::super::set_element_classification::SetElementClassification;
use super::RemoveElementClassification;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &RemoveElementClassification, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.classifications.get(&payload.id).and_then(|set| set.get(&payload.system)) {
        Some(code) => vec![ModelMutation::SetElementClassification(SetElementClassification { id: payload.id.clone(), system: payload.system.clone(), code: code.clone() })],
        None => Vec::new(),
    }
}
