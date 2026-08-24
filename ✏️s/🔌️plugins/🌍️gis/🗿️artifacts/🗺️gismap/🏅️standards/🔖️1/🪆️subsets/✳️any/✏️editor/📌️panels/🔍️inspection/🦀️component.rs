//! 🔍️ GIS 2D play app panel — the inspector: map-view settings plus the selected layer's fields.

use crate::artifacts::gismap::GIS_MAP_SCHEMA;
use crate::editor::gis2d::config::{layer_visible, Gis2dConfig};
use crate::editor::gis2d::terminology::Gis2dPlayLabels;
use crate::editor::gis2d::{ui_label, ui_node_list, GIS_MAP_LAYER_IDS};
use semio_framework_plugin::{
    tree_item_desc, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};

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
/// 🕹️ `ArtifactEditor::render` carries no `InteractionView` (a known SDK gap — see
/// `w3c-summary.md`'s flagged `open_context_menu`/render follow-up), so this panel can no longer
/// tell which layer is currently selected and always shows the map-wide summary now — the
/// per-selected-layer detail branch (id/label/visible-toggle) that used to read `cfg.selected_ids`
/// is gone with it (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub fn render(cfg: &Gis2dConfig, labels: &Gis2dPlayLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let visible_count = GIS_MAP_LAYER_IDS.iter().filter(|(id, _, _)| layer_visible(cfg, id)).count();
    let items = ui_node_list([
        tree_item_desc("gis2d-play-inspector.schema", ui_label(labels.schema.as_str())?, Some(GIS_MAP_SCHEMA.into())),
        tree_item_desc(
            "gis2d-play-inspector.visible-count",
            ui_label(labels.layers_visible.as_str())?,
            Some(format!("{visible_count}/{}", GIS_MAP_LAYER_IDS.len())),
        ),
    ])?;
    PanelTreeBuilder::new("gis2d-play-inspector")?
        .section("gis2d-play-inspector.summary", Some(ui_label(labels.map_layer.as_str())?), true, items)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::gis2d::testkit::{app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn the_inspector_always_summarises_the_schema_and_visible_count() {
        let mut app = app();
        let json = render_body(&mut app, GIS2D_PLAY_BODY_INSPECTION);
        assert!(json.contains(GIS_MAP_SCHEMA));
        assert!(json.contains(&format!("{}/{}", GIS_MAP_LAYER_IDS.len(), GIS_MAP_LAYER_IDS.len())));
    }

    #[semio_framework_async_macros::async_test]
    async fn the_definition_binds_the_framework_inspection_tab_to_this_body() {
        let definition = definition();
        assert!(matches!(definition.group, PanelGroup::Details));
        assert_eq!(definition.body_key.as_deref(), Some(GIS2D_PLAY_BODY_INSPECTION));
    }
}
//#endregion 🧪️Tests
