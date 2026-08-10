//! 📄️ GIS 2D play app panel — the document tree: the map's layer stack, selectable.

use crate::apps::gis2d::config::Gis2dConfig;
use crate::apps::gis2d::terminology::{gis2d_layer_label, Gis2dPlayLabels};
use crate::apps::gis2d::{gis2d_action, gis2d_layer_tree_item, GIS_MAP_LAYER_IDS};
use semio_framework_plugin::{Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};
use serde_json::json;

//#region 🔖️Constants
pub const GIS2D_PLAY_BODY_DOCUMENT: &str = "gis2d.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(GIS2D_PLAY_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(cfg: &Gis2dConfig, labels: &Gis2dPlayLabels) -> UiNode {
    let builder = PanelTreeBuilder::new("gis2d-play-document");
    let layer_items: Vec<UiTreeItemNode> = GIS_MAP_LAYER_IDS
        .iter()
        .map(|(id, _, icon)| gis2d_layer_tree_item(builder.item_id("layer", id), Label::data(gis2d_layer_label(id, labels)), Some((*id).into()), icon, gis2d_action("setSelection", Some(json!({ "ids": [id] })))))
        .collect();
    builder
        .section("gis2d-play-document.layers", Some(Label::data(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL)), true, layer_items)
        .selected(cfg.selected_ids.iter().map(|id| format!("gis2d-play-document.layer.{id}")).collect())
        .selection_change(gis2d_action("setSelection", None))
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::gis2d::testkit::{app, render as render_body};

    #[test]
    fn document_lists_map_layers() {
        let mut app = app();
        assert!(render_body(&mut app, GIS2D_PLAY_BODY_DOCUMENT).contains("gis2d-play-document.layer.raster"));
    }

    #[test]
    fn the_definition_binds_the_framework_document_tab_to_this_body() {
        let definition = definition();
        assert!(matches!(definition.kind, PanelTabKind::App(ref id) if id == FRAMEWORK_PANEL_TAB_ARTIFACT_ID));
        assert_eq!(definition.body_key.as_deref(), Some(GIS2D_PLAY_BODY_DOCUMENT));
    }
}
//#endregion 🧪️Tests
