//! 🗑️ CAD mutation — `DeleteNode` payload + `MutationKind` impl.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🗑️ Removes an existing [`crate::artifacts::cad::CadNode`] from the scene graph tree.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-node")]
pub struct DeleteNode {
    pub node_id: String,
}

impl MutationKind<CadSnapshot, CadMutation> for DeleteNode {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "node", kind: "delete-node", record: "DeletedNode" };

    async fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<crate::artifacts::cad::diff::CadDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete node \"{}\"", self.node_id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.node_id.clone()]
    }
}
//#endregion 🔖️Mutation
