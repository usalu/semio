//! 🕸️ 🕸️ Mathematical play app commands command — `node-graph-edit`.

use crate::artifacts::mathematical::op::MathematicalMutation;
use crate::artifacts::mathematical::schema::mutations::replace_graph::mutation::ReplaceGraph;
use crate::artifacts::mathematical::{MathematicalEdge, MathematicalNode, MathematicalSnapshot};
use crate::editor::mathematical::config::{MathematicalConfig, MathematicalConfigMutation};
use pack::json::Value as JsonValue;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

/// 🎨️ `nodeGraphActions.edit` (`"nodeGraphEdit"`) is the shared renderer-wide action id the generic
/// node-graph canvas dispatches interactive edit gestures under (see the React node-graph surface,
/// `dispatch(nodeGraphActions.edit, { operations: [...] })`) — renaming or splitting it here would
/// silently strand every node-graph interaction the frontend still targets under that id. Keeps its
/// former batched-array shape (`operations_json`, a JSON array of tagged sub-edits) verbatim rather
/// than splitting into one typed variant per sub-edit kind.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslRecord)]
#[dsl(keyword = "node-graph-edit")]
pub struct NodeGraphEdit {
    pub operations_json: String,
}

pub async fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, MathematicalSnapshot>, _cfg: &ConfigView<'_, MathematicalConfig>) -> Result<Emit<MathematicalMutation, MathematicalConfigMutation>, Fault> {
    let edit_operations: Vec<JsonValue> = pack::json::parse(&payload.operations_json).ok().and_then(|value| value.as_array().map(<[JsonValue]>::to_vec)).unwrap_or_default();
    let mut graph = crate::artifacts::mathematical::mathematical_graph(doc.snapshot);
    let mut changed = false;
    for operation in edit_operations {
        match operation.get("operation").and_then(JsonValue::as_str).unwrap_or("") {
            "addNode" => {
                let x = operation.get("x").and_then(JsonValue::as_f64).unwrap_or(0.0);
                let y = operation.get("y").and_then(JsonValue::as_f64).unwrap_or(0.0);
                let id = format!("n{}", graph.nodes.len());
                graph.nodes.push(MathematicalNode { label: id.to_uppercase(), id, x, y });
                changed = true;
            }
            "move" => {
                if let (Some(node_id), Some(x), Some(y)) = (operation.get("nodeId").and_then(JsonValue::as_str), operation.get("x").and_then(JsonValue::as_f64), operation.get("y").and_then(JsonValue::as_f64)) {
                    if let Some(node) = graph.nodes.iter_mut().find(|node| node.id == node_id) {
                        node.x = x;
                        node.y = y;
                        changed = true;
                    }
                }
            }
            "connect" => {
                if let (Some(source), Some(target)) = (operation.get("sourceNodeId").and_then(JsonValue::as_str), operation.get("targetNodeId").and_then(JsonValue::as_str)) {
                    let id = format!("e{}", graph.edges.len());
                    graph.edges.push(MathematicalEdge { id, source: source.into(), target: target.into() });
                    changed = true;
                }
            }
            "deleteSelection" => {
                if let Some(ids) = operation.get("nodeIds").and_then(|value| value.as_array()).and_then(|items| items.iter().map(|item| item.as_str().map(str::to_string)).collect::<Option<Vec<String>>>()) {
                    graph.nodes.retain(|node| !ids.contains(&node.id));
                    graph.edges.retain(|edge| !ids.contains(&edge.source) && !ids.contains(&edge.target));
                    changed = true;
                }
            }
            _ => {}
        }
    }
    if changed {
        Ok(Emit::mutations(vec![MathematicalMutation::ReplaceGraph(ReplaceGraph { graph })]))
    } else {
        Ok(Emit::default())
    }
}
