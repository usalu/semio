//! 🔺️ Diff constructor for `SetElementProperty`: one typed property of one element. An element without any property is created in the
//! properties collection, otherwise its property set is patched. Refused: an unknown element, an empty set or property name, and a value
//! whose measure is not finite. The value type itself makes a kind mismatch unrepresentable.

use super::super::elements;
use super::SetElementProperty;
use crate::{Entry, ModelDiff, ModelSnapshot, PropertySet, PropertySetPatch, PropertyValue};
use protocol::{MutationOutcome, OutcomeCode};
use std::collections::BTreeMap;

fn finite(value: &PropertyValue) -> bool {
    match value {
        PropertyValue::Real { value } | PropertyValue::Length { value } | PropertyValue::Area { value } | PropertyValue::Volume { value } | PropertyValue::Angle { value } => value.is_finite(),
        PropertyValue::Text { .. } | PropertyValue::Integer { .. } | PropertyValue::Boolean { .. } => true,
    }
}

pub fn diff(payload: &SetElementProperty, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !elements::holds_data(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Element \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if payload.pset.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A property set needs a name.", ["pset"]);
    }
    if payload.property.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A property needs a name.", ["property"]);
    }
    if !finite(&payload.value) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A property measure must be finite.", ["value"]);
    }
    let entry = base.properties.get(&payload.id);
    if entry.and_then(|sets| sets.get(&payload.pset)).and_then(|properties| properties.get(&payload.property)) == Some(&payload.value) {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Property \"{}\" of \"{}\" already has this value.", payload.property, payload.id), [payload.id.clone()]);
    }
    let diff = match entry {
        Some(_) => Entry::Patched(PropertySetPatch { assigned: BTreeMap::from([(payload.pset.clone(), BTreeMap::from([(payload.property.clone(), Some(payload.value.clone()))]))]) }),
        None => Entry::Created(PropertySet::from([(payload.pset.clone(), BTreeMap::from([(payload.property.clone(), payload.value.clone())]))])),
    };
    MutationOutcome::new(ModelDiff::properties(payload.id.clone(), diff))
}
