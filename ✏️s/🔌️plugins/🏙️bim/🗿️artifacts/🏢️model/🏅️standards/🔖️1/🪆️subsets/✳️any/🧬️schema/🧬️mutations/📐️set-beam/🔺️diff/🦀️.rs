//! 🔺️ Diff constructor for `SetBeam`: a sparse beam patch naming only the fields that really change. The beam type must exist and both top offsets must be finite
//! (the end offset is assigned: none makes the beam level again). The axis is the business of `set-beam-axis`; no elevation is written, it is inferred.

use semio_s_artifact_bim_model_wall_geometry::standards::v1::subsets::any::schema::mutations::wall_geometry::offsets_flaw;
use crate::standards::v1::subsets::any::schema::mutations::set_beam::SetBeam;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::{Entry, Patch};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetBeam, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(beam) = base.beams.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Beam \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(kind) = &payload.beam_type {
        if !base.beam_types.contains_key(kind) {
            return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Beam type \"{kind}\" does not exist."), ["beam_type"]);
        }
    }
    let end = payload.end_top_offset.as_ref().and_then(|assigned| assigned.value);
    if let Some(flaw) = offsets_flaw(payload.top_offset.unwrap_or(beam.top_offset), end) {
        return flaw.refuse();
    }
    let patch = payload.patch().minimal(beam);
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Beam \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::beams(payload.id.clone(), Entry::Patched(patch)))
}
