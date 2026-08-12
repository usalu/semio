//! 🔺️ Sparse diff builder for `ChangeReferenceLocked`.
use super::mutation::ChangeReferenceLocked;
use crate::artifacts::cad::diff::CadDiff;
use crate::artifacts::cad::diff::apply_reference_patch;
use crate::artifacts::cad::mutations::CadReferencePatch;
use crate::artifacts::cad::CadSnapshot;
use std::collections::BTreeMap;

//#region 🔖️Diff
pub fn diff(payload: &ChangeReferenceLocked, base: &CadSnapshot) -> CadDiff {
    let references = base.references_by_model_definition_id.get(&payload.model_definition_id).cloned().unwrap_or_default();
    let patch = CadReferencePatch { locked: Some(payload.new_locked.clone()), ..Default::default() };
    let next = references
        .into_iter()
        .map(|mut reference| {
            if reference.id == payload.reference_id {
                apply_reference_patch(&mut reference, &patch);
            }
            reference
        })
        .collect();
    CadDiff { references_by_model_definition_id: Some(BTreeMap::from([(payload.model_definition_id.clone(), next)])), ..Default::default() }
}
//#endregion 🔖️Diff
