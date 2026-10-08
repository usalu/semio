//! 🔺️ Diff constructor for `CreateDoorType`: one created door type entry. The id must be free, the material must exist and every dimension must be valid.

use super::super::elements;
use super::CreateDoorType;
use crate::{is_positive_length, Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateDoorType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let record = &payload.door_type;
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if !base.materials.contains_key(&record.material) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Material \"{}\" does not exist.", record.material), ["door_type", "material"]);
    }
    let broken = [
        ("width", is_positive_length(record.width), "A door width must be a positive length."),
        ("height", is_positive_length(record.height), "A door height must be a positive length."),
        ("frame_width", is_positive_length(record.frame_width), "A door frame width must be a positive length."),
        ("frame_depth", is_positive_length(record.frame_depth), "A door frame depth must be a positive length."),
    ]
    .into_iter()
    .find_map(|(field, holds, message)| (!holds).then_some((field, message)));
    if let Some((field, message)) = broken {
        return MutationOutcome::refuse(OutcomeCode::Invariant, message, ["door_type", field]);
    }
    MutationOutcome::new(ModelDiff::door_types(payload.id.clone(), Entry::Created(record.clone())))
}
