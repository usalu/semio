//! 🔺️ Diff constructor for `DeleteRoof`: the roof leaves in one sparse diff together with its properties and classifications (see the shared cascade).
//! The outcome carries a cascade note when more than the roof leaves.

use super::super::cascade;
use super::DeleteRoof;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteRoof, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.roofs.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Roof \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Roof", Some(&payload.id))
}
