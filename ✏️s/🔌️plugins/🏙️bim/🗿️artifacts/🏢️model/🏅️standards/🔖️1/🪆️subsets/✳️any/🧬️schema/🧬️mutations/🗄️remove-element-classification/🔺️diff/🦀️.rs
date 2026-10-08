//! 🔺️ Diff constructor for `RemoveElementClassification`: one deleted classification entry. Refused: an unknown element or an element
//! without a classification.

use super::super::elements;
use super::RemoveElementClassification;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &RemoveElementClassification, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !elements::exists(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Element \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if !base.classifications.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Element \"{}\" has no classification.", payload.id), [payload.id.clone(), "classification".to_string()]);
    }
    MutationOutcome::new(ModelDiff::classifications(payload.id.clone(), Entry::Deleted))
}
