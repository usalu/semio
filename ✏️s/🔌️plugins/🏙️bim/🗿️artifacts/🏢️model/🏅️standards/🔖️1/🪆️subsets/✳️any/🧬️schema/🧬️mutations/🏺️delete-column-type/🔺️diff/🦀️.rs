//! 🔺️ Diff constructor for `DeleteColumnType`: one deleted column type entry; refused while columns still use it.

use super::DeleteColumnType;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteColumnType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.column_types.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Column type \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if base.columns.values().any(|row| row.column_type == payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Column type \"{}\" is still used by columns.", payload.id), [payload.id.clone()]);
    }
    MutationOutcome::new(ModelDiff::column_types(payload.id.clone(), Entry::Deleted))
}
