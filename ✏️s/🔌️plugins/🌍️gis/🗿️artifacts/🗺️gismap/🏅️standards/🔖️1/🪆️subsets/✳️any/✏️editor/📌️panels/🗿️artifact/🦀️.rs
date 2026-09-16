//! 📄️ GIS 2D play app panel — the document tree: the map's layer stack, selectable.
//!
//! 🪟️ The layer section is virtualised the framework way: it publishes its FULL extent through
//! `TreeWindow { total, offset }` and materialises only the slice the host asked for
//! (`ViewModel::tree_windows`, read once per render as [`TreeWindows`]). There is no page cursor and
//! no `+N` continuation row, and the same law holds when the roster stops being a compile-time array.
//!
//! 🎯️ Rows carry a `granularity` and no binding of their own: the tree root owns the single
//! `interactionSelect` binding [`PanelTreeBuilder::interaction_domain`] stamps, and every row is
//! keyed by its RAW layer id, which is what the framework matches selection presence against.

use crate::editor::gis2d::modes::edit::windows::map::config::MapWindowConfig;
use crate::editor::gis2d::terminology::{gis2d_layer_label, Gis2dPlayLabels};
use crate::editor::gis2d::{gis2d_layer_tree_item, ui_label, GIS2D_INTERACTION_DOMAIN, GIS2D_LAYER_GRANULARITY, GIS2D_PLAY_APP_ID, GIS_MAP_LAYER_IDS};
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, TreeWindows, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const GIS2D_PLAY_BODY_ARTIFACT: &str = "gis2d.play.artifact";
pub const GIS2D_PLAY_DOCUMENT_ROOT: &str = "gis2d-play-document";
pub const GIS2D_PLAY_DOCUMENT_LAYERS: &str = "gis2d-play-document.layers";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"),
        group: PanelGroup::Workbench,
        body_key: Some(GIS2D_PLAY_BODY_ARTIFACT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ `_cfg` is unused now — layer selection moved into the framework-owned `"features"` interaction
/// domain (granularity `"layer"`, ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM);
/// `.interaction_domain(..)?` below has the framework's renderer translate clicks into the tree's
/// one injected `interactionSelect` and stamp presence from `InteractionState`, replacing the deleted
/// `.selected()?`/`.selection_change()` calls.
pub fn render(_cfg: &MapWindowConfig, labels: &Gis2dPlayLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    PanelTreeBuilder::new(GIS2D_PLAY_DOCUMENT_ROOT)?
        .window_section(windows, GIS2D_PLAY_DOCUMENT_LAYERS, Some(ui_label(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL)?), true, GIS_MAP_LAYER_IDS, |(id, _, icon)| {
            gis2d_layer_tree_item(id, ui_label(gis2d_layer_label(id, labels))?, Some((*id).into()), icon, None, Some(GIS2D_LAYER_GRANULARITY))
        })?
        .interaction_domain(GIS2D_PLAY_APP_ID, GIS2D_INTERACTION_DOMAIN)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
