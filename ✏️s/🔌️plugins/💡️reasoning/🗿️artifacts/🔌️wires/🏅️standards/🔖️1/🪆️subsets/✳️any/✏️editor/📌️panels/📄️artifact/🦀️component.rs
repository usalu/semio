//! 📄️ Wires play app panel — the document tree: identities and relationships of the current fixture.

use crate::artifacts::wires::schema::{dsl_id, fixture_edges, wires_identities, wires_relationships};
use crate::artifacts::wires::WiresSnapshot;
use crate::editor::wires::terminology::WiresLabels;
use crate::editor::wires::{ui_value_map, ui_value_text, wires_action, WIRES_GRANULARITY_EDGE, WIRES_GRANULARITY_NODE, WIRES_INTERACTION_GRAPH};
use semio_framework_plugin::{
    tree_item_with_action, BuiltNode, InteractionTarget, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiFixedList, UiValue, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
    INTERACTION_SELECT_ACTION_ID,
};

//#region 🔖️Constants
pub const WIRES_PLAY_BODY_DOCUMENT: &str = "reasoning.wires.document";
const WIRES_PLAY_DOCUMENT_NAMESPACE: &str = "wires-play-document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(WIRES_PLAY_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
async fn identity_label_lookup(wires: &dsl::DslValue, identity_id: u64) -> Option<String> {
    wires_identities(wires).iter().find(|identity| dsl_id(identity.get("identityId")) == Some(identity_id)).and_then(|identity| identity.get("label").and_then(|value| value.as_str())).map(str::to_string)
}

async fn wires_identity_kind_name(wires: &dsl::DslValue, identity_kind_id: &str) -> Option<String> {
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

async fn wires_relationship_document_label(wires: &dsl::DslValue, edge_id: &str, labels: &WiresLabels) -> Option<String> {
    let relationship = wires_relationships(wires).iter().find(|row| row.get("edgeId").and_then(|value| value.as_str()) == Some(edge_id))?;
    let kind = relationship.get("kind")?.as_str()?;
    let source_id = dsl_id(relationship.get("sourceIdentityId"))?;
    let target_id = dsl_id(relationship.get("targetIdentityId"))?;
    let source = identity_label_lookup(wires, source_id)?;
    let target = identity_label_lookup(wires, target_id)?;
    Some(format!("{}: {source} → {target}", crate::editor::wires::terminology::relationship_kind_display_name(kind, labels)))
}

fn selection_args(id: &str, granularity: &str) -> semio_framework_plugin::UiAssemblyResult<UiValue> {
    let targets = serde_json::to_string(&[InteractionTarget { granularity: granularity.into(), id: id.into() }])
        .map_err(|error| PluginAssemblyError::new("ui.action-argument", error.to_string()))?;
    ui_value_map([
        ("domainId", ui_value_text(WIRES_INTERACTION_GRAPH)?),
        ("merge", ui_value_text("replace")?),
        ("method", ui_value_text("pick")?),
        ("targets", ui_value_text(targets)?),
    ])
}

/// 🕹️ Row `id` is the BARE identity/edge id (not a namespaced row id) — the framework's
/// `.interaction_domain(WIRES_INTERACTION_GRAPH)?` presence stamping matches `state.selection`/`.hover`
/// ids against a row's own `id` verbatim, and canvas hit-testing resolves those exact bare ids too;
/// a prefixed row id would desync tree/canvas cross-highlighting (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub async fn render(document: &WiresSnapshot, labels: &WiresLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let wires = &document.wires_fixture;
    let board = &crate::artifacts::wires::wires_working_board(document);
    let mut identity_items = UiFixedList::<BuiltNode>::default();
    for identity in wires_identities(wires) {
        let node_id = identity.get("nodeId").and_then(|value| value.as_str()).ok_or_else(|| PluginAssemblyError::new("ui.document", "wires identity node id is required"))?;
        let label = identity.get("label").and_then(|value| value.as_str()).ok_or_else(|| PluginAssemblyError::new("ui.document", "wires identity label is required"))?;
        let identity_kind = identity.get("identityKind").and_then(|value| value.as_str());
        let description = match identity_kind {
            Some(kind) => wires_identity_kind_name(wires, kind).await.filter(|kind_name| kind_name != label),
            None => None,
        };
        let item = tree_item_with_action(node_id, Label::data(label), description, wires_action(INTERACTION_SELECT_ACTION_ID, Some(selection_args(node_id, WIRES_GRANULARITY_NODE)?))?)?;
        identity_items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "wires identity admission failed"))?;
    }
    let mut relationship_items = UiFixedList::<BuiltNode>::default();
    for edge in fixture_edges(board) {
        let edge_id = edge.get("id").and_then(|value| value.as_str()).ok_or_else(|| PluginAssemblyError::new("ui.document", "wires relationship id is required"))?;
        let label = wires_relationship_document_label(wires, edge_id, labels).await.unwrap_or_else(|| edge_id.into());
        let item = tree_item_with_action(edge_id, Label::data(label), None, wires_action(INTERACTION_SELECT_ACTION_ID, Some(selection_args(edge_id, WIRES_GRANULARITY_EDGE)?))?)?;
        relationship_items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "wires relationship admission failed"))?;
    }
    // 🕹️ `.selected()?`/`.highlighted()?`/`.selection_change()` deleted — the framework stamps this
    // tree's presence from the "graph" `InteractionState` post-render and would overwrite whatever
    // this function stamped anyway.
    PanelTreeBuilder::new(WIRES_PLAY_DOCUMENT_NAMESPACE)?
        .section_or_placeholder("wires-play-document.identities", Some(labels.identities.into()), true, identity_items, Label::data("(none)"))?
        .section_or_placeholder("wires-play-document.relationships", Some(labels.relationships.into()), false, relationship_items, Label::data("(none)"))?
        .interaction_domain(WIRES_INTERACTION_GRAPH)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::wires::testkit::{metabolism_app, render as render_body};
    use crate::editor::wires::WIRES_PLAY_BODY_DOCUMENT as APP_BODY_DOCUMENT;

    #[semio_framework_async_macros::async_test]
    async fn document_has_identities_section() {
        let mut app = metabolism_app();
        let json = render_body(&mut app, APP_BODY_DOCUMENT);
        assert!(json.contains("wires-play-document.identities"));
        assert!(json.contains("Metabolism"));
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_the_framework_document_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
        assert_eq!(definition.body_key.as_deref(), Some(WIRES_PLAY_BODY_DOCUMENT));
    }
}
//#endregion 🧪️Tests
