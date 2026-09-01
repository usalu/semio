//! 🌱️ Fem2d mutation — `CreateNode` payload + `MutationKind` impl.

use crate::artifacts::fem2d::{Fem2dSnapshot, FemNode};
use crate::artifacts::fem2d::diff::{Fem2dDiff, Fem2dNodesDelta};
use crate::artifacts::fem2d::mutations::{Fem2dMutation, delete_node};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌱️ Brings a new [`FemNode`] structural node into existence.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-node")]
pub struct CreateNode {
    pub node: FemNode,
}

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for CreateNode {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "node", kind: "create-node", record: "CreatedNode" };

    fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<crate::artifacts::fem2d::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Create node \"{}\"", self.node.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.node.id.clone()]
    }
}
//#endregion 🔖️Mutation
