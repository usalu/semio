//! 🛍️ Writer play app panel — the language catalogue (currently a single static jack description).

use crate::editor::writer::terminology::WriterPlayLabels;
use semio_framework_plugin::{tree_item, BuiltNode, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiAssemblyResult, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

//#region 🔖️Constants
pub const WRITER_PLAY_BODY_CATALOGUE: &str = "writer.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
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
pub fn render(labels: &WriterPlayLabels) -> UiAssemblyResult<BuiltNode> {
    let entries = crate::editor::writer::ui_node_list([tree_item("writer-catalogue.jack", Label::data(labels.jack_description.as_str()))])?;
    PanelTreeBuilder::new("writer-catalogue")?.section("writer-catalogue.language", Some(labels.language.into()), true, entries)?.build()
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
