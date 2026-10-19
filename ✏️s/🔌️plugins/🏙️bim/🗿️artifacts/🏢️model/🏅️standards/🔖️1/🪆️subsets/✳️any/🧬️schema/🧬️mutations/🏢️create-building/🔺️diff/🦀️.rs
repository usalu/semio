//! 🔺️ Diff constructor for `CreateBuilding`: one created building entry; the site must exist, the id must be free.

use crate::standards::v1::subsets::any::schema::mutations::create_building::CreateBuilding;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateBuilding, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if !base.sites.contains_key(&payload.building.site) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Site \"{}\" does not exist.", payload.building.site), ["building", "site"]);
    }
    MutationOutcome::new(ModelDiff::buildings(payload.id.clone(), Entry::Created(payload.building.clone())))
}
