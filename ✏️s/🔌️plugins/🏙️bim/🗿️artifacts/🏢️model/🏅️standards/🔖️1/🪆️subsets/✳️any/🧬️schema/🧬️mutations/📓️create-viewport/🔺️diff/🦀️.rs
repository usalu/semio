//! 🔺️ Diff constructor for `CreateViewport`: one created viewport entry. The id must be free across every collection, then the viewport must be writable (see `viewport_problem`).

use crate::standards::v1::subsets::any::schema::mutations::create_viewport::CreateViewport;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{viewport_problem, ModelSnapshot};
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateViewport, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(problem) = viewport_problem(base, &payload.id, &payload.viewport) {
        let code = if problem.missing { OutcomeCode::TargetMissing } else { OutcomeCode::Invariant };
        return MutationOutcome::refuse(code, problem.message, ["viewport", problem.field]);
    }
    MutationOutcome::new(ModelDiff::viewports(payload.id.clone(), Entry::Created(payload.viewport.clone())))
}
