//! 🔺️ Diff constructor for `SetWallLocation`: a one-field wall patch. The offset curves of the wall faces are inferred from it.

use crate::standards::v1::subsets::any::schema::mutations::set_wall_location::SetWallLocation;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::building_elements::walls_and_curtain_walls::WallPatch;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetWallLocation, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(wall) = base.walls.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Wall \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if wall.location == payload.location {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Wall \"{}\" already is located on this line.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::walls(payload.id.clone(), Entry::Patched(WallPatch { location: Some(payload.location), ..Default::default() })))
}
