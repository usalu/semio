//! 🔍️ Writer play app panel — document/camera inspection plus jack diagnostics.

use crate::editor::writer::terminology::WriterPlayLabels;
use crate::{writer_text, WriterSnapshot};
use semio_framework_plugin::{tree_item, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, TreeWindows, UiAssemblyResult, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};
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
/// 🪟️ The lint list used to be cut off at eight diagnostics with nothing saying so; it is a windowed
/// section now, so the section always reports how many diagnostics the document really has.
pub fn render(document: &WriterSnapshot, labels: &WriterPlayLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let text = writer_text(document);
    let document_items = crate::editor::writer::ui_node_list([
        tree_item("writer-inspector.document.schema", crate::editor::writer::ui_label(format!("Schema: {}", document.schema))?),
        tree_item("writer-inspector.document.id", crate::editor::writer::ui_label(format!("Id: {}", document.id))?),
        tree_item("writer-inspector.document.language", crate::editor::writer::ui_label(format!("Language: {}", document.language_id))?),
        tree_item("writer-inspector.document.uri", crate::editor::writer::ui_label(format!("Uri: {}", document.uri))?),
        tree_item("writer-inspector.document.lines", crate::editor::writer::ui_label(format!("Lines: {}", text.lines().count()))?),
    ])?;
    let mut tree = PanelTreeBuilder::new("writer-inspector")?.section("writer-inspector.document", Some(crate::editor::writer::ui_label(labels.artifact.as_str())?), true, document_items)?;
    if document.language_id == "jack" {
        let graph = example_graph();
        let messages: Vec<String> = lint(&graph, &text).into_iter().map(|diag| diag.message).collect();
        if !messages.is_empty() {
            let indexed: Vec<(usize, &String)> = messages.iter().enumerate().collect();
            tree = tree.window_section(windows, "writer-inspector.diagnostics", Some(crate::editor::writer::ui_label(labels.diagnostics.as_str())?), true, &indexed, |(index, message)| {
                tree_item(format!("writer-inspector.diagnostics.{index}"), crate::editor::writer::ui_label(message.as_str())?)
            })?;
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
