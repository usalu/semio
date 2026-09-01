//! 🔺️ Sparse diff builder for `MoveReference`.
use super::MoveReference;
use crate::artifacts::cad::diff::apply_reference_patch;
use crate::artifacts::cad::diff::CadDiff;
use crate::artifacts::cad::mutations::CadReferencePatch;
use crate::artifacts::cad::CadSnapshot;
use std::collections::BTreeMap;

//#region 🔖️Diff
pub fn diff(payload: &MoveReference, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    let references = base.references_by_model_definition_id.get(&payload.model_definition_id).cloned().unwrap_or_default();
    let Some(existing) = references.iter().find(|reference| reference.id == payload.reference_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Reference \"{}\" does not exist.", payload.reference_id), [payload.model_definition_id.clone(), payload.reference_id.clone()]);
    };
    if payload.new_origin.iter().any(|component| !component.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Reference \"{}\" origin must be finite, got {:?}.", payload.reference_id, payload.new_origin), [payload.model_definition_id.clone(), payload.reference_id.clone()]);
    }
    if existing.origin == payload.new_origin {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Reference \"{}\" is already at {:?}.", payload.reference_id, payload.new_origin));
    }
    let patch = CadReferencePatch { origin: Some(payload.new_origin.clone()), ..Default::default() };
    let next = references
        .into_iter()
        .map(|mut reference| {
            if reference.id == payload.reference_id {
                apply_reference_patch(&mut reference, &patch);
            }
            reference
        })
        .collect();
    protocol::MutationOutcome::new(CadDiff { references_by_model_definition_id: Some(BTreeMap::from([(payload.model_definition_id.clone(), next)])), ..Default::default() })
}
//#endregion 🔖️Diff
