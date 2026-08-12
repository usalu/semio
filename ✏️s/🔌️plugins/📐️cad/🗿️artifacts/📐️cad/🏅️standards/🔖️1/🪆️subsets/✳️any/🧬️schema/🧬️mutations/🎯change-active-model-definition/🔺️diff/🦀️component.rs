//! 🔺️ Sparse diff builder for `ChangeActiveModelDefinition`.
use super::mutation::ChangeActiveModelDefinition;
use crate::artifacts::cad::diff::CadDiff;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeActiveModelDefinition, _base: &CadSnapshot) -> CadDiff {
    CadDiff { active_model_definition_id: Some(payload.new_model_definition_id.clone()), ..Default::default() }
}
//#endregion 🔖️Diff
