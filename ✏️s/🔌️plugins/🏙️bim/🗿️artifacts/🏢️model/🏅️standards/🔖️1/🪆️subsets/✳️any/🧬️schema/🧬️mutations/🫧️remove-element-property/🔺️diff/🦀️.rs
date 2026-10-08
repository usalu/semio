//! 🔺️ Diff constructor for `RemoveElementProperty`: one property of one element leaves; when it was the element's last property the
//! whole entry is deleted so that no empty property set remains. Refused: an unknown element or a property the element does not have.

use super::super::elements;
use super::RemoveElementProperty;
use crate::{Entry, ModelDiff, ModelSnapshot, PropertySetPatch};
use protocol::{MutationOutcome, OutcomeCode};
use std::collections::BTreeMap;

pub fn diff(payload: &RemoveElementProperty, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !elements::exists(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Element \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let held = base.properties.get(&payload.id).filter(|sets| sets.get(&payload.pset).is_some_and(|properties| properties.contains_key(&payload.property)));
    let Some(sets) = held else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Element \"{}\" has no property \"{}\" in \"{}\".", payload.id, payload.property, payload.pset), [payload.id.clone(), payload.pset.clone(), payload.property.clone()]);
    };
    let last = sets.len() == 1 && sets[&payload.pset].len() == 1;
    let entry = if last { Entry::Deleted } else { Entry::Patched(PropertySetPatch { assigned: BTreeMap::from([(payload.pset.clone(), BTreeMap::from([(payload.property.clone(), None)]))]) }) };
    MutationOutcome::new(ModelDiff::properties(payload.id.clone(), entry))
}
