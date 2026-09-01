//! 🔺️ Sparse diff builder for `ChangeActiveModelDefinition`.
use super::ChangeActiveModelDefinition;
use crate::artifacts::cad::diff::CadDiff;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeActiveModelDefinition, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    if base.active_model_definition_id == payload.new_model_definition_id {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Active model definition is already \"{}\".", payload.new_model_definition_id));
    }
    protocol::MutationOutcome::new(CadDiff { active_model_definition_id: Some(payload.new_model_definition_id.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
