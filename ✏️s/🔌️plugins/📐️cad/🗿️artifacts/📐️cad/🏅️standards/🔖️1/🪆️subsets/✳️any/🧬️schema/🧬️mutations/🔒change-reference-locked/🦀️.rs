//! 🔒️ CAD mutation — `ChangeReferenceLocked` payload + `MutationKind` impl.

use crate::artifacts::cad::diff::CadDiff;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️Mutation
/// 🔒️ Change lock state of one reference overlay's `locked` field.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-reference-locked")]
pub struct ChangeReferenceLocked {
    pub model_definition_id: String,
    pub reference_id: String,
    pub new_locked: bool,
}

impl MutationKind<CadSnapshot, CadMutation> for ChangeReferenceLocked {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "change", entity: "reference", kind: "change-reference-locked", record: "ChangedReferenceLocked" };

    fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<crate::artifacts::cad::diff::CadDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change lock state of reference \"{}\"", self.reference_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.model_definition_id.clone(), self.reference_id.clone()]
    }
}
//#endregion 🔖️Mutation
