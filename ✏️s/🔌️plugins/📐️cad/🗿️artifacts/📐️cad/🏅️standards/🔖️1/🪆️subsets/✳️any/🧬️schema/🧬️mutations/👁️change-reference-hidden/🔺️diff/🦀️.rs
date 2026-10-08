//! 🔺️ Sparse diff builder for `ChangeReferenceHidden`.
use super::ChangeReferenceHidden;
use crate::diff::{CadDiff, CadReferencesDelta};
use crate::mutations::CadReferencePatch;
use crate::CadSnapshot;
use std::collections::BTreeMap;

//#region 🔖️Diff
pub fn diff(payload: &ChangeReferenceHidden, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    let Some(existing) = base.references_by_model_definition_id.get(&payload.model_definition_id).and_then(|references| references.iter().find(|reference| reference.id == payload.reference_id)) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Reference \"{}\" does not exist.", payload.reference_id), [payload.model_definition_id.clone(), payload.reference_id.clone()]);
    };
    if existing.hidden == payload.new_hidden {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Reference \"{}\" already has hidden = {}.", payload.reference_id, payload.new_hidden));
    }
    let patch = CadReferencePatch { hidden: Some(payload.new_hidden), ..Default::default() };
    protocol::MutationOutcome::new(CadDiff {
        references_by_model_definition_id: Some(BTreeMap::from([(payload.model_definition_id.clone(), CadReferencesDelta::modification(payload.reference_id.clone(), patch))])),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
