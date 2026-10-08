//! ↩️ Inverse of `RemoveElementProperty`: the concrete `SetElementProperty` carrying the removed typed value, none when the property was absent.

use super::super::set_element_property::SetElementProperty;
use super::RemoveElementProperty;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &RemoveElementProperty, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.properties.get(&payload.id).and_then(|sets| sets.get(&payload.pset)).and_then(|properties| properties.get(&payload.property)) {
        Some(value) => vec![ModelMutation::SetElementProperty(SetElementProperty { id: payload.id.clone(), pset: payload.pset.clone(), property: payload.property.clone(), value: value.clone() })],
        None => Vec::new(),
    }
}
