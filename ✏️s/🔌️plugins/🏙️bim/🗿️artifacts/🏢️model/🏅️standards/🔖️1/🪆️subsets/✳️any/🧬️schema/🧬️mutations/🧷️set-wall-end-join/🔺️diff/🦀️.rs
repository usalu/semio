//! 🔺️ Diff constructor for `SetWallEndJoin`: a one-field wall patch assigning (or clearing) the join preference of one end of the axis.
//! Restating the current preference is a no-op. Nothing derived is written: the wall layout of the wall and of the walls it touches
//! follows by inference.

use semio_s_artifact_bim_model_modify::standards::v1::subsets::any::schema::mutations::modify::WallEnd;
use crate::standards::v1::subsets::any::schema::mutations::set_wall_end_join::SetWallEndJoin;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::{Assigned, Entry};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::building_elements::walls_and_curtain_walls::WallPatch;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetWallEndJoin, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(wall) = base.walls.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Wall \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let current = if payload.end == WallEnd::Start { wall.start_join } else { wall.end_join };
    if current == payload.join {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("The {:?} of wall \"{}\" already joins like this.", payload.end, payload.id), [payload.id.clone()]);
    }
    let assigned = Some(Assigned::new(payload.join));
    let patch = if payload.end == WallEnd::Start { WallPatch { start_join: assigned, ..Default::default() } } else { WallPatch { end_join: assigned, ..Default::default() } };
    MutationOutcome::new(ModelDiff::walls(payload.id.clone(), Entry::Patched(patch)))
}
