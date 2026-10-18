//! 🔺️ Diff constructor for `CreateBeamType`: one created beam type entry. The id must be free, the material must exist and the profile must be a valid cross-section.

use crate::standards::v1::subsets::any::schema::mutations::elements;
use crate::standards::v1::subsets::any::schema::mutations::create_beam_type::CreateBeamType;
use crate::standards::v1::subsets::any::schema::{snapshot::{profile_problem, ModelSnapshot}, diff::{Entry, ModelDiff}};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateBeamType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let record = &payload.beam_type;
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if !base.materials.contains_key(&record.material) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Material \"{}\" does not exist.", record.material), ["beam_type", "material"]);
    }
    if let Some(problem) = profile_problem(&record.profile) {
        return MutationOutcome::refuse(OutcomeCode::Invariant, problem, ["beam_type", "profile"]);
    }
    MutationOutcome::new(ModelDiff::beam_types(payload.id.clone(), Entry::Created(record.clone())))
}
