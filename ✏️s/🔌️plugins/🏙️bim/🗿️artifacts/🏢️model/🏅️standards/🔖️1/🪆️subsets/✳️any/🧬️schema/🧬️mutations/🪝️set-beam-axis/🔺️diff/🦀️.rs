//! 🔺️ Diff constructor for `SetBeamAxis`: a one-field beam patch. The axis must have finite end points, length and a real arc, exactly like the axis of a wall;
//! the inferred sweep, the joins with columns and the quantities follow by inference.

use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::schema::mutations::wall_geometry::flaw;
use crate::standards::v1::subsets::any::schema::mutations::set_beam_axis::SetBeamAxis;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::building_elements::columns_beams_slabs::BeamPatch;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetBeamAxis, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(beam) = base.beams.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Beam \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(flaw) = flaw(&payload.axis) {
        return flaw.under(&["axis"]).refuse();
    }
    if beam.axis == payload.axis {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Beam \"{}\" already runs along this axis.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::beams(payload.id.clone(), Entry::Patched(BeamPatch { axis: Some(payload.axis.clone()), ..Default::default() })))
}
