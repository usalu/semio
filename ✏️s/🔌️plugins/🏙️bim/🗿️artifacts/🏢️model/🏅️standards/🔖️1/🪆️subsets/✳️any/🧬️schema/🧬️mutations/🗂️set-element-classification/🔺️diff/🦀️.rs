//! 🔺️ Diff constructor for `SetElementClassification`: the code of an element or type in one classification system. A holder without classifications gets its first created, otherwise the system's code is patched in.
//! Refused: an unknown holder, an unknown system and a blank code; a code outside the system's table is allowed (a diagnostic reports it), so tables can change without breaking the elements.

use super::super::elements;
use super::SetElementClassification;
use crate::{ClassificationSet, ClassificationSetPatch, Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};
use std::collections::BTreeMap;

pub fn diff(payload: &SetElementClassification, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !elements::holds_data(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Element \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if !base.classification_systems.contains_key(&payload.system) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Classification system \"{}\" does not exist.", payload.system), ["system"]);
    }
    if payload.code.trim().is_empty() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A classification needs a code.", ["code"]);
    }
    let entry = match base.classifications.get(&payload.id) {
        None => Entry::Created(ClassificationSet::from([(payload.system.clone(), payload.code.clone())])),
        Some(set) if set.get(&payload.system) == Some(&payload.code) => {
            return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Element \"{}\" already has this classification.", payload.id), [payload.id.clone()]);
        }
        Some(_) => Entry::Patched(ClassificationSetPatch { assigned: BTreeMap::from([(payload.system.clone(), Some(payload.code.clone()))]) }),
    };
    MutationOutcome::new(ModelDiff::classifications(payload.id.clone(), entry))
}
