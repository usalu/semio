//! 🔁️ Fem2d mutation — `ReplaceNode` payload + `MutationKind` impl.

use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use crate::{Fem2dSnapshot, FemNode};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🔁️ Whole-value swap of an existing node's payload — the one gesture in this vocabulary that
/// MOVES a node. Moving a node an element, a support or a nodal load still names is deliberately
/// allowed: every referrer addresses it by `id`, and a replacement may not change that id, so the
/// plan geometry travels while the topology stays exactly where it was.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-node")]
pub struct ReplaceNode {
    pub id: String,
    pub new_node: FemNode,
}

impl MutationKind<Fem2dSnapshot, Fem2dMutation> for ReplaceNode {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "node", kind: "replace-node", record: "ReplacedNode" };

    fn diff(&self, base: &Fem2dSnapshot) -> protocol::MutationOutcome<crate::standards::v1::subsets::any::schema::diff::Fem2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem2dSnapshot) -> Vec<Fem2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Replace node \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
