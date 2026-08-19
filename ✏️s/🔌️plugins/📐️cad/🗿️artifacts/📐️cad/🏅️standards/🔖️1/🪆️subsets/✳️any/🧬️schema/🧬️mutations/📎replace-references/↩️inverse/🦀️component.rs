//! ↩️ Inverse for `ReplaceReferences` — restores `base`'s reference list wholesale.
use super::mutation::ReplaceReferences;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &ReplaceReferences, base: &CadSnapshot) -> Vec<CadMutation> {
    let before = base.references_by_model_definition_id.get(&payload.model_definition_id).cloned().unwrap_or_default();
    vec![CadMutation::ReplaceReferences(ReplaceReferences { model_definition_id: payload.model_definition_id.clone(), references: before })]
}
//#endregion 🔖️Inverse
