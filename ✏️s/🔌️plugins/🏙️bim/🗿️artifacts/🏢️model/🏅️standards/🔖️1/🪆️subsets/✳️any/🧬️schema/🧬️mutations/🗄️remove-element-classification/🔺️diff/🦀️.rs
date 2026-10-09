//! 🔺️ Diff constructor for `RemoveElementClassification`: the classification of a holder in one system leaves; when it was the holder's last the whole entry is deleted so that no empty classification set remains.
//! Refused: an unknown holder or a holder without a classification in that system.

use super::super::elements;
use super::RemoveElementClassification;
use crate::{ClassificationSetPatch, Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};
use std::collections::BTreeMap;

pub fn diff(payload: &RemoveElementClassification, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !elements::holds_data(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Element \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let Some(set) = base.classifications.get(&payload.id).filter(|set| set.contains_key(&payload.system)) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Element \"{}\" has no classification in \"{}\".", payload.id, payload.system), [payload.id.clone(), payload.system.clone()]);
    };
    let entry = if set.len() == 1 { Entry::Deleted } else { Entry::Patched(ClassificationSetPatch { assigned: BTreeMap::from([(payload.system.clone(), None)]) }) };
    MutationOutcome::new(ModelDiff::classifications(payload.id.clone(), entry))
}
