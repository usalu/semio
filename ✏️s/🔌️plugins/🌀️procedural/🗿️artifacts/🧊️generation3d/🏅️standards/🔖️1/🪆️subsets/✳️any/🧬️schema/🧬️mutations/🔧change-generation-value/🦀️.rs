//! 🔧 `change-generation-value` payload — sets one answer value within a generation's form-values
//! map (single-field setter on a nested-addressed target, per `📓️taxonomy.md`'s `change` row).

use crate::artifacts::generation3d::diff::Generation3dDiff;
use crate::artifacts::generation3d::mutations::Generation3dMutation;
use crate::artifacts::generation3d::Generation3dSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ChangeGenerationValue
/// 🔧 Nested address: outermost `id` (the generation) then `question_id` (the form field).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct ChangeGenerationValue {
    pub id: String,
    pub question_id: String,
    pub new_value: dsl::DslValue,
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for ChangeGenerationValue {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "generation-value", kind: "change-generation-value", record: "ChangedGenerationValue" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::artifacts::generation3d::mutations::change_generation_value::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
        crate::artifacts::generation3d::mutations::change_generation_value::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change generation \"{}\" value \"{}\"", self.id, self.question_id)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone(), self.question_id.clone()]
    }
}
//#endregion 🔖️ChangeGenerationValue
