//! 🔺️ Diff constructor for `SetElementClassification`: the element's classification reference. An element without one gets it created,
//! otherwise the differing fields are patched. Refused: an unknown element and a classification without system or code.

use super::super::elements;
use super::SetElementClassification;
use crate::{ClassificationPatch, Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetElementClassification, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !elements::exists(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Element \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let next = &payload.classification;
    if next.system.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A classification needs a system.", ["classification", "system"]);
    }
    if next.code.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A classification needs a code.", ["classification", "code"]);
    }
    let entry = match base.classifications.get(&payload.id) {
        None => Entry::Created(next.clone()),
        Some(current) if current == next => {
            return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Element \"{}\" already has this classification.", payload.id), [payload.id.clone()]);
        }
        Some(current) => Entry::Patched(ClassificationPatch {
            system: (current.system != next.system).then(|| next.system.clone()),
            code: (current.code != next.code).then(|| next.code.clone()),
            title: (current.title != next.title).then(|| next.title.clone()),
        }),
    };
    MutationOutcome::new(ModelDiff::classifications(payload.id.clone(), entry))
}
