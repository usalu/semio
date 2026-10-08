//! 🔺️ Diff constructor for `DeleteSlab`: the slab leaves in one sparse diff together with its properties and classifications (see the shared cascade).
//! The outcome carries a cascade note when more than the slab leaves.

use super::super::cascade;
use super::DeleteSlab;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteSlab, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.slabs.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Slab \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Slab", Some(&payload.id))
}
