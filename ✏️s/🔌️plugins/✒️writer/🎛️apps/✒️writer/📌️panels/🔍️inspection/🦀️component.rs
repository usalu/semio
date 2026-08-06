//! 🔍️ Writer play app panel — document/camera inspection plus jack diagnostics.

use crate::apps::writer::config::WriterConfig;
use crate::apps::writer::terminology::WriterPlayLabels;
use crate::artifacts::writer::WriterProjection;
use semio_framework_plugin::{ui_declarative_sections_to_tree, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode, UiPresence, UiSectionNode, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};
use trinity::core::{example_graph, lint, Diagnostic};

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
pub fn render(document: &WriterProjection, config: &WriterConfig, labels: &WriterPlayLabels) -> UiNode {
    let mut sections = vec![
        UiSectionNode {
            id: "writer-inspector.document".into(),
            label: Some(labels.document.into()),
            default_open: Some(true),
            children: vec![
                ui_text(Label::data(format!("Schema: {}", document.schema))),
                ui_text(Label::data(format!("Id: {}", document.id))),
                ui_text(Label::data(format!("Language: {}", document.language_id))),
                ui_text(Label::data(format!("Uri: {}", document.uri))),
                ui_text(Label::data(format!("Lines: {}", document.text.lines().count()))),
            ],
            presence: UiPresence::default(),
            menu: None,
        },
        UiSectionNode {
            id: "writer-inspector.camera".into(),
            label: Some(labels.camera.into()),
            default_open: Some(false),
            children: vec![ui_text(Label::data(format!("x: {}", config.camera.x))), ui_text(Label::data(format!("y: {}", config.camera.y))), ui_text(Label::data(format!("zoom: {}", config.camera.zoom)))],
            presence: UiPresence::default(),
            menu: None,
        },
    ];
    if document.language_id == "jack" {
        let graph = example_graph();
        let messages: Vec<String> = lint(&graph, &document.text).into_iter().map(|diag: Diagnostic| diag.message).take(8).collect();
        if !messages.is_empty() {
            sections.push(UiSectionNode {
                id: "writer-inspector.diagnostics".into(),
                label: Some(labels.diagnostics.into()),
                default_open: Some(true),
                children: messages.into_iter().map(Label::data).map(ui_text).collect(),
                presence: UiPresence::default(),
                menu: None,
            });
        }
    }
    ui_declarative_sections_to_tree(&sections)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::writer::testkit::{dispatch, new_app, render as render_body};
    use crate::apps::writer::WriterCommand;

    #[test]
    fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
        assert_eq!(definition.body_key.as_deref(), Some(WRITER_PLAY_BODY_INSPECTION));
    }

    #[test]
    fn writer_labels_resolve_native_by_default() {
        let mut app = new_app();
        let inspection = render_body(&mut app, WRITER_PLAY_BODY_INSPECTION);
        assert!(inspection.contains("\"Document\""));
        assert!(inspection.contains("\"Camera\""));
    }

    #[test]
    fn writer_labels_resolve_german_locale() {
        let mut app = new_app();
        dispatch(&mut app, WriterCommand::SetLocale(crate::apps::writer::commands::locale::set_locale::SetLocale { value: "de".into() }));
        let inspection = render_body(&mut app, WRITER_PLAY_BODY_INSPECTION);
        assert!(inspection.contains("Dokument"));
        assert!(inspection.contains("Kamera"));
        assert!(!inspection.contains("\"Camera\""));
    }
}
//#endregion 🧪️Tests
