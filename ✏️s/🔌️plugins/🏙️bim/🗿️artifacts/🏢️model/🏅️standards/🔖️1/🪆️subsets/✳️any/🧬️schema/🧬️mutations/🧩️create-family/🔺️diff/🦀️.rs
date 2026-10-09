//! 🔺️ Diff constructor for `CreateFamily`: one created family entry. The id is free in every collection and the name is not blank. Parameters and solids come afterwards.

use super::super::elements;
use super::super::family_rules;
use super::CreateFamily;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateFamily, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(fault) = family_rules::family_record_fault(&payload.family) {
        return MutationOutcome::refuse(fault.code, fault.message.clone(), fault.path(Some("family")));
    }
    MutationOutcome::new(ModelDiff::families(payload.id.clone(), Entry::Created(payload.family.clone())))
}
