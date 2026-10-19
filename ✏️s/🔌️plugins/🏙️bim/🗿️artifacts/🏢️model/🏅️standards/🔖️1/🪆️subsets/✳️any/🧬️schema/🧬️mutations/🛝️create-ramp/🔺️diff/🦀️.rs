//! 🔺️ Diff constructor for `CreateRamp`: one created ramp entry. The storey and the material must exist, the path needs two distinct finite vertices, width, thickness and
//! slope limit are positive, the landings are non-negative, and a storey top constraint must name a storey of the same building. Length, rise, slope, landings, solid and
//! compliance are never stored: they are inferred.

use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::placement::{ramp_issue, refuse_stair};
use crate::standards::v1::subsets::any::schema::mutations::create_ramp::CreateRamp;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateRamp, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let ramp = &payload.ramp;
    if let Some(noun) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if !base.storeys.contains_key(&ramp.storey) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{}\" does not exist.", ramp.storey), ["ramp", "storey"]);
    }
    if let Some(issue) = ramp_issue(base, ramp) {
        return refuse_stair(issue, &["ramp"]);
    }
    MutationOutcome::new(ModelDiff::ramps(payload.id.clone(), Entry::Created(ramp.clone())))
}
