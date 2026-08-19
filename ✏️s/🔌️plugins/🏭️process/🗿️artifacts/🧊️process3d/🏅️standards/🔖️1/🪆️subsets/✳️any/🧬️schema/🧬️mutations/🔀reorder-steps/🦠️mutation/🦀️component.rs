//! 🔀 `reorder-steps` payload — repositions one id-keyed [`ProcessStep`] within the document's
//! ordered timeline (order is user-meaningful here, unlike the unordered `machines` collection).

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ReorderSteps
/// 🔀 `to_index` is FINAL-state, clamped to the list length — mirrors the pre-migration
/// generic-move semantics the old collection-op engine implemented.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReorderSteps {
    pub id: String,
    pub to_index: usize,
}

impl protocol::MutationKind<Process3dSnapshot, Process3dMutation> for ReorderSteps {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "step", kind: "reorder-steps", record: "ReorderedSteps" };

    async fn diff(&self, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
        crate::artifacts::process3d::mutations::reorder_steps::diff::diff(self, base)
    }

    async fn inverse(&self, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
        crate::artifacts::process3d::mutations::reorder_steps::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Reorder step \"{}\"", self.id)
    }

    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️ReorderSteps
