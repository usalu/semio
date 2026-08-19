//! 🔺️ Sparse diff builder for `ReplaceReferences`.
use super::mutation::ReplaceReferences;
use crate::artifacts::cad::diff::CadDiff;
use crate::artifacts::cad::CadSnapshot;
use std::collections::BTreeMap;

//#region 🔖️Diff
pub async fn diff(payload: &ReplaceReferences, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    let existing = base.references_by_model_definition_id.get(&payload.model_definition_id);
    if existing == Some(&payload.references) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("References for \"{}\" are already up to date.", payload.model_definition_id));
    }
    protocol::MutationOutcome::new(CadDiff { references_by_model_definition_id: Some(BTreeMap::from([(payload.model_definition_id.clone(), payload.references.clone())])), ..Default::default() })
}
//#endregion 🔖️Diff
