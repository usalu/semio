//! 🗑️ Fem2d mutation — `DeleteNode` payload + `MutationKind` impl.
use crate::artifacts::fem2d::mutations::Fem2dMutation;
use crate::artifacts::fem2d::Fem2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🗑️ Removes an existing structural node by id, capturing nothing itself (the removed payload is
/// recovered from `base` inside `↩️inverse`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "delete-node")]
pub struct DeleteNode {
    pub id: String,
}

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for DeleteNode {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "node", kind: "delete-node", record: "DeletedNode" };

    async fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem2d::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete node \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
