//! 🔍️ GIS 2D play app panel — the inspector: map-view settings plus the selected layer's fields.

use crate::editor::gis2d::modes::edit::windows::map::config::{layer_visible, MapWindowConfig};
use crate::editor::gis2d::terminology::{gis2d_layer_label, Gis2dPlayLabels};
use crate::editor::gis2d::{ui_label, ui_node_list, Gis2dInteractionSnapshot, GIS2D_FEATURE_GRANULARITY, GIS_MAP_LAYER_IDS};
use crate::{GisMapSnapshot, GIS_MAP_SCHEMA};
use semio_framework_plugin::{tree_item_desc, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

//#region 🔖️Constants
pub const GIS2D_PLAY_BODY_INSPECTION: &str = "gis2d.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(GIS2D_PLAY_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ The map-wide summary plus the live `"features"` domain's own detail rows: a `"layer"`
/// selection adds the picked layer's id/label/visibility, a `"feature"` selection adds the picked
/// feature's id and its document kind (position/route/region). `interaction` is the framework-owned
/// selection `ArtifactEditor::render_with_request_context` threads in (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM); an empty domain renders the summary alone,
/// which is exactly what the interaction-less `render` twin shows.
pub fn render(document: &GisMapSnapshot, cfg: &MapWindowConfig, interaction: &Gis2dInteractionSnapshot, labels: &Gis2dPlayLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let visible_count = GIS_MAP_LAYER_IDS.iter().filter(|(id, _, _)| layer_visible(cfg, id)).count();
    let items = ui_node_list([
        tree_item_desc("gis2d-play-inspector.schema", ui_label(labels.schema.as_str())?, Some(GIS_MAP_SCHEMA.into())),
        tree_item_desc("gis2d-play-inspector.visible-count", ui_label(labels.layers_visible.as_str())?, Some(format!("{visible_count}/{}", GIS_MAP_LAYER_IDS.len()))),
        tree_item_desc("gis2d-play-inspector.selected-count", ui_label(labels.selected.as_str())?, Some(interaction.ids.len().to_string())),
    ])?;
    let builder = PanelTreeBuilder::new("gis2d-play-inspector")?.section("gis2d-play-inspector.summary", Some(ui_label(labels.map_layer.as_str())?), true, items)?;
    if let Some(layer) = interaction.selected_layer() {
        let layer_items = ui_node_list([
            tree_item_desc("gis2d-play-inspector.layer.id", ui_label(labels.map_layer.as_str())?, Some(layer.to_string())),
            tree_item_desc("gis2d-play-inspector.layer.label", ui_label(labels.feature.as_str())?, Some(gis2d_layer_label(layer, labels).to_string())),
            tree_item_desc("gis2d-play-inspector.layer.visible", ui_label(labels.visible.as_str())?, Some(layer_visible(cfg, layer).to_string())),
        ])?;
        return builder.section("gis2d-play-inspector.layer", Some(ui_label(labels.map_layer.as_str())?), true, layer_items)?.build();
    }
    let Some(feature) = (interaction.granularity == GIS2D_FEATURE_GRANULARITY).then(|| interaction.ids.first()).flatten() else { return builder.build() };
    let kind = feature_kind(document, feature);
    let feature_items = ui_node_list([
        tree_item_desc("gis2d-play-inspector.feature.id", ui_label(labels.feature.as_str())?, Some(feature.clone())),
        tree_item_desc("gis2d-play-inspector.feature.kind", ui_label(labels.feature_kind.as_str())?, Some(kind.to_string())),
    ])?;
    builder.section("gis2d-play-inspector.feature", Some(ui_label(labels.feature.as_str())?), true, feature_items)?.build()
}

/// 🗺️ Which document collection a selected feature id belongs to — `"unknown"` for an id the
/// document no longer carries (a stale selection surviving a delete).
fn feature_kind(document: &GisMapSnapshot, id: &str) -> &'static str {
    let has = |features: &[crate::MapFeature]| features.iter().any(|feature| feature.id == id);
    if has(&document.positions) {
        "position"
    } else if has(&document.routes) {
        "route"
    } else if has(&document.regions) {
        "region"
    } else {
        "unknown"
    }
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
