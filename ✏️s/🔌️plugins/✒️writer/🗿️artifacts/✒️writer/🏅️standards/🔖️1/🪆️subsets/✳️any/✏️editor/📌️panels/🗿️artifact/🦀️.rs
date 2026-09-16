//! 📄️ Writer play app panel — the document AST outline tree (nested Content/Outline sub-tabs sharing
//! one render).

use crate::editor::writer::terminology::WriterPlayLabels;
use crate::schema::{parse_jack_ast, JackAstNode};
use crate::{writer_text, WriterSnapshot};
use crate::editor::writer::{WRITER_INTERACTION_AST, WRITER_INTERACTION_GRANULARITY, WRITER_PLAY_APP_ID};
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase};
use semio_framework_plugin::{tree_item, tree_window_item, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiText, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const WRITER_PLAY_BODY_ARTIFACT: &str = "writer.play.artifact";
const WRITER_PANEL_TAB_ARTIFACT_CONTENT_ID: &str = "framework.panel.document.content";
const WRITER_PANEL_TAB_ARTIFACT_OUTLINE_ID: &str = "framework.panel.document.outline";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🌳️ Nested children of the document tab — demonstrates the recursive panel-tab tree (stacked tab rows)?.
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"),
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
///
/// 🪟️ Every AST level nests through [`tree_window_item`], so a wide `match`/`pattern` node streams
/// its children in host-chosen windows instead of failing the render past the fixed child capacity.
fn jack_ast_to_tree_item(node: &JackAstNode, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let item = ui::tree_item(crate::editor::writer::ui_label(node.label.clone())?)
        .try_id(&node.id)
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "writer AST id admission failed"))?
        .description(UiText::try_from_str(&node.kind).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "writer AST kind admission failed"))?)
        .granularity(UiText::try_from_str(WRITER_INTERACTION_GRANULARITY).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "writer AST granularity admission failed"))?);
    let default_open = matches!(node.kind.as_str(), "query" | "match" | "pattern" | "return");
    tree_window_item(windows, item, &node.id, default_open, &node.children, |child| jack_ast_to_tree_item(child, windows))
}

pub fn render(document: &WriterSnapshot, labels: &WriterPlayLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    if document.language_id != "jack" {
        let items = crate::editor::writer::ui_node_list([tree_item("writer-document.id", crate::editor::writer::ui_label(document.id.clone())?), tree_item("writer-document.language", crate::editor::writer::ui_label(document.language_id.clone())?)])?;
        return PanelTreeBuilder::new("writer-document")?.section("writer-document.meta", Some(crate::editor::writer::ui_label(labels.artifact.as_str())?), true, items)?.build();
    }
    let root = parse_jack_ast(&writer_text(document));
    PanelTreeBuilder::new("writer-play-document")?
        .window_section_or_placeholder(
            windows,
            "writer-play-document.ast",
            Some(crate::editor::writer::ui_label(labels.artifact.as_str())?),
            true,
            std::slice::from_ref(&root),
            |node| jack_ast_to_tree_item(node, windows),
            crate::editor::writer::ui_label(labels.empty_query.as_str())?,
        )?
        .interaction_domain(WRITER_PLAY_APP_ID, WRITER_INTERACTION_AST)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
