//! 🔵️ Wires play app command — `add-node`: one graph `create-node` leaf in the composed board child.

use crate::editor::wires::{wires_select_effect, WIRES_GRANULARITY_NODE};
use crate::{GraphNodeId, SemioGraphMutation, SemioValue, SemioValueEntry, WiresMutation, WiresSnapshot};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::mutations::create_node::CreateNode;

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "add-node")]
pub struct AddNode {
    pub kind: String,
}

/// ⭕️ The `create-node` leaf of a fresh circle node `id` of `kind` at the origin, labelled with its id: the board defaults
/// (`handles` none, `radius` [`crate::schema::WIRES_DEFAULT_NODE_RADIUS`], `shape` circle) as keyed properties, ascending.
pub fn create_board_node(id: &str, kind: &str) -> SemioGraphMutation {
    let properties = vec![
        SemioValueEntry { key: "handles".into(), value: SemioValue::List { items: Vec::new() } },
        SemioValueEntry { key: "radius".into(), value: SemioValue::Float { lexeme: crate::schema::WIRES_DEFAULT_NODE_RADIUS.to_string() } },
        SemioValueEntry { key: "shape".into(), value: SemioValue::Str { value: "circle".into() } },
    ];
    SemioGraphMutation::CreateNode(CreateNode { id: GraphNodeId::new(id), kind: kind.into(), label: id.into(), position: SemioPoint2 { x: 0.0, y: 0.0 }, width: 0.0, height: 0.0, ports: Vec::new(), properties, at: None })
}

/// 🕹️ Creates the node under the first unused `node-<n>` id of the composed board and selects it through the framework
/// interaction effect.
pub fn handle(payload: &AddNode, doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WiresMutation, NoConfigMutation>, Fault> {
    let composed = crate::wires_composed_from_children(doc.snapshot, &doc.children)?;
    let kind = if payload.kind.is_empty() { "identity" } else { payload.kind.as_str() };
    let id = (1..).map(|ordinal| format!("node-{ordinal}")).find(|candidate| crate::schema::board_node(&composed.board, candidate).is_none()).unwrap_or_default();
    let mut emit = crate::wires_child_emit(doc.snapshot, vec![create_board_node(&id, kind)]);
    emit.effects.push(wires_select_effect(&[id], WIRES_GRANULARITY_NODE, "replace"));
    Ok(emit)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
