//! 🔧 `change-generation-value` payload — sets one answer value within a generation's form-values
//! map (single-field setter on a nested-addressed target, per `📓️taxonomy.md`'s `change` row).

use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::mutations::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️ChangeGenerationValue
/// 🔧 Nested address: outermost `id` (the generation) then `question_id` (the form field).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeGenerationValue {
    pub id: String,
    pub question_id: String,
    pub new_value: Value,
}

impl protocol::MutationKind<Procedural3dSnapshot, Procedural3dMutation> for ChangeGenerationValue {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "generation-value", kind: "change-generation-value", record: "ChangedGenerationValue" };

    async fn diff(&self, base: &Procedural3dSnapshot) -> protocol::MutationOutcome<Procedural3dDiff> {
        crate::artifacts::procedural3d::mutations::change_generation_value::diff::diff(self, base)
    }

    async fn inverse(&self, base: &Procedural3dSnapshot) -> Vec<Procedural3dMutation> {
        crate::artifacts::procedural3d::mutations::change_generation_value::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change generation \"{}\" value \"{}\"", self.id, self.question_id)
    }

    async fn target(&self) -> Vec<String> {
        vec![self.id.clone(), self.question_id.clone()]
    }
}
//#endregion 🔖️ChangeGenerationValue
