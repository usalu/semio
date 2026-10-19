//! 🔺️ Diff constructor for `CreateBeam`: one created beam entry. The storey and the beam type must exist, the axis (a line or an arc, like the axis of a wall) must have length and
//! a real sweep, and both top offsets must be finite. The beam top lies the signed `top_offset` above (positive) or below (negative) the storey top at the start and the
//! `end_top_offset` at the end: no elevation is stored, it is inferred.

use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::schema::mutations::wall_geometry::{flaw, offsets_flaw};
use crate::standards::v1::subsets::any::schema::mutations::create_beam::CreateBeam;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Entry;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateBeam, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let beam = &payload.beam;
    if let Some(noun) = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if !base.storeys.contains_key(&beam.storey) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Storey \"{}\" does not exist.", beam.storey), ["beam", "storey"]);
    }
    if !base.beam_types.contains_key(&beam.beam_type) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Beam type \"{}\" does not exist.", beam.beam_type), ["beam", "beam_type"]);
    }
    if let Some(flaw) = flaw(&beam.axis) {
        return flaw.under(&["beam", "axis"]).refuse();
    }
    if let Some(flaw) = offsets_flaw(beam.top_offset, beam.end_top_offset) {
        return flaw.under(&["beam"]).refuse();
    }
    MutationOutcome::new(ModelDiff::beams(payload.id.clone(), Entry::Created(beam.clone())))
}
