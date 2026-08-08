//! ↩️ Inverse for `PatchReference`.
use crate::artifacts::cad::mutations::{reverse_reference_patch, CadMutation, CadReferencePatch};
use crate::artifacts::cad::CadProjection;

//#region 🔖️Inverse
pub fn inverse(base: &CadProjection, model_definition_id: &str, reference_id: &str, patch: &CadReferencePatch) -> Vec<CadMutation> {
    base.references_by_model_definition_id
        .get(model_definition_id)
        .and_then(|references| {
            references.iter().find(|reference| reference.id == *reference_id).map(|before| {
                vec![CadMutation::PatchReference {
                    model_definition_id: model_definition_id.into(),
                    reference_id: reference_id.into(),
                    patch: reverse_reference_patch(before, patch),
                }]
            })
        })
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
