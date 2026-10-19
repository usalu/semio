//! 🔺️ Diff constructor for `CreateView`: one created view entry. The id must be free across every collection, then the view must be writable: its building exists, its
//! name is filled and unique within the building, each kind owns exactly its fields (see `view_problem`), the numbers are positive and finite and the hidden categories are
//! unique and in category order.

use crate::standards::v1::subsets::any::schema::mutations::create_view::CreateView;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{view_problem, ModelSnapshot};
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateView, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(problem) = view_problem(base, &payload.id, &payload.view) {
        let code = if problem.missing { OutcomeCode::TargetMissing } else { OutcomeCode::Invariant };
        return MutationOutcome::refuse(code, problem.message, ["view", problem.field]);
    }
    MutationOutcome::new(ModelDiff::views(payload.id.clone(), Entry::Created(payload.view.clone())))
}
