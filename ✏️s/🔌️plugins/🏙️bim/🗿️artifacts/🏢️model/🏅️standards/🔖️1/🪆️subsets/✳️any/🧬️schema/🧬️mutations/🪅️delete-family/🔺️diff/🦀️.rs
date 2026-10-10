//! 🔺️ Diff constructor for `DeleteFamily`: the family, its parameters and its solids leave in one sparse diff, together with the properties and classifications keyed by the family. A family a column type,
//! beam type, curtain wall type or railing section uses as its profile cannot go: refused as `mutation.target-referenced`.

use super::super::cascade;
use super::super::family_rules;
use super::DeleteFamily;
use crate::{Entry, KeyedDelta, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteFamily, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.families.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Family \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if let Some(noun) = family_rules::profile_user(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Family \"{}\" is still the profile of {noun}.", payload.id), [payload.id.clone()]);
    }
    if let Some(noun) = family_rules::component_user(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Family \"{}\" is still placed by {noun}.", payload.id), [payload.id.clone()]);
    }
    let parameters: Vec<String> = base.family_parameters.iter().filter(|(_, row)| row.family == payload.id).map(|(key, _)| key.clone()).collect();
    let solids: Vec<String> = base.family_solids.iter().filter(|(_, row)| row.family == payload.id).map(|(id, _)| id.clone()).collect();
    let removed = parameters.len() + solids.len();
    let mut removal = cascade::data_diff(base, &payload.id);
    removal.families = Some(KeyedDelta::one(payload.id.clone(), Entry::Deleted));
    removal.family_parameters = (!parameters.is_empty()).then(|| KeyedDelta(parameters.into_iter().map(|key| (key, Entry::Deleted)).collect()));
    removal.family_solids = (!solids.is_empty()).then(|| KeyedDelta(solids.into_iter().map(|id| (id, Entry::Deleted)).collect()));
    let outcome = MutationOutcome::new(removal);
    if removed == 0 {
        outcome
    } else {
        outcome.info(OutcomeCode::Cascade, format!("Family \"{}\" took {removed} parameter(s) and solid(s) with it.", payload.id))
    }
}
