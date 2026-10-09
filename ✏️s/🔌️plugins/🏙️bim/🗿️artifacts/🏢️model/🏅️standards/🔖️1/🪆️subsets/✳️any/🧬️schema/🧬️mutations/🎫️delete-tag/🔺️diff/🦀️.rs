//! 🔺️ Diff constructor for `DeleteTag`: the tag leaves in one sparse diff (see the shared cascade); the elements it names stay untouched.

use super::super::cascade;
use super::DeleteTag;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteTag, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.tags.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Tag \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Tag", Some(&payload.id))
}
