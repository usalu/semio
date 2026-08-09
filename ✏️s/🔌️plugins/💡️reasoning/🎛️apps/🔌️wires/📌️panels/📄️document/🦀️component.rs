//! 📄️ Wires play app panel — the document tree: identities and relationships of the current fixture.

use crate::apps::wires::terminology::WiresLabels;
use crate::apps::wires::wires_action;
use crate::artifacts::wires::engine::{dsl_id, fixture_edges, fixture_nodes, wires_identities, wires_relationships};
use crate::artifacts::wires::WiresSnapshot;
use semio_framework_plugin::{
    tree_item_with_action, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
};
use serde_json::json;

//#region 🔖️Constants
pub const WIRES_PLAY_BODY_DOCUMENT: &str = "reasoning.wires.document";
const WIRES_PLAY_DOCUMENT_NAMESPACE: &str = "wires-play-document";
const WIRES_DOCUMENT_IDENTITY_PREFIX: &str = "wires-play-document.identity.";
const WIRES_DOCUMENT_RELATIONSHIP_PREFIX: &str = "wires-play-document.relationship.";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_DOCUMENT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(WIRES_PLAY_BODY_DOCUMENT.into()),
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
    Some(format!("{}: {source} → {target}", crate::apps::wires::terminology::relationship_kind_display_name(kind, labels)))
}

fn document_tree_selected_ids(board: &dsl::DslValue, selected: &[String]) -> Vec<String> {
    let namespace = PanelTreeBuilder::new(WIRES_PLAY_DOCUMENT_NAMESPACE);
    selected
        .iter()
        .filter_map(|id| {
            if fixture_nodes(board).iter().any(|node| node.get("id").and_then(|value| value.as_str()) == Some(id.as_str())) {
                return Some(namespace.item_id("identity", id));
            }
            if fixture_edges(board).iter().any(|edge| edge.get("id").and_then(|value| value.as_str()) == Some(id.as_str())) {
                return Some(namespace.item_id("relationship", id));
            }
            None
        })
        .collect()
}

pub fn render(document: &WiresSnapshot, selected: &[String], labels: &WiresLabels) -> UiNode {
    let wires = &document.wires_fixture;
    let board = &document.board_fixture;
    let identity_items: Vec<UiTreeItemNode> = wires_identities(wires)
        .iter()
        .filter_map(|identity| {
            let node_id = identity.get("nodeId")?.as_str()?;
            let label = identity.get("label")?.as_str()?;
            let identity_kind = identity.get("identityKind").and_then(|value| value.as_str());
            let description = identity_kind.and_then(|kind| wires_identity_kind_name(wires, kind)).filter(|kind_name| kind_name != label);
            Some(tree_item_with_action(format!("{WIRES_DOCUMENT_IDENTITY_PREFIX}{node_id}"), Label::data(label), description, wires_action("setSelection", Some(json!({ "ids": [node_id] })))))
        })
        .collect();
    let relationship_items: Vec<UiTreeItemNode> = fixture_edges(board)
        .iter()
        .filter_map(|edge| {
            let edge_id = edge.get("id")?.as_str()?;
            Some(tree_item_with_action(
                format!("{WIRES_DOCUMENT_RELATIONSHIP_PREFIX}{edge_id}"),
                Label::data(wires_relationship_document_label(wires, edge_id, labels).unwrap_or_else(|| edge_id.into())),
                None,
                wires_action("setSelection", Some(json!({ "ids": [edge_id] }))),
            ))
        })
        .collect();
    PanelTreeBuilder::new(WIRES_PLAY_DOCUMENT_NAMESPACE)
        .section_or_placeholder("wires-play-document.identities", Some(labels.identities.into()), true, identity_items, Label::data("(none)"))
        .section_or_placeholder("wires-play-document.relationships", Some(labels.relationships.into()), false, relationship_items, Label::data("(none)"))
        .selected(document_tree_selected_ids(board, selected))
        .selection_change(wires_action("setSelection", None))
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::wires::testkit::{metabolism_app, render as render_body};
    use crate::apps::wires::WIRES_PLAY_BODY_DOCUMENT as APP_BODY_DOCUMENT;

    #[test]
    fn document_has_identities_section() {
        let mut app = metabolism_app();
        let json = render_body(&mut app, APP_BODY_DOCUMENT);
        assert!(json.contains("wires-play-document.identities"));
        assert!(json.contains("Metabolism"));
    }

    #[test]
    fn definition_binds_the_framework_document_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_DOCUMENT_ID);
        assert_eq!(definition.body_key.as_deref(), Some(WIRES_PLAY_BODY_DOCUMENT));
    }
}
//#endregion 🧪️Tests
