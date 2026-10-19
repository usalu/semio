//! 🔺️ Diff constructor for `SetSheetRevision`: a sparse revision patch of exactly the provided fields that differ. The revision that results must be writable (see `revision_problem`); a patch that restates the current values is a
//! `mutation.no-op`.

use crate::standards::v1::subsets::any::schema::mutations::set_sheet_revision::SetSheetRevision;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{revision_problem, ModelSnapshot};
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::{Entry, Patch};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetSheetRevision, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.sheet_revisions.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Revision \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let patch = payload.patch().minimal(record);
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Revision \"{}\" already holds these values.", payload.id), [payload.id.clone()]);
    }
    if let Some(problem) = revision_problem(base, &payload.id, &patch.write(record)) {
        let code = if problem.missing { OutcomeCode::TargetMissing } else { OutcomeCode::Invariant };
        return MutationOutcome::refuse(code, problem.message, [problem.field]);
    }
    MutationOutcome::new(ModelDiff::sheet_revisions(payload.id.clone(), Entry::Patched(patch)))
}
