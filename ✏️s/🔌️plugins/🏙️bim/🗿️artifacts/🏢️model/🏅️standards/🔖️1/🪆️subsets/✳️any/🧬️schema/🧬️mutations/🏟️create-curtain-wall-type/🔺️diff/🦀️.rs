//! 🔺️ Diff constructor for `CreateCurtainWallType`: one created curtain wall type entry. Both grid rules are sound (a positive spacing, or explicit lines at
//! positive distances in strictly ascending order), both mullion sections have positive dimensions, the default panel names existing types or materials and both
//! materials exist.

use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::schema::mutations::wall_geometry::curtain_type_flaw;
use crate::standards::v1::subsets::any::schema::mutations::create_curtain_wall_type::CreateCurtainWallType;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateCurtainWallType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(flaw) = curtain_type_flaw(base, &payload.curtain_wall_type) {
        return flaw.under(&["curtain_wall_type"]).refuse();
    }
    MutationOutcome::new(ModelDiff::curtain_wall_types(payload.id.clone(), Entry::Created(payload.curtain_wall_type.clone())))
}
