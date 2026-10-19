//! 🔺️ Diff constructor for `OffsetWall`: one created wall, a copy of the original with its axis replaced by the parallel axis at the
//! signed distance to the left of the direction (a shifted line, or a concentric arc of the same sweep). Hosted openings, properties and
//! classifications stay with the original. Refused: an unknown wall, a taken new id, a distance that is not finite or has no effect and an
//! offset that collapses an arc into its centre.

use semio_s_artifact_bim_model_modify::standards::v1::subsets::any::schema::mutations::modify::cut::offset_axis;
use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::schema::mutations::wall_geometry::flaw;
use crate::standards::v1::subsets::any::schema::mutations::offset_wall::OffsetWall;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{ModelSnapshot, Wall};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &OffsetWall, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(wall) = base.walls.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Wall \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(noun) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.new_id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.new_id), [payload.new_id.clone()]);
    }
    if !(payload.distance.is_finite() && payload.distance.abs() > 1e-9) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "An offset needs a finite distance of at least a nanometre.", ["distance"]);
    }
    let Some(axis) = offset_axis(&wall.axis, payload.distance) else {
        return MutationOutcome::refuse(OutcomeCode::Invariant, "The offset curve of the wall collapses.", ["distance"]);
    };
    if let Some(flaw) = flaw(&axis) {
        return flaw.under(&["distance"]).refuse();
    }
    MutationOutcome::new(ModelDiff::walls(payload.new_id.clone(), Entry::Created(Wall { axis, ..wall.clone() })))
}
