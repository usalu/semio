//! 🔺️ Diff constructor for `DeleteWindowType`: one deleted window type entry; refused while openings still use it.

use super::DeleteWindowType;
use crate::{Entry, ModelDiff, ModelSnapshot, OpeningKind};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteWindowType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.window_types.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Window type \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if base.openings.values().any(|row| matches!(&row.kind, OpeningKind::Window { window_type } if *window_type == payload.id)) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Window type \"{}\" is still used by openings.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::window_types(payload.id.clone(), Entry::Deleted))
}
