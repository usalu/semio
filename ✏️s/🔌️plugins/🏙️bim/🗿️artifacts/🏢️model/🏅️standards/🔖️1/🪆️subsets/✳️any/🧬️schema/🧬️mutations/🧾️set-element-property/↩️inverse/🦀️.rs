//! ↩️ Inverse of `SetElementProperty`: an absolute `SetElementProperty` back to the base value, or a `RemoveElementProperty` when the
//! property did not exist; none when the element is absent.

use super::super::elements;
use super::super::remove_element_property::RemoveElementProperty;
use super::SetElementProperty;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetElementProperty, base: &ModelSnapshot) -> Vec<ModelMutation> {
    if !elements::exists(base, &payload.id) {
        return Vec::new();
    }
    let old = base.properties.get(&payload.id).and_then(|sets| sets.get(&payload.pset)).and_then(|properties| properties.get(&payload.property));
    match old {
        Some(value) => vec![ModelMutation::SetElementProperty(SetElementProperty { id: payload.id.clone(), pset: payload.pset.clone(), property: payload.property.clone(), value: value.clone() })],
        None => vec![ModelMutation::RemoveElementProperty(RemoveElementProperty { id: payload.id.clone(), pset: payload.pset.clone(), property: payload.property.clone() })],
    }
}
