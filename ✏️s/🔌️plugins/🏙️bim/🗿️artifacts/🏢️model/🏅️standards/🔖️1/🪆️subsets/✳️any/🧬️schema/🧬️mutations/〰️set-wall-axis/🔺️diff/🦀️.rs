//! 🔺️ Diff constructor for `SetWallAxis`: a one-field wall patch. The axis must have finite end points, length and a real arc; hosted
//! openings are never touched, their placement along the new axis is inferred.

use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::schema::mutations::wall_geometry::flaw;
use crate::standards::v1::subsets::any::schema::mutations::set_wall_axis::SetWallAxis;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::building_elements::walls_and_curtain_walls::WallPatch;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetWallAxis, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(wall) = base.walls.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Wall \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(flaw) = flaw(&payload.axis) {
        return flaw.under(&["axis"]).refuse();
    }
    if wall.axis == payload.axis {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Wall \"{}\" already runs along this axis.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::walls(payload.id.clone(), Entry::Patched(WallPatch { axis: Some(payload.axis.clone()), ..Default::default() })))
}
