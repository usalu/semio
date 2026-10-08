//! 🔺️ Diff constructor for `CreateWindowType`: one created window type entry. The id must be free, the material must exist and every dimension must be valid.

use super::super::elements;
use super::CreateWindowType;
use crate::{is_non_negative_length, is_positive_length, Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateWindowType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    let record = &payload.window_type;
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if !base.materials.contains_key(&record.material) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Material \"{}\" does not exist.", record.material), ["window_type", "material"]);
    }
    let broken = [
        ("width", is_positive_length(record.width), "A window width must be a positive length."),
        ("height", is_positive_length(record.height), "A window height must be a positive length."),
        ("sill", is_non_negative_length(record.sill), "A window sill must be zero or more."),
        ("frame_width", is_positive_length(record.frame_width), "A window frame width must be a positive length."),
        ("frame_depth", is_positive_length(record.frame_depth), "A window frame depth must be a positive length."),
        ("panes", record.panes >= 1, "A window needs at least one pane."),
    ]
    .into_iter()
    .find_map(|(field, holds, message)| (!holds).then_some((field, message)));
    if let Some((field, message)) = broken {
        return MutationOutcome::refuse(OutcomeCode::Invariant, message, ["window_type", field]);
    }
    MutationOutcome::new(ModelDiff::window_types(payload.id.clone(), Entry::Created(record.clone())))
}
