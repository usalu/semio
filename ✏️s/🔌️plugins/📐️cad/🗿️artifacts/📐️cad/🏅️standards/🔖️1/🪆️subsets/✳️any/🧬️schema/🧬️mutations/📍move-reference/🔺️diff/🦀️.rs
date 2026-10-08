//! 🔺️ Sparse diff builder for `MoveReference`.
use super::MoveReference;
use crate::diff::{CadDiff, CadReferencesDelta};
use crate::mutations::CadReferencePatch;
use crate::CadSnapshot;
use std::collections::BTreeMap;

//#region 🔖️Diff
pub fn diff(payload: &MoveReference, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    let Some(existing) = base.references_by_model_definition_id.get(&payload.model_definition_id).and_then(|references| references.iter().find(|reference| reference.id == payload.reference_id)) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Reference \"{}\" does not exist.", payload.reference_id), [payload.model_definition_id.clone(), payload.reference_id.clone()]);
    };
    if payload.new_origin.iter().any(|component| !component.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Reference \"{}\" origin must be finite, got {:?}.", payload.reference_id, payload.new_origin), [payload.model_definition_id.clone(), payload.reference_id.clone()]);
    }
    if existing.origin == payload.new_origin {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Reference \"{}\" is already at {:?}.", payload.reference_id, payload.new_origin));
    }
    let patch = CadReferencePatch { origin: Some(payload.new_origin), ..Default::default() };
    protocol::MutationOutcome::new(CadDiff {
        references_by_model_definition_id: Some(BTreeMap::from([(payload.model_definition_id.clone(), CadReferencesDelta::modification(payload.reference_id.clone(), patch))])),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
