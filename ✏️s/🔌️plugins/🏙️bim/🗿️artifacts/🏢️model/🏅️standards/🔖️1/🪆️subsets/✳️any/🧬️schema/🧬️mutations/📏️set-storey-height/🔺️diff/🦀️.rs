//! 🔺️ Diff constructor for `SetStoreyHeight`: a one-field storey patch. Nothing downstream is written: every elevation above and
//! every wall resolved by this storey follows by inference.

use crate::standards::v1::subsets::any::schema::mutations::set_storey_height::SetStoreyHeight;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::site_and_spaces::StoreyPatch;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetStoreyHeight, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(storey) = base.storeys.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !(payload.height.is_finite() && payload.height > 0.0) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "A storey height must be a positive length.", ["height"]);
    }
    if storey.height == payload.height {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Storey \"{}\" already is {} m high.", payload.id, payload.height), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::storeys(payload.id.clone(), Entry::Patched(StoreyPatch { height: Some(payload.height), ..Default::default() })))
}
