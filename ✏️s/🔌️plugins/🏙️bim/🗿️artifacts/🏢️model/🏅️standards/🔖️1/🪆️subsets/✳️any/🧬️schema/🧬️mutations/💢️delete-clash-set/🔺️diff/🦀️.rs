//! 🔺️ Diff constructor for `DeleteClashSet`: the clash set leaves in one sparse diff together with its properties and classifications (see the shared cascade).

use super::super::cascade;
use super::DeleteClashSet;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteClashSet, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.clash_sets.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Clash set \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Clash set", Some(&payload.id))
}
