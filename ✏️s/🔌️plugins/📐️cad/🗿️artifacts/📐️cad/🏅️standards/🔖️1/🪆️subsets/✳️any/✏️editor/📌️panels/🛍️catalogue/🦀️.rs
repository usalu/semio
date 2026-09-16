//! 🛍️ CAD play app panel — the typology catalogue: one clickable row per creatable object typology.

use crate::editor::cad::terminology::{typology_label, CadLabels};
use crate::editor::cad::{cad_action, cad_tree_item, ui_label, ui_value_map, ui_value_text, TYPOLOGY_CATALOG};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, TreeWindows, UiAssemblyResult, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

//#region 🔖️Constants
pub const CAD_PLAY_BODY_CATALOGUE: &str = "cad.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(CAD_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🛍️ Every catalogue row keeps its own `addObject` binding (it is an app action, not a domain
/// pick), and the section is windowed like any other list so the host owns its open/scroll state.
pub fn build_catalogue_tree(labels: &CadLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    PanelTreeBuilder::new("cad-play-catalogue")?
        .window_section(windows, "cad-play-catalogue.typologies", Some(ui_label(labels.typologies.as_str())?), true, TYPOLOGY_CATALOG, |entry| {
            // 🔑️ `UiMapBuilder::push` admits keys in strictly ascending order only.
            let args = ui_value_map([("modelDefinitionId", ui_value_text(entry.model_definition_id)?), ("typology", ui_value_text(entry.typology)?)])?;
            cad_tree_item(format!("cad-play-catalogue.{}", entry.typology), typology_label(entry.typology, labels), Some(entry.icon), cad_action("addObject", Some(args))?)
        })?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
