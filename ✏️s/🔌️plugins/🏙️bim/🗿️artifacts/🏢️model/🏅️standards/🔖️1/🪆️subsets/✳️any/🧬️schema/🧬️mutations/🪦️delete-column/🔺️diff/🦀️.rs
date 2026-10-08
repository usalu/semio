//! 🔺️ Diff constructor for `DeleteColumn`: the column leaves in one sparse diff together with its properties and classifications (see the shared cascade).
//! The outcome carries a cascade note when more than the column leaves.

use super::super::cascade;
use super::DeleteColumn;
use crate::{ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteColumn, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.columns.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Column \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    cascade::outcome(base, std::slice::from_ref(&payload.id), "Column", Some(&payload.id))
}
