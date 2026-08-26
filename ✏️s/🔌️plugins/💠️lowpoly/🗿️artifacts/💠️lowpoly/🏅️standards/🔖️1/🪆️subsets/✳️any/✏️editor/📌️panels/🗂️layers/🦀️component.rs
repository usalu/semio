//! 🗂️ Lowpoly play app panel — the active object's paint layer stack.

use crate::editor::lowpoly::terminology::LowpolyLabels;
use crate::editor::lowpoly::view::{active_object, LowpolyView};
use crate::editor::lowpoly::{lowpoly_action, ui_node_list, ui_value_map, ui_value_number};
use semio_framework_plugin::{tree_item_with_action, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiText};

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
pub fn render(view: LowpolyView<'_>, labels: &LowpolyLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let object = active_object(view);
    let layers = object.map_or(&[][..], |entry| entry.paint_layers.as_slice());
    let active_layer = view.config.active_paint_layer;
    let items = ui_node_list(layers.iter().enumerate().map(|(index, layer)| {
        let args = ui_value_map([("layerIndex", ui_value_number(index as f64))])?;
        let mut node = tree_item_with_action(format!("lowpoly-layer:{index}"), Label::data(layer.name.clone()), Some(format!("{} · {}", layer.opacity, layer.blend_mode)), lowpoly_action("setActivePaintLayer", Some(args))?)?;
        if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
            props.icon = Some(UiText::try_from_str("layers").ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly layer icon admission failed"))?);
        }
        Ok(node)
    }))?;
    PanelTreeBuilder::new("lowpoly-play-layers")?.section("lowpoly-play-layers.paint", Some(labels.paint_layers.into()), true, items)?.selected([format!("lowpoly-layer:{active_layer}")])?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::editor::lowpoly::testkit::{app, render};

    #[semio_framework_async_macros::async_test]
    async fn layers_panel_lists_the_base_layer() {
        let mut a = app();
        let json = render(&mut a, super::LOWPOLY_PLAY_BODY_LAYERS).await;
        assert!(json.contains("lowpoly-layer:0"));
    }
}
//#endregion 🧪️Tests
