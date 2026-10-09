//! 🔺️ Diff constructor for `DeleteClassificationSystem`: the system leaves together with every classification that names it: an element or type that has no other classification loses its whole entry, one that has
//! others is patched to no code in this system. The cascade is restorable (one `set-element-classification` per removed classification), so it never refuses except for more rows than one removal restores.

use super::super::cascade;
use super::DeleteClassificationSystem;
use crate::{ClassificationSetPatch, Entry, KeyedDelta, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};
use std::collections::BTreeMap;

pub fn diff(payload: &DeleteClassificationSystem, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.classification_systems.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Classification system \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let holders = cascade::classified_by(base, &payload.id);
    if holders.len() > cascade::INVERSE_ROWS {
        return MutationOutcome::refuse(OutcomeCode::InverseRefused, format!("Classification system \"{}\" classifies {} records where one removal restores at most {}; reclassify in parts.", payload.id, holders.len(), cascade::INVERSE_ROWS), [payload.id.clone()]);
    }
    let classifications: BTreeMap<String, Entry<crate::ClassificationSet, ClassificationSetPatch>> = holders
        .iter()
        .map(|(holder, _)| {
            let last = base.classifications.get(holder).is_some_and(|set| set.len() == 1);
            let entry = if last { Entry::Deleted } else { Entry::Patched(ClassificationSetPatch { assigned: BTreeMap::from([(payload.id.clone(), None)]) }) };
            (holder.clone(), entry)
        })
        .collect();
    let outcome = MutationOutcome::new(ModelDiff {
        classification_systems: Some(KeyedDelta::one(payload.id.clone(), Entry::Deleted)),
        classifications: (!classifications.is_empty()).then(|| KeyedDelta(classifications)),
        ..ModelDiff::default()
    });
    if holders.is_empty() {
        outcome
    } else {
        outcome.info(OutcomeCode::Cascade, format!("Classification system \"{}\" took {} classification(s) with it.", payload.id, holders.len()))
    }
}
