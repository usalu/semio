//! ↩️ Inverse of `RemoveElementClassification`: the concrete `SetElementClassification` carrying the removed reference, none when absent.

use super::super::set_element_classification::SetElementClassification;
use super::RemoveElementClassification;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &RemoveElementClassification, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.classifications.get(&payload.id) {
        Some(classification) => vec![ModelMutation::SetElementClassification(SetElementClassification { id: payload.id.clone(), classification: classification.clone() })],
        None => Vec::new(),
    }
}
