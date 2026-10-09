//! \u{1f53a}\uFE0F Diff constructor for \u00B6DeleteSchedule\u00B6: the schedule leaves in one sparse diff. Its rows and totals are inferred and nothing else refers to it, so nothing cascades.

use super::DeleteSchedule;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteSchedule, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.schedules.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Schedule \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::schedules(payload.id.clone(), Entry::Deleted))
}
