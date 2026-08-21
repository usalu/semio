//! 🛍️ Wires play app panel — the identity/relationship kind catalogue (click to add).

use crate::editor::wires::terminology::WiresLabels;
use crate::editor::wires::wires_action;
use dsl::DslValue;
use semio_framework_plugin::{tree_item_with_action, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};
use serde_json::json;

//#region 🔖️Constants
pub const WIRES_PLAY_BODY_CATALOGUE: &str = "reasoning.wires.catalogue";
const WIRES_PLAY_CATALOGUE_TAB_ID: &str = "framework.panel.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(WIRES_PLAY_CATALOGUE_TAB_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(WIRES_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
async fn wires_kind_catalog_entries(wires: &DslValue, key: &str) -> Vec<DslValue> {
    wires
        .get("kindCatalogs")
        .and_then(|value| value.get(key))
        .and_then(|value| value.as_array())
        .map(|values| values.to_vec())
        .or_else(|| wires.get("board").and_then(|value| value.get("meta")).and_then(|value| value.get("kindCatalogs")).and_then(|value| value.get(key)).and_then(|value| value.as_array()).map(|values| values.to_vec()))
        .unwrap_or_default()
}

async fn catalog_kind_label(entry: &DslValue) -> String {
    entry.get("name").and_then(|value| value.as_str()).filter(|value| !value.is_empty()).or_else(|| entry.get("id").and_then(|value| value.as_str())).unwrap_or("kind").into()
}

async fn kind_catalog_items(namespace: &PanelTreeBuilder, kind: &str, entries: &[DslValue]) -> Vec<UiTreeItemNode> {
    entries
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            let kind_id = entry.get("id").and_then(|value| value.as_str()).unwrap_or("kind");
            let action = match kind {
                "identity-kinds" => wires_action("addNode", Some(json!({ "kind": kind_id }))),
                "relationship-kinds" => wires_action("addRelationship", Some(json!({ "kind": kind_id }))),
                _ => wires_action("addNode", Some(json!({ "kind": kind_id }))),
            };
            tree_item_with_action(namespace.item_id(kind, &format!("{index}.{kind_id}")), Label::data(catalog_kind_label(entry)), Some(kind_id.into()), action)
        })
        .collect()
}

pub async fn render(wires: &DslValue, labels: &WiresLabels) -> UiNode {
    let namespace = PanelTreeBuilder::new("wires-play-kinds");
    let identity_entries = wires_kind_catalog_entries(wires, "identityKinds");
    let relationship_entries = wires_kind_catalog_entries(wires, "relationshipKinds");
    let identity_items = kind_catalog_items(&namespace, "identity-kinds", &identity_entries);
    let relationship_items = kind_catalog_items(&namespace, "relationship-kinds", &relationship_entries);
    namespace
        .section_or_placeholder("wires-play-kinds.identity-kinds", Some(labels.identity_kinds.into()), true, identity_items, Label::data("(none)"))
        .section_or_placeholder("wires-play-kinds.relationship-kinds", Some(labels.relationship_kinds.into()), true, relationship_items, Label::data("(none)"))
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::wires::testkit::{metabolism_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn catalogue_lists_identity_and_relationship_kinds() {
        let mut app = metabolism_app();
        let json = render_body(&mut app, WIRES_PLAY_BODY_CATALOGUE);
        assert!(json.contains("Identity kinds"));
        assert!(json.contains("Relationship kinds"));
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_the_catalogue_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key.as_deref(), Some(WIRES_PLAY_BODY_CATALOGUE));
    }
}
//#endregion 🧪️Tests
