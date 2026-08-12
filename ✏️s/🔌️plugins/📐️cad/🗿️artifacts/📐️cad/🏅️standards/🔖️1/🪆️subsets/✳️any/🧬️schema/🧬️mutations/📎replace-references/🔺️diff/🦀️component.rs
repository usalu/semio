//! 🔺️ Sparse diff builder for `ReplaceReferences`.
use super::mutation::ReplaceReferences;
use crate::artifacts::cad::diff::CadDiff;
use crate::artifacts::cad::CadSnapshot;
use std::collections::BTreeMap;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceReferences, _base: &CadSnapshot) -> CadDiff {
    CadDiff { references_by_model_definition_id: Some(BTreeMap::from([(payload.model_definition_id.clone(), payload.references.clone())])), ..Default::default() }
}
//#endregion 🔖️Diff
