//! 🔺️ Diff constructor for `DeleteViewport`: the viewport leaves in one sparse diff together with its properties and classifications (see the shared cascade). The outcome carries a cascade note when more than the
//! viewport leaves.

use super::super::cascade;
use super::DeleteViewport;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteViewport, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.viewports.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Viewport \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Viewport", Some(&payload.id))
}
