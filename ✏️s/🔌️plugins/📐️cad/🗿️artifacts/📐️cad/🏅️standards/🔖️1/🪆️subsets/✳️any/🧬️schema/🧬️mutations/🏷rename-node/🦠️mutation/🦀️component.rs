//! 🏷️ CAD mutation — `RenameNode` payload + `MutationKind` impl.
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🏷️ Renames an existing [`crate::artifacts::cad::CadNode`]'s `label`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "rename-node")]
pub struct RenameNode {
    pub node_id: String,
    pub new_label: String,
}

impl MutationKind<CadSnapshot, CadMutation> for RenameNode {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "rename", entity: "node", kind: "rename-node", record: "RenamedNode" };

    async fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<crate::artifacts::cad::diff::CadDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &CadSnapshot) -> Vec<CadMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Rename node to \"{}\"", self.new_label)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.node_id.clone()]
    }
}
//#endregion 🔖️Mutation
