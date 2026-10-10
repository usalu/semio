//! 🔺️ Diff constructor for `DeleteRule`: the rule leaves in one sparse diff together with its properties and classifications (see the shared cascade).

use super::super::cascade;
use super::DeleteRule;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteRule, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.rules.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Rule \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Rule", Some(&payload.id))
}
