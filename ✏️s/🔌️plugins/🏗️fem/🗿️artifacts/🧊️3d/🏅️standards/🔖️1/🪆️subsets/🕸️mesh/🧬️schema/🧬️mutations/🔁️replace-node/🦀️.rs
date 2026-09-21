//! 🔁️ Fem3d mutation — `ReplaceNode` payload + `MutationKind` impl.

use crate::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use crate::{Fem3dSnapshot, FemNode};
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Mutation
/// 🔁️ Whole-value swap of an existing node's payload — the one gesture in this vocabulary that
/// MOVES a node (the inspector's coordinate edits and the transform gumball both spell a move this
/// way). Moving a node an element, a support or a nodal load still names is deliberately allowed:
/// every referrer addresses it by `id`, and a replacement may not change that id, so the geometry
/// travels while the topology stays exactly where it was.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "replace-node")]
pub struct ReplaceNode {
    pub id: String,
    pub new_node: FemNode,
}

impl MutationKind<Fem3dSnapshot, Fem3dMutation> for ReplaceNode {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "replace", entity: "node", kind: "replace-node", record: "ReplacedNode" };

    fn diff(&self, base: &Fem3dSnapshot) -> protocol::MutationOutcome<crate::standards::v1::subsets::any::schema::diff::Fem3dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Fem3dSnapshot) -> Vec<Fem3dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native(&format!("Replace node \"{}\"", self.id), &format!("Knoten \"{}\" ersetzen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️Mutation
