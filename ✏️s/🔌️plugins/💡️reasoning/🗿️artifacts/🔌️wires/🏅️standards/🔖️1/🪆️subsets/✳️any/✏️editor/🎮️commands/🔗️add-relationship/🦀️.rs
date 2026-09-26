//! 🔗️ 🔗️ Wires play app commands command — `add-relationship`.

use crate::editor::wires::{wires_select_effect, WIRES_GRANULARITY_EDGE, WIRES_INTERACTION_GRAPH};
use crate::op::WiresMutation;
use crate::schema::{entity_id, fixture_edges, wires_identities};
use crate::standards::v1::subsets::any::schema::inferences::find_board_node;
use crate::WiresSnapshot;
use dsl::DslValue;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
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
fn endpoints(payload: &AddRelationship, document: &WiresSnapshot, selected: &[String]) -> Result<(String, String), Fault> {
    let (source, target) = if payload.source_id.is_empty() && payload.target_id.is_empty() {
        let mut nodes = selected.iter().filter(|id| find_board_node(document, id).is_some());
        match (nodes.next(), nodes.next()) {
            (Some(source), Some(target)) => (source.clone(), target.clone()),
            _ => return Err(refusal("wires.relationship-endpoints-missing", "addRelationship needs sourceId and targetId, or two selected nodes".into())),
        }
    } else {
        (payload.source_id.clone(), payload.target_id.clone())
    };
    for (argument, id) in [("sourceId", &source), ("targetId", &target)] {
        if id.is_empty() {
            return Err(refusal("wires.relationship-endpoint-missing", format!("addRelationship needs {argument}")));
        }
        if find_board_node(document, id).is_none() {
            return Err(refusal("mutation.target-missing", format!("Node \"{id}\" does not exist.")));
        }
    }
    if source == target {
        return Err(refusal("wires.relationship-self", format!("addRelationship cannot connect node \"{source}\" to itself")));
    }
    Ok((source, target))
}

/// 🔗️ Connects `sourceId` to `targetId` with a new edge of `kind`, adds the semantic relationship when
/// both nodes carry an identity, and selects the new edge.
fn relate(payload: &AddRelationship, document: &WiresSnapshot, selected: &[String]) -> Result<Emit<WiresMutation, NoConfigMutation>, Fault> {
    let (source, target) = endpoints(payload, document, selected)?;
    let kind = if payload.kind.is_empty() { DEFAULT_RELATIONSHIP_KIND } else { payload.kind.as_str() };
    let board = crate::wires_working_board(document);
    let edge_id = (1..).map(|ordinal| format!("edge-{ordinal}")).find(|candidate| !fixture_edges(&board).iter().any(|edge| entity_id(edge, "id") == Some(candidate.as_str()))).unwrap_or_default();
    let edge = DslValue::object([
        ("id".into(), DslValue::String(edge_id.clone())),
        ("edgeKind".into(), DslValue::String(format!("wires.{kind}"))),
        ("source".into(), DslValue::String(source.clone())),
        ("target".into(), DslValue::String(target.clone())),
    ]);
    let identity = |node: &str| wires_identities(&document.wires_fixture).iter().find(|row| entity_id(row, "nodeId") == Some(node)).and_then(|row| row.get("identityId").cloned());
    let relationship = match (identity(&source), identity(&target)) {
        (Some(source_identity), Some(target_identity)) => DslValue::object([
            ("edgeId".into(), DslValue::String(edge_id.clone())),
            ("kind".into(), DslValue::String(kind.into())),
            ("sourceIdentityId".into(), source_identity),
            ("targetIdentityId".into(), target_identity),
        ]),
        _ => DslValue::Null,
    };
    Ok(Emit { artifact_mutations: vec![crate::mutations::connect_nodes(edge, relationship)], effects: vec![wires_select_effect(&[edge_id], WIRES_GRANULARITY_EDGE, "replace")], ..Default::default() })
}

/// 🕹️ `app_commands!`'s generated `dispatch(doc, cfg)` has no interaction slot, so this path only
/// honours named endpoints; the live routes pass the graph selection through [`apply`] / [`apply_with_state`].
pub fn handle(payload: &AddRelationship, doc: &ArtifactView<'_, WiresSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WiresMutation, NoConfigMutation>, Fault> {
    relate(payload, doc.snapshot, &[])
}

/// 🔗️ The `ArtifactApp::handle` route: named endpoints, else the graph selection.
pub fn apply(payload: &AddRelationship, doc: &ArtifactView<'_, WiresSnapshot>, interaction: &InteractionView<'_>) -> Result<Emit<WiresMutation, NoConfigMutation>, Fault> {
    relate(payload, doc.snapshot, &interaction.selection(WIRES_INTERACTION_GRAPH).ids)
}

/// 🧵️ The retained-tool twin of [`apply`] over the raw interaction state a bounded reducer receives.
pub fn apply_with_state(payload: &AddRelationship, doc: &ArtifactView<'_, WiresSnapshot>, interaction: &protocol::InteractionState) -> Result<Emit<WiresMutation, NoConfigMutation>, Fault> {
    let selected = interaction.selection.get(WIRES_INTERACTION_GRAPH).map(|domain| domain.ids.clone()).unwrap_or_default();
    relate(payload, doc.snapshot, &selected)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
