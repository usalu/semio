//! 🔺️ Diff constructor for `DeleteView`: the view leaves in one sparse diff together with its properties and classifications (see the shared cascade).
//! The outcome carries a cascade note when more than the view leaves.

use super::super::cascade;
use super::DeleteView;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteView, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.views.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("View \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "View", Some(&payload.id))
}
