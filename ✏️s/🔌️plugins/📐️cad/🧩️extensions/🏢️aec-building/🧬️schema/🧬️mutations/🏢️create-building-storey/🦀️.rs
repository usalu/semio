//! 🏢️ Building storey composite mutation source.

use semio_framework_os_kernel::{FromValue, ToValue};
use semio_s_artifact_cad_cad::mutations::create_node::CreateNode;
use semio_s_artifact_cad_cad::{CadMutation, CadNode, CadSnapshot};

/// 🏢️ Composite mutation contributed onto cad's `s.cad.cad` artifact — a real building-domain
/// workflow step cad itself has no notion of (a bare CAD tool has no concept of a "storey"), planned
/// entirely from cad's document-owned `create-node` leaf mutation; pane focus stays host-owned
/// through `protocol::Planner::call`. Frozen id grammar (contract freeze §3):
/// `"<target-document-schema>#<contributor-plugin-id>:<kebab-kind>"` — assembled by
/// `ArtifactContribution::resolve`, never hand-formatted here.
// 🌱️ `CompositeMutationKind`'s supertrait bound is `ToValue`/`FromValue` (see that trait's own
// doc) — no `serde` derive needed here at all.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, protocol::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CreateBuildingStorey {
    pub storey_id: String,
    pub level_index: i32,
    pub storey_name: String,
}

impl CreateBuildingStorey {
    pub(super) fn storey_label(&self) -> String {
        format!("Level {}: {}", self.level_index, self.storey_name)
    }
}

impl protocol::CompositeMutationKind<CadSnapshot, CadMutation> for CreateBuildingStorey {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "building-storey", kind: "create-building-storey", record: "CreatedBuildingStorey" };

    fn plan(&self, _base: &CadSnapshot, planner: &mut protocol::Planner<CadSnapshot, CadMutation>) -> Result<(), protocol::PlanError> {
        planner.call(CadMutation::CreateNode(CreateNode { node: CadNode { id: self.storey_id.clone(), label: self.storey_label(), kind: "building-storey".into() } }))
    }

    fn label(&self) -> String {
        format!("Create building storey \"{}\"", self.storey_label())
    }

    fn target(&self) -> Vec<String> {
        vec![self.storey_id.clone()]
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
