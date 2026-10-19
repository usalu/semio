//! 🔺️ Diff constructor for `CreateSheet`: one created sheet entry. The id must be free across every collection, then the sheet must be writable (see `sheet_problem`).

use crate::standards::v1::subsets::any::schema::mutations::create_sheet::CreateSheet;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{sheet_problem, ModelSnapshot};
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateSheet, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(problem) = sheet_problem(base, &payload.id, &payload.sheet) {
        let code = if problem.missing { OutcomeCode::TargetMissing } else { OutcomeCode::Invariant };
        return MutationOutcome::refuse(code, problem.message, ["sheet", problem.field]);
    }
    MutationOutcome::new(ModelDiff::sheets(payload.id.clone(), Entry::Created(payload.sheet.clone())))
}
