//! ↩️ Inverse for `MoveReference` — recovers the pre-mutation `origin` from `base`.
use super::mutation::MoveReference;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &MoveReference, base: &CadSnapshot) -> Vec<CadMutation> {
    base.references_by_model_definition_id
        .get(&payload.model_definition_id)
        .and_then(|references| references.iter().find(|reference| reference.id == payload.reference_id))
        .map(|reference| vec![CadMutation::MoveReference(MoveReference { model_definition_id: payload.model_definition_id.clone(), reference_id: payload.reference_id.clone(), new_origin: reference.origin })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
