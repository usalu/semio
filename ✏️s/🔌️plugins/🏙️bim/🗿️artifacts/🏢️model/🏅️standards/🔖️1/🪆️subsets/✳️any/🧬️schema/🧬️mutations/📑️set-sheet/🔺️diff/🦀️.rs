//! 🔺️ Diff constructor for `SetSheet`: a sparse sheet patch of exactly the provided fields that differ. The sheet that results must be writable (see `sheet_problem`); a patch that restates the current values is a
//! `mutation.no-op`.

use super::SetSheet;
use crate::{sheet_problem, Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetSheet, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.sheets.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Sheet \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let patch = payload.patch().minimal(record);
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Sheet \"{}\" already holds these values.", payload.id), [payload.id.clone()]);
    }
    if let Some(problem) = sheet_problem(base, &payload.id, &patch.write(record)) {
        let code = if problem.missing { OutcomeCode::TargetMissing } else { OutcomeCode::Invariant };
        return MutationOutcome::refuse(code, problem.message, [problem.field]);
    }
    MutationOutcome::new(ModelDiff::sheets(payload.id.clone(), Entry::Patched(patch)))
}
