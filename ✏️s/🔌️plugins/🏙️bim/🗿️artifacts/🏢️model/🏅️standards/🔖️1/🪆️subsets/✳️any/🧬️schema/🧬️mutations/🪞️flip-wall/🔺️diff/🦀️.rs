//! 🔺️ Diff constructor for `FlipWall`: a one-field wall patch writing the reversed axis. A wall whose axis is its own reverse is a no-op.

use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::schema::mutations::wall_geometry::flipped;
use crate::standards::v1::subsets::any::schema::mutations::flip_wall::FlipWall;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::building_elements::walls_and_curtain_walls::WallPatch;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &FlipWall, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(wall) = base.walls.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Wall \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let axis = flipped(&wall.axis);
    if axis == wall.axis {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Wall \"{}\" has no direction to flip.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::walls(payload.id.clone(), Entry::Patched(WallPatch { axis: Some(axis), ..Default::default() })))
}
