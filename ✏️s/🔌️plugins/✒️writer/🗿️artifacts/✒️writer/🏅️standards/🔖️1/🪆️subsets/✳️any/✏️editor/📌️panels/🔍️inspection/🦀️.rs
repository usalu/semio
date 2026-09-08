//! 🔍️ Writer play app panel — document/camera inspection plus jack diagnostics.

use crate::{writer_text, WriterSnapshot};
use crate::editor::writer::config::WriterConfig;
use crate::editor::writer::terminology::WriterPlayLabels;
use semio_framework_plugin::{tree_item, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiAssemblyResult, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};
use semio_s_artifact_trinity_jack::core::{example_graph, lint};

//#region 🔖️Constants
pub const WRITER_PLAY_BODY_INSPECTION: &str = "writer.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(WRITER_PLAY_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document: &WriterSnapshot, config: &WriterConfig, labels: &WriterPlayLabels) -> UiAssemblyResult<BuiltNode> {
    let text = writer_text(document);
    let document_items = crate::editor::writer::ui_node_list([
        tree_item("writer-inspector.document.schema", crate::editor::writer::ui_label(format!("Schema: {}", document.schema))?),
        tree_item("writer-inspector.document.id", crate::editor::writer::ui_label(format!("Id: {}", document.id))?),
        tree_item("writer-inspector.document.language", crate::editor::writer::ui_label(format!("Language: {}", document.language_id))?),
        tree_item("writer-inspector.document.uri", crate::editor::writer::ui_label(format!("Uri: {}", document.uri))?),
        tree_item("writer-inspector.document.lines", crate::editor::writer::ui_label(format!("Lines: {}", text.lines().count()))?),
    ])?;
    let camera_items = crate::editor::writer::ui_node_list([
        tree_item("writer-inspector.camera.x", crate::editor::writer::ui_label(format!("x: {}", config.camera.x))?),
        tree_item("writer-inspector.camera.y", crate::editor::writer::ui_label(format!("y: {}", config.camera.y))?),
        tree_item("writer-inspector.camera.zoom", crate::editor::writer::ui_label(format!("zoom: {}", config.camera.zoom))?),
    ])?;
    let mut tree = PanelTreeBuilder::new("writer-inspector")?.section("writer-inspector.document", Some(crate::editor::writer::ui_label(labels.document.as_str())?), true, document_items)?.section("writer-inspector.camera", Some(crate::editor::writer::ui_label(labels.camera.as_str())?), false, camera_items)?;
    if document.language_id == "jack" {
        let graph = example_graph();
        let messages: Vec<String> = lint(&graph, &text).into_iter().map(|diag| diag.message).take(8).collect();
        if !messages.is_empty() {
            let diagnostics = crate::editor::writer::ui_node_list(messages.into_iter().enumerate().map(|(index, message)| tree_item(format!("writer-inspector.diagnostics.{index}"), crate::editor::writer::ui_label(message)?)))?;
            tree = tree.section("writer-inspector.diagnostics", Some(crate::editor::writer::ui_label(labels.diagnostics.as_str())?), true, diagnostics)?;
        }
    }
    tree.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
