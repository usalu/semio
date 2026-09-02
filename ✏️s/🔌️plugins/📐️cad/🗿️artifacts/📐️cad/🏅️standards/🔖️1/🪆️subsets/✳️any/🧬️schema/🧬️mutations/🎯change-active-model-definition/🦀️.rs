//! 🎯️ CAD mutation — `ChangeActiveModelDefinition` payload + `MutationKind` impl.

use crate::artifacts::cad::diff::CadDiff;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
/// 🎯️ Changes the document-level `active_model_definition_id` selector.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-active-model-definition")]
pub struct ChangeActiveModelDefinition {
    pub new_model_definition_id: String,
}

impl MutationKind<CadSnapshot, CadMutation> for ChangeActiveModelDefinition {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "active-model-definition", kind: "change-active-model-definition", record: "ChangedActiveModelDefinition" };

    fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<crate::artifacts::cad::diff::CadDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Switch active model definition to \"{}\"", self.new_model_definition_id)
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Mutation
