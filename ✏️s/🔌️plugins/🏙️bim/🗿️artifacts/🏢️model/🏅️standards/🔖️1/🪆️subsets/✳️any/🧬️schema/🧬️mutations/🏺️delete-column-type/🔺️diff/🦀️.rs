//! 🔺️ Diff constructor for `DeleteColumnType`: one deleted column type entry together with the properties and classifications keyed by the type; refused while columns still use it.

use super::super::cascade;
use super::DeleteColumnType;
use crate::{Entry, KeyedDelta, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteColumnType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.column_types.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Column type \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if base.columns.values().any(|row| row.column_type == payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Column type \"{}\" is still used by columns.", payload.id), [payload.id.clone()]);
    }
    let mut removal = cascade::data_diff(base, &payload.id);
    removal.column_types = Some(KeyedDelta::one(payload.id.clone(), Entry::Deleted));
    MutationOutcome::new(removal)
}
