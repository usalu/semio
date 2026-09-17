//! 📄️ Wires play app panel — the document tree: identities and relationships of the current fixture.
//!
//! 🪟️ Both sections are **virtualised**: each reports its full `total` and materialises only the row
//! window the host asked for (`TreeWindows::for_body`, threaded in from `WiresPlayApp::render`), so a
//! real reasoning board scrolls to its end instead of hard-failing past the fixed child cap. Picks cost
//! zero argument arena now — the tree carries ONE `interactionSelect` binding and each row names only
//! its granularity, replacing the per-row `{domainId, merge, method, targets}` map.

use crate::editor::wires::terminology::WiresLabels;
use crate::editor::wires::{ui_label, WIRES_GRANULARITY_EDGE, WIRES_GRANULARITY_NODE, WIRES_INTERACTION_GRAPH, WIRES_PLAY_APP_ID};
use crate::schema::{dsl_id, fixture_edges, wires_identities, wires_relationships};
use crate::WiresSnapshot;
use semio_framework_plugin::{
    BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiAssemblyResult, UiText, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
};
use semio_framework_ui_contract::{Buildable, HasBase};

//#region 🔖️Constants
pub const WIRES_PLAY_BODY_ARTIFACT: &str = "reasoning.wires.document";
const WIRES_PLAY_DOCUMENT_NAMESPACE: &str = "wires-play-document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"),
        group: PanelGroup::Workbench,
        body_key: Some(WIRES_PLAY_BODY_ARTIFACT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn identity_label_lookup(wires: &dsl::DslValue, identity_id: u64) -> Option<String> {
    wires_identities(wires).iter().find(|identity| dsl_id(identity.get("identityId")) == Some(identity_id)).and_then(|identity| identity.get("label").and_then(|value| value.as_str())).map(str::to_string)
}

fn wires_identity_kind_name(wires: &dsl::DslValue, identity_kind_id: &str) -> Option<String> {
    wires
        .get("kindCatalogs")
        .and_then(|value| value.get("identityKinds"))
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .chain(wires.get("board").and_then(|value| value.get("meta")).and_then(|value| value.get("kindCatalogs")).and_then(|value| value.get("identityKinds")).and_then(|value| value.as_array()).into_iter().flatten())
        .find(|row| row.get("id").and_then(|value| value.as_str()) == Some(identity_kind_id))
        .and_then(|row| row.get("name").and_then(|value| value.as_str()))
        .map(str::to_string)
}

fn wires_relationship_document_label(wires: &dsl::DslValue, edge_id: &str, labels: &WiresLabels) -> Option<String> {
    let relationship = wires_relationships(wires).iter().find(|row| row.get("edgeId").and_then(|value| value.as_str()) == Some(edge_id))?;
    let kind = relationship.get("kind")?.as_str()?;
    let source_id = dsl_id(relationship.get("sourceIdentityId"))?;
    let target_id = dsl_id(relationship.get("targetIdentityId"))?;
    let source = identity_label_lookup(wires, source_id)?;
    let target = identity_label_lookup(wires, target_id)?;
    Some(format!("{}: {source} → {target}", crate::editor::wires::terminology::relationship_kind_display_name(kind, labels)))
}

/// 🕹️ One pick row of the "graph" domain. Row `id` is the BARE identity/edge id (not a namespaced row
/// id) — the framework matches `state.selection`/`.hover` ids against a row's own `id` verbatim and
/// canvas hit-testing resolves those exact bare ids too, so a prefixed row id would desync tree/canvas
/// cross-highlighting (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM). The row carries only
/// its granularity: the activation a click needs is the ONE tree-level `interactionSelect` binding
/// `PanelTreeBuilder::interaction_domain` stamps, so the per-row argument map is gone.
fn pick_row(id: &str, label: impl AsRef<str>, description: Option<String>, granularity: &str) -> UiAssemblyResult<BuiltNode> {
    let admission = |reason: &'static str| PluginAssemblyError::new("ui.fixed-capacity", reason);
    let mut row = semio_framework_ui_contract::tree_item(ui_label(label)?)
        .try_id(id)
        .map_err(|_| admission("wires row id admission failed"))?
        .granularity(UiText::try_from_str(granularity).ok_or_else(|| admission("wires row granularity admission failed"))?);
    if let Some(description) = description {
        row = row.description(UiText::try_from_string(description).map_err(|_| admission("wires row description admission failed"))?);
    }
    row.try_build().map_err(|_| admission("wires row admission failed"))
}

fn identity_row(wires: &dsl::DslValue, identity: &dsl::DslValue) -> UiAssemblyResult<BuiltNode> {
    let node_id = identity.get("nodeId").and_then(|value| value.as_str()).ok_or_else(|| PluginAssemblyError::new("ui.document", "wires identity node id is required"))?;
    let label = identity.get("label").and_then(|value| value.as_str()).ok_or_else(|| PluginAssemblyError::new("ui.document", "wires identity label is required"))?;
    let description = identity.get("identityKind").and_then(|value| value.as_str()).and_then(|kind| wires_identity_kind_name(wires, kind)).filter(|kind_name| kind_name != label);
    pick_row(node_id, label, description, WIRES_GRANULARITY_NODE)
}

fn relationship_row(wires: &dsl::DslValue, edge: &dsl::DslValue, labels: &WiresLabels) -> UiAssemblyResult<BuiltNode> {
    let edge_id = edge.get("id").and_then(|value| value.as_str()).ok_or_else(|| PluginAssemblyError::new("ui.document", "wires relationship id is required"))?;
    let label = wires_relationship_document_label(wires, edge_id, labels).unwrap_or_else(|| edge_id.into());
    pick_row(edge_id, label, None, WIRES_GRANULARITY_EDGE)
}

/// 🕹️ `.selected()?`/`.highlighted()?`/`.selection_change()` deleted — the framework stamps this tree's
/// presence from the "graph" `InteractionState` post-render and would overwrite whatever this function
/// stamped anyway.
pub fn render(document: &WiresSnapshot, labels: &WiresLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let wires = &document.wires_fixture;
    let board = &crate::wires_working_board(document);
    let identities = wires_identities(wires);
    let relationships = fixture_edges(board);
    PanelTreeBuilder::new(WIRES_PLAY_DOCUMENT_NAMESPACE)?
        .window_section_or_placeholder(windows, "wires-play-document.identities", Some(ui_label(labels.identities.as_str())?), true, identities, |identity| identity_row(wires, identity), ui_label("(none)")?)?
        .window_section_or_placeholder(windows, "wires-play-document.relationships", Some(ui_label(labels.relationships.as_str())?), false, relationships, |edge| relationship_row(wires, edge, labels), ui_label("(none)")?)?
        .interaction_domain(WIRES_PLAY_APP_ID, WIRES_INTERACTION_GRAPH)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
