//! 🔺️ Diff constructor for `DeleteCeiling`: the ceiling leaves in one sparse diff together with its properties and classifications (see the shared cascade).
//! The outcome carries a cascade note when more than the ceiling leaves.

use super::super::cascade;
use super::DeleteCeiling;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteCeiling, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.ceilings.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Ceiling \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Ceiling", Some(&payload.id))
}
