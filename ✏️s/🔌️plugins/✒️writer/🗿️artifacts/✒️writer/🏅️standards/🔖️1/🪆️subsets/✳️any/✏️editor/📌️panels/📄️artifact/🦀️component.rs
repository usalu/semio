//! 📄️ Writer play app panel — the document AST outline tree (nested Content/Outline sub-tabs sharing
//! one render).

use crate::editor::writer::config::WriterConfig;
use crate::editor::writer::terminology::WriterPlayLabels;
use crate::artifacts::writer::schema::{jack_ast_tree_icon, parse_jack_ast, JackAstNode};
use crate::artifacts::writer::{writer_text, WriterSnapshot};
use semio_framework_plugin::{
    tree_item, ui_declarative_sections_to_tree, ui_text, IconName, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiPresence, UiSectionNode, UiTreeItemNode,
    FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
};

//#region 🔖️Constants
pub const WRITER_PLAY_BODY_ARTIFACT: &str = "writer.play.document";
const WRITER_PANEL_TAB_ARTIFACT_CONTENT_ID: &str = "framework.panel.document.content";
const WRITER_PANEL_TAB_ARTIFACT_OUTLINE_ID: &str = "framework.panel.document.outline";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🌳️ Nested children of the document tab — demonstrates the recursive panel-tab tree (stacked tab rows).
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: None,
        children: vec![
            PanelTabDefinition { kind: PanelTabKind::App(WRITER_PANEL_TAB_ARTIFACT_CONTENT_ID.into()), label: LocalizedLabel::native("Content", "Inhalt"), group: PanelGroup::Workbench, body_key: Some(WRITER_PLAY_BODY_ARTIFACT.into()), children: Vec::new() },
            PanelTabDefinition { kind: PanelTabKind::App(WRITER_PANEL_TAB_ARTIFACT_OUTLINE_ID.into()), label: LocalizedLabel::native("Outline", "Gliederung"), group: PanelGroup::Workbench, body_key: Some(WRITER_PLAY_BODY_ARTIFACT.into()), children: Vec::new() },
        ],
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ `ast` domain items — no per-item `action` (and `UiTreeItemNode` no longer even carries
/// `hover_action`/`unhover_action` fields): the tree is bound to the `ast` interaction domain via
/// `.interaction_domain("ast")` below, so the framework auto-injects `interactionSelect`/
/// `interactionHover` for every row (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM —
/// never declare those actions yourself).
async fn jack_ast_to_tree_item(node: &JackAstNode) -> UiTreeItemNode {
    let children: Vec<UiTreeItemNode> = node.children.iter().map(jack_ast_to_tree_item).collect();
    UiTreeItemNode {
        id: node.id.clone(),
        label: Label::data(node.label.clone()),
        description: Some(node.kind.clone()),
        // 🛟️ `and_then(IconName::from_str)` (not the panicking `IconName::from`) so a jack AST kind
        // whose icon string isn't (yet) in the shared icon catalog just renders with no icon.
        icon_id: jack_ast_tree_icon(&node.kind).and_then(IconName::from_str),
        presence: UiPresence::default(),
        default_open: Some(matches!(node.kind.as_str(), "query" | "match" | "pattern" | "return")),
        action: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: if children.is_empty() { None } else { Some(children) },
        control: None,
        dimmed: None,
        menu: None,
    }
}

pub async fn render(document: &WriterSnapshot, _config: &WriterConfig, labels: &WriterPlayLabels) -> UiNode {
    if document.language_id != "jack" {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "writer-document".into(),
            label: Some(labels.document.into()),
            default_open: Some(true),
            children: vec![ui_text(Label::data(document.id.clone())), ui_text(Label::data(document.language_id.clone()))],
            presence: UiPresence::default(),
            menu: None,
        }]);
    }
    let root = parse_jack_ast(&writer_text(document));
    let items = if root.kind == "error" {
        vec![UiTreeItemNode { description: Some(root.kind.clone()), icon_id: jack_ast_tree_icon(&root.kind).and_then(IconName::from_str), ..tree_item(root.id.as_str(), Label::data(root.label.as_str())) }]
    } else {
        vec![jack_ast_to_tree_item(&root)]
    };
    PanelTreeBuilder::new("writer-play-document")
        .section_or_placeholder("writer-play-document.ast", Some(labels.document.into()), true, items, labels.empty_query)
        .interaction_domain("ast")
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::writer::testkit::{app_with_jack, new_app, render as render_body};

    #[test]
    async fn renders_document_tree_for_jack() {
        use semio_framework_plugin::PluginApp;
        let mut app = new_app();
        let node = app.render(WRITER_PLAY_BODY_ARTIFACT, Some(&crate::artifacts::writer::dsl::jack_example_json()), &semio_framework_plugin::ViewModel::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"type\":\"tree\""));
        assert!(json.contains("Query"));
    }

    #[test]
    async fn definition_binds_the_framework_document_tab_and_its_children_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
        assert_eq!(definition.children.len(), 2);
        assert!(definition.children.iter().all(|child| child.body_key.as_deref() == Some(WRITER_PLAY_BODY_ARTIFACT)));
    }

    /// 🌳️ The AST section only appears for `jack`-language documents (see `render`'s early return for
    /// any other language) — load the jack fixture first.
    #[test]
    async fn document_lists_the_ast_section_for_jack_documents() {
        let mut app = app_with_jack();
        assert!(render_body(&mut app, WRITER_PLAY_BODY_ARTIFACT).contains("writer-play-document.ast"));
    }

    /// 📄️ A non-jack (default/plaintext) document renders the plain id/language fallback section
    /// instead of the AST tree.
    #[test]
    async fn document_falls_back_to_a_plain_section_for_non_jack_documents() {
        let mut app = new_app();
        assert!(render_body(&mut app, WRITER_PLAY_BODY_ARTIFACT).contains("writer-document"));
    }
}
//#endregion 🧪️Tests
