//! 🔺️ Diff constructor for `SetWallAxis`: a one-field wall patch. The axis must have finite end points, length and a real arc; hosted
//! openings are never touched, their placement along the new axis is inferred.

use crate::standards::v1::subsets::any::schema::mutations::wall_geometry::flaw;
use crate::standards::v1::subsets::any::schema::mutations::set_wall_axis::SetWallAxis;
use crate::standards::v1::subsets::any::schema::{diff::{Entry, ModelDiff, patches::WallPatch}, snapshot::ModelSnapshot};
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
