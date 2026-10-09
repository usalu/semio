//! 🔺️ Diff constructor for `DeleteCeilingType`: one deleted ceiling type entry together with the properties and classifications keyed by the type; refused while a ceiling still uses the type.

use super::super::cascade;
use super::DeleteCeilingType;
use crate::{Entry, KeyedDelta, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &DeleteCeilingType, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if !base.ceiling_types.contains_key(&payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetMissing, format!("Ceiling type \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    if base.ceilings.values().any(|row| row.ceiling_type == payload.id) {
        return MutationOutcome::refuse(OutcomeCode::TargetReferenced, format!("Ceiling type \"{}\" is still used by ceilings.", payload.id), [payload.id.clone()]);
    }
    let mut removal = cascade::data_diff(base, &payload.id);
    removal.ceiling_types = Some(KeyedDelta::one(payload.id.clone(), Entry::Deleted));
    MutationOutcome::new(removal)
}
