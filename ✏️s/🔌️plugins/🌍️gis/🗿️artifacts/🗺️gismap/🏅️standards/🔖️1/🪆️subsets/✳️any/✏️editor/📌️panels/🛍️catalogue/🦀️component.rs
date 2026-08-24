//! 🛍️ GIS 2D play app panel — the catalogue tree: every map layer as a visibility toggle.

use crate::editor::gis2d::terminology::{gis2d_layer_label, Gis2dPlayLabels};
use crate::editor::gis2d::{gis2d_action, gis2d_layer_tree_item, ui_node_list, ui_value_map, ui_value_text, GIS_MAP_LAYER_IDS};
use semio_framework_plugin::{Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

//#region 🔖️Constants
pub const GIS2D_PLAY_BODY_CATALOGUE: &str = "gis2d.play.catalogue";
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
pub fn render(labels: &Gis2dPlayLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let builder = PanelTreeBuilder::new("gis2d-play-catalogue")?;
    let items = ui_node_list(GIS_MAP_LAYER_IDS.iter().map(|(id, _, icon)| {
        let args = ui_value_map([("layerId", ui_value_text(id)?)])?;
        gis2d_layer_tree_item(builder.item_id("layer", id)?, Label::data(gis2d_layer_label(id, labels)), None, icon, Some(gis2d_action("toggleLayerVisibility", Some(args))?))
    }))?;
    builder.section("gis2d-play-catalogue.layers", Some(Label::data(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL)), true, items)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::gis2d::testkit::{app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn catalogue_lists_layer_toggles() {
        let mut app = app();
        assert!(render_body(&mut app, GIS2D_PLAY_BODY_CATALOGUE).contains("gis2d-play-catalogue.layer.water"));
    }

    #[semio_framework_async_macros::async_test]
    async fn the_definition_binds_the_framework_catalogue_tab_to_this_body() {
        assert_eq!(definition().body_key.as_deref(), Some(GIS2D_PLAY_BODY_CATALOGUE));
    }
}
//#endregion 🧪️Tests
