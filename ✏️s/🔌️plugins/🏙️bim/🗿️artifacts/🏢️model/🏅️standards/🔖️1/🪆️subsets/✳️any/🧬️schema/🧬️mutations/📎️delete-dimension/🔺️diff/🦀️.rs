//! 🔺️ Diff constructor for `DeleteDimension`: the dimension leaves in one sparse diff (see the shared cascade); the elements it names stay untouched.

use super::super::cascade;
use super::DeleteDimension;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteDimension, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.dimensions.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Dimension \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Dimension", Some(&payload.id))
}
