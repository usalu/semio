//! ↩️ Inverse for `ChangeReferenceLocked` — recovers the pre-mutation `locked` from `base`.
use super::mutation::ChangeReferenceLocked;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ChangeReferenceLocked, base: &CadSnapshot) -> Vec<CadMutation> {
    base.references_by_model_definition_id
        .get(&payload.model_definition_id)
        .and_then(|references| references.iter().find(|reference| reference.id == payload.reference_id))
        .map(|reference| vec![CadMutation::ChangeReferenceLocked(ChangeReferenceLocked { model_definition_id: payload.model_definition_id.clone(), reference_id: payload.reference_id.clone(), new_locked: reference.locked })])
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
