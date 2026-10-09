//! 🔺️ Diff constructor for `CreateFamilySolid`: one created solid entry. The id is free in every collection, the family exists, the name is not blank, every formula slot parses and uses only
//! existing parameters of the family, and a polygon has three points and a sweep path two. The mesh, the volume and the issues of the solid are inferred, never stored.

use super::super::elements;
use super::super::family_rules;
use super::CreateFamilySolid;
use crate::{Entry, ModelDiff, ModelSnapshot};
use protocol::{MutationOutcome, OutcomeCode};

pub fn diff(payload: &CreateFamilySolid, base: &ModelSnapshot) -> MutationOutcome<ModelDiff> {
    if let Some(noun) = elements::taken(base, &payload.id) {
        return MutationOutcome::refuse(OutcomeCode::DuplicateId, format!("{noun} \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    if let Some(fault) = family_rules::solid_fault(base, &payload.solid) {
        return MutationOutcome::refuse(fault.code, fault.message.clone(), fault.path(Some("solid")));
    }
    MutationOutcome::new(ModelDiff::family_solids(payload.id.clone(), Entry::Created(payload.solid.clone())))
}
