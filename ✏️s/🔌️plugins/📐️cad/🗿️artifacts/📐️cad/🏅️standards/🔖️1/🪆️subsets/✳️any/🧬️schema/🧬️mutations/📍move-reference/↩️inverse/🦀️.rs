//! ↩️ Inverse for `MoveReference` — recovers the pre-mutation `origin` from `base`.
use super::MoveReference;
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::move_reference::MoveReference>>(payload: &MoveReference, base: &CadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(base.references_by_model_definition_id
        .get(&payload.model_definition_id)
        .and_then(|references| references.iter().find(|reference| reference.id == payload.reference_id))
        .map(|reference| vec![Op::from(MoveReference { model_definition_id: payload.model_definition_id.clone(), reference_id: payload.reference_id.clone(), new_origin: reference.origin })])
        .unwrap_or_default())
}
//#endregion 🔖️Inverse
