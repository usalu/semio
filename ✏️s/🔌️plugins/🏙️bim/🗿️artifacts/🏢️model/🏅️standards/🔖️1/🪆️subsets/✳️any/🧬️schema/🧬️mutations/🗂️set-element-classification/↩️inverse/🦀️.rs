//! ↩️ Inverse of `SetElementClassification`: the absolute `SetElementClassification` back to the base reference, or a
//! `RemoveElementClassification` when the element had none; none when the element is absent.

use super::super::elements;
use super::super::remove_element_classification::RemoveElementClassification;
use super::SetElementClassification;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetElementClassification, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if !elements::exists(base, &payload.id) {
        return Vec::new();
    }
    match base.classifications.get(&payload.id) {
        Some(classification) => vec![ModelMutation::SetElementClassification(SetElementClassification { id: payload.id.clone(), classification: classification.clone() })],
        None => vec![ModelMutation::RemoveElementClassification(RemoveElementClassification { id: payload.id.clone() })],
    }
}
