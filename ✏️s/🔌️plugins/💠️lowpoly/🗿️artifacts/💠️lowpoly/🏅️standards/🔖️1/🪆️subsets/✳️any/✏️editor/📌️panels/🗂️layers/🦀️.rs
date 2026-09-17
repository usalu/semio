//! 🗂️ Lowpoly play app panel — the active object's paint layer stack.

use crate::editor::lowpoly::terminology::LowpolyLabels;
use crate::editor::lowpoly::view::{active_object, LowpolyView};
use crate::editor::lowpoly::{lowpoly_action, ui_label, ui_value_map, ui_value_number};
use crate::LowpolyPaintLayer;
use semio_framework_plugin::{tree_item_with_action, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiAssemblyResult, UiText};

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
/// 🗂️ One paint-layer row: its own `setActivePaintLayer` action, keyed by stack index. The layers panel
/// binds no interaction domain — a paint layer is not a mesh pick target — so the row keeps its binding.
fn layer_row(index: usize, layer: &LowpolyPaintLayer) -> UiAssemblyResult<BuiltNode> {
    let args = ui_value_map([("layerIndex", ui_value_number(index as f64))])?;
    let mut node = tree_item_with_action(format!("lowpoly-layer:{index}"), layer.name.clone(), Some(format!("{} · {}", layer.opacity, layer.blend_mode)), lowpoly_action("setActivePaintLayer", Some(args))?)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
        props.icon = Some(UiText::try_from_str("layers").ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly layer icon admission failed"))?);
    }
    Ok(node)
}

pub fn render(view: LowpolyView<'_>, labels: &LowpolyLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let object = active_object(view);
    let layers = object.map_or(&[][..], |entry| entry.paint_layers.as_slice());
    let active_layer = view.config.active_paint_layer;
    let entries: Vec<_> = layers.iter().enumerate().collect();
    PanelTreeBuilder::new("lowpoly-play-layers")?
        .window_section(windows, "lowpoly-play-layers.paint", Some(ui_label(labels.paint_layers.as_str())?), true, &entries, |(index, layer)| layer_row(*index, layer))?
        .selected([format!("lowpoly-layer:{active_layer}")])?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
