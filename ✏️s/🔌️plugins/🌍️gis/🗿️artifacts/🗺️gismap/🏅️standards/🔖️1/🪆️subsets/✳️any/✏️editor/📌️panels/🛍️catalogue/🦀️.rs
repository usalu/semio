//! 🛍️ GIS 2D play app panel — the catalogue tree: every map layer as a visibility toggle.
//!
//! 🪟️ The roster section is virtualised like every other container: it stamps its FULL extent and
//! materialises only the host's slice (`ViewModel::tree_windows`), never a `+N` row.
//!
//! 🕹️ Deliberately UNBOUND to an interaction domain: a catalogue row is not a pick of the
//! `"features"` domain, it is its own `toggleLayerVisibility` command, so the rows keep their action
//! and their catalogue-namespaced keys and stamp no `granularity`.

use crate::editor::gis2d::terminology::{gis2d_layer_label, Gis2dPlayLabels};
use crate::editor::gis2d::{gis2d_action, gis2d_layer_tree_item, ui_label, ui_value_map, ui_value_text, GIS_MAP_LAYER_IDS};
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, TreeWindows, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

//#region 🔖️Constants
pub const GIS2D_PLAY_BODY_CATALOGUE: &str = "gis2d.play.catalogue";
pub const GIS2D_PLAY_CATALOGUE_ROOT: &str = "gis2d-play-catalogue";
pub const GIS2D_PLAY_CATALOGUE_LAYERS: &str = "gis2d-play-catalogue.layers";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(GIS2D_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(labels: &Gis2dPlayLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    PanelTreeBuilder::new(GIS2D_PLAY_CATALOGUE_ROOT)?
        .window_section(windows, GIS2D_PLAY_CATALOGUE_LAYERS, Some(ui_label(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL)?), true, GIS_MAP_LAYER_IDS, |(id, _, icon)| {
            let args = ui_value_map([("layerId", ui_value_text(id)?)])?;
            gis2d_layer_tree_item(format!("{GIS2D_PLAY_CATALOGUE_ROOT}.layer.{id}"), ui_label(gis2d_layer_label(id, labels))?, None, icon, Some(gis2d_action("toggleLayerVisibility", Some(args))?), None)
        })?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
