//! 🔺️ Diff constructor for `SetBeamAxis`: a one-field beam patch. The axis must have finite end points, length and a real arc, exactly like the axis of a wall;
//! the inferred sweep, the joins with columns and the quantities follow by inference.

use super::super::wall_geometry::flaw;
use super::SetBeamAxis;
use crate::{BeamPatch, Entry, ModelDiff, ModelSnapshot};
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
