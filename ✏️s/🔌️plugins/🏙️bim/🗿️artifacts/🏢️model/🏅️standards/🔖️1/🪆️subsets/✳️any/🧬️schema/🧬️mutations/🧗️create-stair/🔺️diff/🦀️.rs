//! 🔺️ Diff constructor for `CreateStair`: one created stair entry. The storey must exist, width, maximum riser and minimum tread must
//! be positive, the flight must be sensible and a storey top constraint must name a storey of the same building. Riser count and
//! tread depth are never stored: they are inferred from the storey levels.

use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::placement::{refuse_stair, stair_issue};
use crate::standards::v1::subsets::any::schema::mutations::create_stair::CreateStair;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateStair, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let stair = &payload.stair;
    if let Some(noun) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if !base.storeys.contains_key(&stair.storey) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{}\" does not exist.", stair.storey), ["stair", "storey"]);
    }
    if let Some(issue) = stair_issue(base, stair) {
        return refuse_stair(issue, &["stair"]);
    }
    MutationOutcome::new(ModelDiff::stairs(payload.id.clone(), Entry::Created(stair.clone())))
}
