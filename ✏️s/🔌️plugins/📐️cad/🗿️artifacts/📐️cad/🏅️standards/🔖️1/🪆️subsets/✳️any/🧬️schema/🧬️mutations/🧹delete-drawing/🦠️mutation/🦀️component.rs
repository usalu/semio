//! 🧹️ `delete-drawing` — removes the entry matching `child_id` from `drawings`. Idempotent no-op
//! if absent; the inverse escrows the removed handle from BASE.

use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-drawing")]
pub struct DeleteDrawing {
    pub child_id: String,
}

impl MutationKind<CadSnapshot, CadMutation> for DeleteDrawing {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "drawing", kind: "delete-drawing", record: "DeletedDrawing" };

    fn diff(&self, base: &CadSnapshot) -> crate::artifacts::cad::diff::CadDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete drawing child {}", self.child_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.child_id.clone()]
    }
}
//#endregion 🔖️Mutation
