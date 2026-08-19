//! 🛍️ Writer play app panel — the language catalogue (currently a single static jack description).

use crate::editor::writer::terminology::WriterPlayLabels;
use semio_framework_plugin::{ui_declarative_sections_to_tree, ui_text, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode, UiPresence, UiSectionNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

//#region 🔖️Constants
pub const WRITER_PLAY_BODY_CATALOGUE: &str = "writer.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(WRITER_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub async fn render(labels: &WriterPlayLabels) -> UiNode {
    ui_declarative_sections_to_tree(&[UiSectionNode { id: "writer-catalogue".into(), label: Some(labels.language.into()), default_open: Some(true), children: vec![ui_text(labels.jack_description)], presence: UiPresence::default(), menu: None }])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::writer::testkit::{new_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn renders_catalogue_panel() {
        let mut app = new_app();
        assert!(render_body(&mut app, WRITER_PLAY_BODY_CATALOGUE).contains("jack"));
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_the_framework_catalogue_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_CATALOGUE_ID);
        assert_eq!(definition.body_key.as_deref(), Some(WRITER_PLAY_BODY_CATALOGUE));
    }
}
//#endregion 🧪️Tests
