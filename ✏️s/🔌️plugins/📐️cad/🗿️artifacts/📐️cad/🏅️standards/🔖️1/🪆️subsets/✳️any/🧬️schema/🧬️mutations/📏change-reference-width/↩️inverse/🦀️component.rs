//! ↩️ Inverse for `ChangeReferenceWidth` — recovers the pre-mutation `width_world` from `base`.
use super::mutation::ChangeReferenceWidth;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeReferenceWidth, base: &CadSnapshot) -> Vec<CadMutation> {
    base.references_by_model_definition_id
        .get(&payload.model_definition_id)
        .and_then(|references| references.iter().find(|reference| reference.id == payload.reference_id))
        .map(|reference| vec![CadMutation::ChangeReferenceWidth(ChangeReferenceWidth { model_definition_id: payload.model_definition_id.clone(), reference_id: payload.reference_id.clone(), new_width_world: reference.width_world })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
