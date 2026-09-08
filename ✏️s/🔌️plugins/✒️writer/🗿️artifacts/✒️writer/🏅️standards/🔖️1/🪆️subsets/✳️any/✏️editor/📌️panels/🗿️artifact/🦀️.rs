//! 📄️ Writer play app panel — the document AST outline tree (nested Content/Outline sub-tabs sharing
//! one render).

use crate::artifacts::writer::schema::{parse_jack_ast, JackAstNode};
use crate::artifacts::writer::{writer_text, WriterSnapshot};
use crate::editor::writer::config::WriterConfig;
use crate::editor::writer::terminology::WriterPlayLabels;
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase, HasChildren};
use semio_framework_plugin::{tree_item, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiText, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const WRITER_PLAY_BODY_ARTIFACT: &str = "writer.play.document";
const WRITER_PANEL_TAB_ARTIFACT_CONTENT_ID: &str = "framework.panel.document.content";
const WRITER_PANEL_TAB_ARTIFACT_OUTLINE_ID: &str = "framework.panel.document.outline";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🌳️ Nested children of the document tab — demonstrates the recursive panel-tab tree (stacked tab rows)?.
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: None,
        children: vec![
            PanelTabDefinition {
                kind: PanelTabKind::App(WRITER_PANEL_TAB_ARTIFACT_CONTENT_ID.into()),
                label: LocalizedLabel::native("Content", "Inhalt"),
                group: PanelGroup::Workbench,
                body_key: Some(WRITER_PLAY_BODY_ARTIFACT.into()),
                children: Vec::new(),
            },
            PanelTabDefinition {
                kind: PanelTabKind::App(WRITER_PANEL_TAB_ARTIFACT_OUTLINE_ID.into()),
                label: LocalizedLabel::native("Outline", "Gliederung"),
                group: PanelGroup::Workbench,
                body_key: Some(WRITER_PLAY_BODY_ARTIFACT.into()),
                children: Vec::new(),
            },
        ],
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ `ast` domain items — no per-item `action` (and `UiTreeItemNode` no longer even carries
/// `hover_action`/`unhover_action` fields): the tree is bound to the `ast` interaction domain via
/// `.interaction_domain("ast")?` below, so the framework auto-injects `interactionSelect`/
/// `interactionHover` for every row (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM —
/// never declare those actions yourself).
fn jack_ast_to_tree_item(node: &JackAstNode) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let children = crate::editor::writer::ui_node_list(node.children.iter().map(jack_ast_to_tree_item))?;
    ui::tree_item(crate::editor::writer::ui_label(node.label.clone())?)
        .try_id(&node.id)
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "writer AST id admission failed"))?
        .description(UiText::try_from_str(&node.kind).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "writer AST kind admission failed"))?)
        // 🛟️ `and_then(IconName::from_str)` (not the panicking `IconName::from`) so a jack AST kind
        // whose icon string isn't (yet) in the shared icon catalog just renders with no icon.
        .default_open(matches!(node.kind.as_str(), "query" | "match" | "pattern" | "return"))
        .try_children(children)
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "writer AST child admission failed"))?
        .try_build()
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "writer AST row admission failed"))
}

pub fn render(document: &WriterSnapshot, _config: &WriterConfig, labels: &WriterPlayLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    if document.language_id != "jack" {
        let items = crate::editor::writer::ui_node_list([tree_item("writer-document.id", crate::editor::writer::ui_label(document.id.clone())?), tree_item("writer-document.language", crate::editor::writer::ui_label(document.language_id.clone())?)])?;
        return PanelTreeBuilder::new("writer-document")?.section("writer-document.meta", Some(crate::editor::writer::ui_label(labels.document.as_str())?), true, items)?.build();
    }
    let root = parse_jack_ast(&writer_text(document));
    let items = crate::editor::writer::ui_node_list([jack_ast_to_tree_item(&root)])?;
    PanelTreeBuilder::new("writer-play-document")?.section_or_placeholder("writer-play-document.ast", Some(crate::editor::writer::ui_label(labels.document.as_str())?), true, items, crate::editor::writer::ui_label(labels.empty_query.as_str())?)?.interaction_domain("ast")?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::writer::testkit::{app_with_jack, new_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn renders_document_tree_for_jack() {
        use semio_framework_plugin::PluginApp;
        let mut app = new_app().await;
        let node = app.render(WRITER_PLAY_BODY_ARTIFACT, Some(&crate::artifacts::writer::dsl::jack_example_json()), &semio_framework_plugin::ViewModel::default()).await.expect("render");
        let json = semio_framework_plugin::testkit::project_and_retire_fixture_tree(node).expect("render JSON");
        assert!(json.contains("\"type\":\"tree\""));
        assert!(json.contains("Query"));
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_the_framework_document_tab_and_its_children_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
        assert_eq!(definition.children.len(), 2);
        assert!(definition.children.iter().all(|child| child.body_key.as_deref() == Some(WRITER_PLAY_BODY_ARTIFACT)));
    }

    /// 🌳️ The AST section only appears for `jack`-language documents (see `render`'s early return for
    /// any other language) — load the jack fixture first.
    #[semio_framework_async_macros::async_test]
    async fn document_lists_the_ast_section_for_jack_documents() {
        let mut app = app_with_jack().await;
        assert!(render_body(&mut app, WRITER_PLAY_BODY_ARTIFACT).await.contains("writer-play-document.ast"));
    }

    /// 📄️ A non-jack (default/plaintext) document renders the plain id/language fallback section
    /// instead of the AST tree.
    #[semio_framework_async_macros::async_test]
    async fn document_falls_back_to_a_plain_section_for_non_jack_documents() {
        let mut app = new_app().await;
        assert!(render_body(&mut app, WRITER_PLAY_BODY_ARTIFACT).await.contains("writer-document"));
    }
}
//#endregion 🧪️Tests
