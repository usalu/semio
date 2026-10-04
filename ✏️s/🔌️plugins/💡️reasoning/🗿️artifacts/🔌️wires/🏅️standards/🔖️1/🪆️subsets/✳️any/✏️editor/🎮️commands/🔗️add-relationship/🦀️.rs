//! 🔗️ Wires play app command — `add-relationship`: one graph `create-edge` leaf in the composed board child, carrying the
//! wires relationship of its endpoints' identities as its `relationship` property (self-describing, design §20.9).

use crate::editor::wires::{wires_select_effect, WIRES_GRANULARITY_EDGE, WIRES_INTERACTION_GRAPH};
use crate::schema::{board_edge, board_node, entity_id, wires_identities};
use crate::{GraphEdgeId, GraphNodeId, SemioGraphMutation, SemioValue, SemioValueEntry, WiresComposed, WiresMutation, WiresSnapshot, WIRES_RELATIONSHIP_PROPERTY};
use semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::mutations::create_edge::CreateEdge;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "add-relationship")]
pub struct AddRelationship {
    pub kind: String,
    pub source_id: String,
    pub target_id: String,
}

/// 🏷️ The relationship kind an invocation without one writes.
const DEFAULT_RELATIONSHIP_KIND: &str = "owns";

fn refusal(code: &str, message: String) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new(code), message)
}

/// 🎯️ The two endpoint nodes: the named `sourceId`/`targetId`, else the first two selected nodes in
/// selection order. Every endpoint must be a live board node and the two must differ.
fn endpoints(payload: &AddRelationship, composed: &WiresComposed, selected: &[String]) -> Result<(String, String), Fault> {
    let (source, target) = if payload.source_id.is_empty() && payload.target_id.is_empty() {
        let mut nodes = selected.iter().filter(|id| board_node(&composed.board, id).is_some());
        match (nodes.next(), nodes.next()) {
            (Some(source), Some(target)) => (source.clone(), target.clone()),
            _ => return Err(refusal("wires.relationship.endpoints-missing", "addRelationship needs sourceId and targetId, or two selected nodes".into())),
        }
    } else {
        (payload.source_id.clone(), payload.target_id.clone())
    };
    for (argument, id) in [("sourceId", &source), ("targetId", &target)] {
        if id.is_empty() {
            return Err(refusal("wires.relationship.endpoint-missing", format!("addRelationship needs {argument}")));
        }
        if board_node(&composed.board, id).is_none() {
            return Err(refusal("mutation.target-missing", format!("Node \"{id}\" does not exist.")));
        }
    }
    if source == target {
        return Err(refusal("wires.relationship.self", format!("addRelationship cannot connect node \"{source}\" to itself")));
    }
    Ok((source, target))
}

/// 🔗️ The `create-edge` leaf connecting `source` to `target` with a new edge of `kind`; when both nodes carry an identity the
/// edge records the relationship (`kind`, `sourceIdentityId`, `targetIdentityId`) as its `relationship` property.
fn relationship_edge(snapshot: &WiresSnapshot, edge_id: &str, kind: &str, source: &str, target: &str) -> SemioGraphMutation {
    let identity = |node: &str| wires_identities(&snapshot.wires_fixture).iter().find(|row| entity_id(row, "nodeId") == Some(node)).and_then(|row| row.get("identityId").cloned());
    let relationship = match (identity(source), identity(target)) {
        (Some(source_identity), Some(target_identity)) => vec![SemioValueEntry {
            key: WIRES_RELATIONSHIP_PROPERTY.into(),
            value: SemioValue::Map {
                entries: vec![
                    SemioValueEntry { key: "kind".into(), value: SemioValue::Str { value: kind.into() } },
                    SemioValueEntry { key: "sourceIdentityId".into(), value: crate::semio_value_from_dsl(&source_identity) },
                    SemioValueEntry { key: "targetIdentityId".into(), value: crate::semio_value_from_dsl(&target_identity) },
                ],
            },
        }],
        _ => Vec::new(),
    };
    SemioGraphMutation::CreateEdge(CreateEdge { id: GraphEdgeId::new(edge_id), source: GraphNodeId::new(source), target: GraphNodeId::new(target), kind: format!("wires.{kind}"), label: String::new(), source_port: None, target_port: None, properties: relationship, at: None })
}

/// 🔗️ Connects `sourceId` to `targetId` with a new edge of `kind` under the first unused `edge-<n>` id and selects it.
fn relate(payload: &AddRelationship, doc: &ArtifactView<'_, WiresSnapshot>, selected: &[String]) -> Result<Emit<WiresMutation, NoConfigMutation>, Fault> {
    let composed = crate::wires_composed_from_children(doc.snapshot, &doc.children)?;
    let (source, target) = endpoints(payload, &composed, selected)?;
    let kind = if payload.kind.is_empty() { DEFAULT_RELATIONSHIP_KIND } else { payload.kind.as_str() };
    let edge_id = (1..).map(|ordinal| format!("edge-{ordinal}")).find(|candidate| board_edge(&composed.board, candidate).is_none()).unwrap_or_default();
    let mut emit = crate::wires_child_emit(doc.snapshot, &[relationship_edge(doc.snapshot, &edge_id, kind, &source, &target)]);
    emit.effects.push(wires_select_effect(&[edge_id], WIRES_GRANULARITY_EDGE, "replace"));
    Ok(emit)
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg)` has no interaction slot, so this path only
/// honours named endpoints; the live routes pass the graph selection through [`apply`] / [`apply_with_state`].
pub fn handle(payload: &AddRelationship, doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WiresMutation, NoConfigMutation>, Fault> {
    relate(payload, doc, &[])
}

/// 🔗️ The `ArtifactApp::handle` route: named endpoints, else the graph selection.
pub fn apply(payload: &AddRelationship, doc: &ArtifactView<'_, WiresSnapshot>, interaction: &InteractionView<'_>) -> Result<Emit<WiresMutation, NoConfigMutation>, Fault> {
    relate(payload, doc, &interaction.selection(WIRES_INTERACTION_GRAPH).ids)
}

/// 🧵️ The retained-tool twin of [`apply`] over the raw interaction state a bounded reducer receives.
pub fn apply_with_state(payload: &AddRelationship, doc: &ArtifactView<'_, WiresSnapshot>, interaction: &protocol::InteractionState) -> Result<Emit<WiresMutation, NoConfigMutation>, Fault> {
    let selected = interaction.selection.get(WIRES_INTERACTION_GRAPH).map(|domain| domain.ids.clone()).unwrap_or_default();
    relate(payload, doc, &selected)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
