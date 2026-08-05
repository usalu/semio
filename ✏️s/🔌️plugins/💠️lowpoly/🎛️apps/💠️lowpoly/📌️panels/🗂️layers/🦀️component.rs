//! 🗂️ Lowpoly play app panel — the active object's paint layer stack.

use crate::apps::lowpoly::lowpoly_action;
use crate::apps::lowpoly::terminology::LowpolyLabels;
use crate::apps::lowpoly::view::{active_object, LowpolyView};
use semio_framework_plugin::{tree_item_with_action, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode};
use serde_json::json;

//#region 🔖️Constants
pub const LOWPOLY_PLAY_BODY_LAYERS: &str = "lowpoly.play.layers";
const LOWPOLY_PANEL_TAB_LAYERS_ID: &str = "framework.panel.layers";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(LOWPOLY_PANEL_TAB_LAYERS_ID.into()), label: LocalizedLabel::native("Layers", "Ebenen"), group: PanelGroup::Workbench, body_key: Some(LOWPOLY_PLAY_BODY_LAYERS.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(view: LowpolyView<'_>, labels: &LowpolyLabels) -> UiNode {
    let object = active_object(view);
    let layers = object.map(|entry| entry.paint_layers.as_slice()).unwrap_or(&[]);
    let active_layer = view.config.active_paint_layer;
    let items: Vec<UiTreeItemNode> = layers
        .iter()
        .enumerate()
        .map(|(index, layer)| UiTreeItemNode {
            icon_id: Some("layers".into()),
            ..tree_item_with_action(format!("lowpoly-layer:{index}"), Label::data(layer.name.clone()), Some(format!("{} · {}", layer.opacity, layer.blend_mode)), lowpoly_action("setActivePaintLayer", Some(json!({ "layerIndex": index }))))
        })
        .collect();
    PanelTreeBuilder::new("lowpoly-play-layers").section("lowpoly-play-layers.paint", Some(labels.paint_layers.into()), true, items).selected(vec![format!("lowpoly-layer:{active_layer}")]).build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::apps::lowpoly::testkit::{app, render};

    #[test]
    fn layers_panel_lists_the_base_layer() {
        let mut a = app();
        let json = render(&mut a, super::LOWPOLY_PLAY_BODY_LAYERS);
        assert!(json.contains("lowpoly-layer:0"));
    }
}
//#endregion 🧪️Tests
