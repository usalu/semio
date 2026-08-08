//! ↩️ Inverse for `SetReferences`.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{CadReference, CadSnapshot};

//#region 🔖️Inverse
pub fn inverse(base: &CadSnapshot, model_definition_id: &str, _references: &[CadReference]) -> Vec<CadMutation> {
    let before = base.references_by_model_definition_id.get(model_definition_id).cloned().unwrap_or_default();
    vec![CadMutation::SetReferences { model_definition_id: model_definition_id.into(), references: before }]
}
//#endregion 🔖️Inverse
