//! 🔺️ Diff constructor for `CreateMepElement`: one created MEP element entry. The id is free in every collection, the storey exists, every dimension of the section is a positive length and the path has at
//! least two finite points of which no two consecutive ones coincide. The solid, the length, the volume and the clashes are inferred, never stored.

use super::super::component_rules;
use super::super::elements;
use super::CreateMepElement;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateMepElement, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(fault) = component_rules::mep_fault(base, &payload.mep) {
        return MutationOutcome::refuse(fault.code, fault.message.clone(), fault.path(Some("mep")));
    }
    MutationOutcome::new(ModelDiff::mep_elements(payload.id.clone(), Entry::Created(payload.mep.clone())))
}
