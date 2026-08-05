//! 🔍️ Wires play app panel — the inspector: selected node fields, or a document-wide summary.

use crate::artifacts::wires::engine::{fixture_json_string, fixture_nodes, wires_identities, DefaultWiresExtension};
use crate::artifacts::wires::{MindmapWiresDocument, MINDMAP_WIRES_SCHEMA};
use semio_framework_plugin::{ui_inspector_readonly_field, ui_stack_vertical, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

//#region 🔖️Constants
pub const WIRES_PLAY_BODY_PROPERTIES: &str = "reasoning.wires.properties";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(WIRES_PLAY_BODY_PROPERTIES.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document: &MindmapWiresDocument, selected: &[String]) -> UiNode {
    let selected_nodes: Vec<&dsl::DslValue> = selected.iter().filter_map(|id| fixture_nodes(&document.board_fixture).iter().find(|node| node.get("id").and_then(|value| value.as_str()) == Some(id.as_str()))).collect();
    if selected_nodes.is_empty() {
        let extension = DefaultWiresExtension::from_fixture_json(&fixture_json_string(&document.wires_fixture)).ok();
        return ui_stack_vertical(vec![
            ui_text(Label::data(format!("Schema: {MINDMAP_WIRES_SCHEMA}"))),
            ui_text(Label::data(format!("Identities: {}", extension.as_ref().map_or(0, |ext| ext.mindmap.topics.len())))),
            ui_text(Label::data(format!("Relationships: {}", extension.as_ref().map_or(0, |ext| ext.relationships.len())))),
            ui_text(Label::data(format!("Board nodes: {}", fixture_nodes(&document.board_fixture).len()))),
        ]);
    }
    let node = selected_nodes[0];
    let identity = wires_identities(&document.wires_fixture).iter().find(|identity| identity.get("nodeId").and_then(|value| value.as_str()) == node.get("id").and_then(|value| value.as_str()));
    ui_stack_vertical(vec![
        ui_inspector_readonly_field("wires-play-inspector.id", Label::data("Id"), node.get("id").and_then(|value| value.as_str()).unwrap_or("").to_string()),
        ui_inspector_readonly_field("wires-play-inspector.identity-label", Label::data("Identity"), identity.and_then(|row| row.get("label")).and_then(|value| value.as_str()).unwrap_or("—").to_string()),
        ui_inspector_readonly_field("wires-play-inspector.node-kind", Label::data("Identity Kind"), node.get("nodeKind").and_then(|value| value.as_str()).unwrap_or("—").to_string()),
        ui_inspector_readonly_field("wires-play-inspector.x", Label::data("X"), node.get("x").and_then(|value| value.as_f64()).map_or_else(|| "—".into(), |value| value.to_string())),
        ui_inspector_readonly_field("wires-play-inspector.y", Label::data("Y"), node.get("y").and_then(|value| value.as_f64()).map_or_else(|| "—".into(), |value| value.to_string())),
    ])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::wires::testkit::{metabolism_app, render as render_body};

    #[test]
    fn empty_selection_shows_document_summary() {
        let mut app = metabolism_app();
        let json = render_body(&mut app, WIRES_PLAY_BODY_PROPERTIES);
        assert!(json.contains("Schema:"));
        assert!(json.contains("Board nodes:"));
    }

    #[test]
    fn definition_binds_the_inspection_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key.as_deref(), Some(WIRES_PLAY_BODY_PROPERTIES));
    }
}
//#endregion 🧪️Tests
