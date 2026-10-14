//! ↩️ Inverse for `ReplaceReferences` — restores `base`'s reference list wholesale.
use super::ReplaceReferences;
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::replace_references::ReplaceReferences>>(payload: &ReplaceReferences, base: &CadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    let before = base.references_by_model_definition_id.get(&payload.model_definition_id).cloned().unwrap_or_default();
    Ok(vec![Op::from(ReplaceReferences { model_definition_id: payload.model_definition_id.clone(), references: before })])
}
//#endregion 🔖️Inverse
