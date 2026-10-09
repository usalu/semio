//! 🔺️ Diff constructor for `CreateSheetRevision`: one created revision entry. The id must be free across every collection, then the revision must be writable (see `revision_problem`).

use super::super::elements;
use super::CreateSheetRevision;
use crate::{revision_problem, Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateSheetRevision, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(problem) = revision_problem(base, &payload.id, &payload.sheet_revision) {
        let code = if problem.missing { OutcomeCode::TargetMissing } else { OutcomeCode::Invariant };
        return MutationOutcome::refuse(code, problem.message, ["sheet_revision", problem.field]);
    }
    MutationOutcome::new(ModelDiff::sheet_revisions(payload.id.clone(), Entry::Created(payload.sheet_revision.clone())))
}
