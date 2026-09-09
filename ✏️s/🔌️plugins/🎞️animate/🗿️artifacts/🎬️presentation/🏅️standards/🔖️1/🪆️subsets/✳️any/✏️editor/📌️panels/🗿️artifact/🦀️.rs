//! 📄️ Animate presentation app panel — the document tree: tiles of the current deck.

use crate::editor::animate::terminology::AnimatePresentationLabels;
use crate::editor::animate::ui_label;
use crate::editor::animate::PRESENTATION_INTERACTION_DOMAIN;
use crate::PresentationSnapshot;
use semio_framework_plugin::{tree_item_desc, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};
use semio_framework_ui_contract::BuiltNode;

//#region 🔖️Constants
pub const PRESENTATION_PLAY_BODY_DOCUMENT: &str = "animate.presentation.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(PRESENTATION_PLAY_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn ui_node_list(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        nodes.try_push(value?).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "animate presentation tile admission failed"))?;
    }
    Ok(nodes)
}

/// 🕹️ No per-row selection `action`: the tree is bound to the `tiles` interaction domain via
/// `.interaction_domain(...)?` below, so the framework auto-injects `interactionSelect` for row
/// clicks — never declare that yourself (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub fn render(deck: &PresentationSnapshot, labels: &AnimatePresentationLabels) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let (_, tiles) = crate::presentation_working_scene(deck);
    let items = ui_node_list(tiles.iter().map(|tile| tree_item_desc(tile.id.clone(), ui_label(&tile.name)?, Some(format!("x={:.3} y={:.3} w={:.3} h={:.3}", tile.crop.x, tile.crop.y, tile.crop.width, tile.crop.height)))))?;
    PanelTreeBuilder::new("animate-presentation-play")?
        .section_or_placeholder("animate-presentation-play.tiles", Some(ui_label(labels.tiles_section.as_str())?), true, items, ui_label(labels.no_tiles.as_str())?)?
        .interaction_domain(PRESENTATION_INTERACTION_DOMAIN)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
