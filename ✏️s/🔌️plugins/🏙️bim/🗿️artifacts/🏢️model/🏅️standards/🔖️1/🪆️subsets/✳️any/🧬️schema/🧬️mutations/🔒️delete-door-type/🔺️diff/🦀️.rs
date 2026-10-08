//! 🔺️ Diff constructor for `DeleteDoorType`: one deleted door type entry; refused while openings still use it.

use super::DeleteDoorType;
use crate::{Entry, ModelDiff, ModelSnapshot, OpeningKind};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteDoorType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.door_types.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Door type \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if base.openings.values().any(|row| matches!(&row.kind, OpeningKind::Door { door_type } if *door_type == payload.id)) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Door type \"{}\" is still used by openings.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::door_types(payload.id.clone(), Entry::Deleted))
}
