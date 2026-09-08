//! 🛍️ Writer play app panel — the language catalogue (currently a single static jack description).

use crate::editor::writer::terminology::WriterPlayLabels;
use semio_framework_plugin::{tree_item, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiAssemblyResult, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

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
    let entries = crate::editor::writer::ui_node_list([tree_item("writer-catalogue.jack", crate::editor::writer::ui_label(labels.jack_description.as_str())?)])?;
    PanelTreeBuilder::new("writer-catalogue")?.section("writer-catalogue.language", Some(crate::editor::writer::ui_label(labels.language.as_str())?), true, entries)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
