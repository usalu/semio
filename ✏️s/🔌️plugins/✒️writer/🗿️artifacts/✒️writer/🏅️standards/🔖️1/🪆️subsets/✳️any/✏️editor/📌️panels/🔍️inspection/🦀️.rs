//! 🔍️ Writer play app panel — document/camera inspection plus jack diagnostics.

use crate::artifacts::writer::{writer_text, WriterSnapshot};
use crate::editor::writer::config::WriterConfig;
use crate::editor::writer::terminology::WriterPlayLabels;
use semio_framework_plugin::{tree_item, BuiltNode, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiAssemblyResult, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};
use trinity::core::{example_graph, lint};

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
        tree_item("writer-inspector.document.schema", Label::data(format!("Schema: {}", document.schema))),
        tree_item("writer-inspector.document.id", Label::data(format!("Id: {}", document.id))),
        tree_item("writer-inspector.document.language", Label::data(format!("Language: {}", document.language_id))),
        tree_item("writer-inspector.document.uri", Label::data(format!("Uri: {}", document.uri))),
        tree_item("writer-inspector.document.lines", Label::data(format!("Lines: {}", text.lines().count()))),
    ])?;
    let camera_items = crate::editor::writer::ui_node_list([
        tree_item("writer-inspector.camera.x", Label::data(format!("x: {}", config.camera.x))),
        tree_item("writer-inspector.camera.y", Label::data(format!("y: {}", config.camera.y))),
        tree_item("writer-inspector.camera.zoom", Label::data(format!("zoom: {}", config.camera.zoom))),
    ])?;
    let mut tree = PanelTreeBuilder::new("writer-inspector")?.section("writer-inspector.document", Some(labels.document.into()), true, document_items)?.section("writer-inspector.camera", Some(labels.camera.into()), false, camera_items)?;
    if document.language_id == "jack" {
        let graph = example_graph();
        let messages: Vec<String> = lint(&graph, &text).into_iter().map(|diag| diag.message).take(8).collect();
        if !messages.is_empty() {
            let diagnostics = crate::editor::writer::ui_node_list(messages.into_iter().enumerate().map(|(index, message)| tree_item(format!("writer-inspector.diagnostics.{index}"), Label::data(message))))?;
            tree = tree.section("writer-inspector.diagnostics", Some(labels.diagnostics.into()), true, diagnostics)?;
        }
    }
    tree.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::writer::testkit::{dispatch, new_app, render as render_body};
    use crate::editor::writer::WriterCommand;

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
        assert_eq!(definition.body_key.as_deref(), Some(WRITER_PLAY_BODY_INSPECTION));
    }

    #[semio_framework_async_macros::async_test]
    async fn writer_labels_resolve_native_by_default() {
        let mut app = new_app();
        let inspection = render_body(&mut app, WRITER_PLAY_BODY_INSPECTION);
        assert!(inspection.contains("\"Document\""));
        assert!(inspection.contains("\"Camera\""));
    }

    #[semio_framework_async_macros::async_test]
    async fn writer_labels_resolve_german_locale() {
        let mut app = new_app();
        dispatch(&mut app, WriterCommand::SetLocale(crate::editor::writer::commands::set_locale::SetLocale { value: "de".into() }));
        let inspection = render_body(&mut app, WRITER_PLAY_BODY_INSPECTION);
        assert!(inspection.contains("Dokument"));
        assert!(inspection.contains("Kamera"));
        assert!(!inspection.contains("\"Camera\""));
    }
}
//#endregion 🧪️Tests
