//! 🔺️ Diff constructor for `CreateClashSet`: one created clash set entry. The id must be free across every collection, then the clash set must be writable (see `clash_set_problem`).

use crate::standards::v1::subsets::any::schema::mutations::create_clash_set::CreateClashSet;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{clash_set_problem, ModelSnapshot};
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateClashSet, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(problem) = clash_set_problem(base, &payload.id, &payload.clash_set) {
        let code = if problem.missing { OutcomeCode::TargetMissing } else { OutcomeCode::Invariant };
        return MutationOutcome::refuse(code, problem.message, ["clash_set", problem.field]);
    }
    MutationOutcome::new(ModelDiff::clash_sets(payload.id.clone(), Entry::Created(payload.clash_set.clone())))
}
