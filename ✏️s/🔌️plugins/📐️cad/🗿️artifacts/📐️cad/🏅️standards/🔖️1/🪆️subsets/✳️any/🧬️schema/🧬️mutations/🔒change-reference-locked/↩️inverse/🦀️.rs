//! ↩️ Inverse for `ChangeReferenceLocked` — recovers the pre-mutation `locked` from `base`.
use super::ChangeReferenceLocked;
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::change_reference_locked::ChangeReferenceLocked>>(payload: &ChangeReferenceLocked, base: &CadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(base.references_by_model_definition_id
        .get(&payload.model_definition_id)
        .and_then(|references| references.iter().find(|reference| reference.id == payload.reference_id))
        .map(|reference| vec![Op::from(ChangeReferenceLocked { model_definition_id: payload.model_definition_id.clone(), reference_id: payload.reference_id.clone(), new_locked: reference.locked })])
        .unwrap_or_default())
}
//#endregion 🔖️Inverse
