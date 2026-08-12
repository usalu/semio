//! 🌱️ Fem3d mutation — `CreateNode` payload + `MutationKind` impl.
use crate::artifacts::fem3d::mutations::Fem3dMutation;
use crate::artifacts::fem3d::{Fem3dSnapshot, FemNode};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🌱️ Brings a new [`FemNode`] structural node into existence.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(keyword = "create-node")]
pub struct CreateNode {
    pub node: FemNode,
}

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for CreateNode {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "node", kind: "create-node", record: "CreatedNode" };

    fn diff(&self, base: &Fem3dSnapshot) -> crate::artifacts::fem3d::diff::Fem3dDiff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
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
