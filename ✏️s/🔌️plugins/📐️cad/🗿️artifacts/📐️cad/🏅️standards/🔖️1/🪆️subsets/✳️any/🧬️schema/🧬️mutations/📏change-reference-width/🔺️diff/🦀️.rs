//! 🔺️ Sparse diff builder for `ChangeReferenceWidth`.
use super::ChangeReferenceWidth;
use crate::diff::{CadDiff, CadReferencesDelta};
use crate::mutations::CadReferencePatch;
use crate::CadSnapshot;
use std::collections::BTreeMap;

//#region 🔖️Diff
pub fn diff(payload: &ChangeReferenceWidth, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    let Some(existing) = base.references_by_model_definition_id.get(&payload.model_definition_id).and_then(|references| references.iter().find(|reference| reference.id == payload.reference_id)) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Reference \"{}\" does not exist.", payload.reference_id), [payload.model_definition_id.clone(), payload.reference_id.clone()]);
    };
    if !payload.new_width_world.is_finite() || payload.new_width_world <= 0.0 {
        return protocol::MutationOutcome::fatal(
            "mutation.invariant",
            format!("Reference \"{}\" width must be finite and positive, got {}.", payload.reference_id, payload.new_width_world),
            [payload.model_definition_id.clone(), payload.reference_id.clone()],
        );
    }
    if existing.width_world == payload.new_width_world {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Reference \"{}\" already has width {}.", payload.reference_id, payload.new_width_world));
    }
    let patch = CadReferencePatch { width_world: Some(payload.new_width_world), ..Default::default() };
    protocol::MutationOutcome::new(CadDiff {
        references_by_model_definition_id: Some(BTreeMap::from([(payload.model_definition_id.clone(), CadReferencesDelta::modification(payload.reference_id.clone(), patch))])),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
