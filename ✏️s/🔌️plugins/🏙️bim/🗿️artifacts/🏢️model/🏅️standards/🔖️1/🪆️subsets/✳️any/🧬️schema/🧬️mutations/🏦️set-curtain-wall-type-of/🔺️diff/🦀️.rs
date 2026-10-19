//! 🔺️ Diff constructor for `SetCurtainWallTypeOf`: a one-field curtain wall patch. The curtain wall type must exist; grid, mullions and panels are inferred from it.

use crate::standards::v1::subsets::any::schema::mutations::set_curtain_wall_type_of::SetCurtainWallTypeOf;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::building_elements::walls_and_curtain_walls::CurtainWallPatch;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetCurtainWallTypeOf, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(wall) = base.curtain_walls.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Curtain wall \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if !base.curtain_wall_types.contains_key(&payload.curtain_wall_type) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Curtain wall type \"{}\" does not exist.", payload.curtain_wall_type), ["curtain_wall_type"]);
    }
    if wall.curtain_wall_type == payload.curtain_wall_type {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Curtain wall \"{}\" already is of type \"{}\".", payload.id, payload.curtain_wall_type), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::curtain_walls(payload.id.clone(), Entry::Patched(CurtainWallPatch { curtain_wall_type: Some(payload.curtain_wall_type.clone()), ..Default::default() })))
}
