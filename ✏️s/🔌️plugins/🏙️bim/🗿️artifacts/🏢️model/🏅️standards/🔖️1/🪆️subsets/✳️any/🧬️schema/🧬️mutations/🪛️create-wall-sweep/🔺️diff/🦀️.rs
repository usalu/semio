//! 🔺️ Diff constructor for `CreateWallSweep`: one created wall sweep entry. The id is free in every collection, the host wall and the material exist, the profile has positive dimensions, the height above the wall base is not
//! negative and the inset leaves part of the profile showing. The solid, the length and the areas of the sweep are inferred.

use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::schema::mutations::wall_depth::sweep_flaw;
use crate::standards::v1::subsets::any::schema::mutations::create_wall_sweep::CreateWallSweep;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateWallSweep, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(flaw) = sweep_flaw(base, &payload.wall_sweep) {
        return flaw.under(&["wall_sweep"]).refuse();
    }
    MutationOutcome::new(ModelDiff::wall_sweeps(payload.id.clone(), Entry::Created(payload.wall_sweep.clone())))
}
