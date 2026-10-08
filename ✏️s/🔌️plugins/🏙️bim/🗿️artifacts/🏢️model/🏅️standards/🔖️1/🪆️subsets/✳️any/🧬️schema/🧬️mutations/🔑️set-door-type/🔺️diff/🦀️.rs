//! 🔺️ Diff constructor for `SetDoorType`: a sparse door type patch of exactly the provided fields. A provided material
//! must exist and every provided value must keep the dimensions valid; providing nothing new is a no-op.

use super::SetDoorType;
use crate::{is_positive_length, Entry, ModelDiff, ModelSnapshot, Patch};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &SetDoorType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let Some(record) = base.door_types.get(&payload.id) else {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Door type \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if let Some(material) = payload.material.as_ref().filter(|material| !base.materials.contains_key(*material)) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Material \"{material}\" does not exist."), ["material"]);
    }
    let broken = [
        ("width", payload.width.is_none_or(is_positive_length), "A door width must be a positive length."),
        ("height", payload.height.is_none_or(is_positive_length), "A door height must be a positive length."),
        ("frame_width", payload.frame_width.is_none_or(is_positive_length), "A door frame width must be a positive length."),
        ("frame_depth", payload.frame_depth.is_none_or(is_positive_length), "A door frame depth must be a positive length."),
    ]
    .into_iter()
    .find_map(|(field, holds, message)| (!holds).then_some((field, message)));
    if let Some((field, message)) = broken {
        return MutationOutcome::refuse(OutcomeCode::Invariant, message, [field]);
    }
    let patch = payload.patch().minimal(record);
    if patch.is_empty() {
        return MutationOutcome::refuse(OutcomeCode::NoOp, format!("Door type \"{}\" already has these values.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::door_types(payload.id.clone(), Entry::Patched(patch)))
}
