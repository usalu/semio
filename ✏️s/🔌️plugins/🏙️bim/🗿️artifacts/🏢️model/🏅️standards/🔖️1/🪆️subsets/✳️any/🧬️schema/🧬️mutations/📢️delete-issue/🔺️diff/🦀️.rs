//! 🔺️ Diff constructor for `DeleteIssue`: the issue leaves in one sparse diff together with its comments, its properties and classifications (see the shared cascade).

use super::super::cascade;
use super::DeleteIssue;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteIssue, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.issues.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Issue \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Issue", Some(&payload.id))
}
