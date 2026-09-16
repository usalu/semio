//! 📄️ Animate presentation app panel — the document tree: tiles of the current deck.

use crate::editor::animate::terminology::AnimatePresentationLabels;
use crate::editor::animate::ui_label;
use crate::editor::animate::{PRESENTATION_INTERACTION_DOMAIN, PRESENTATION_INTERACTION_GRANULARITY, PRESENTATION_PLAY_APP_ID};
use crate::PresentationSnapshot;
use semio_framework_plugin::{tree_item_desc, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, TreeWindows, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};
use semio_framework_ui_contract::BuiltNode;

//#region 🔖️Constants
pub const PRESENTATION_PLAY_BODY_ARTIFACT: &str = "animate.presentation.play.artifact";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"),
        group: PanelGroup::Workbench,
        body_key: Some(PRESENTATION_PLAY_BODY_ARTIFACT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ No per-row selection `action`: the tree is bound to the `tiles` interaction domain via
/// `.interaction_domain(...)?` below, so the framework auto-injects `interactionSelect` for row
/// clicks — never declare that yourself (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
///
/// 🪟️ A real deck outgrows one node's fixed child capacity, so the tile list is a windowed section.
pub fn render(deck: &PresentationSnapshot, labels: &AnimatePresentationLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let (_, tiles) = crate::presentation_working_scene(deck);
    PanelTreeBuilder::new("animate-presentation-play")?
        .window_section_or_placeholder(
            windows,
            "animate-presentation-play.tiles",
            Some(ui_label(labels.tiles_section.as_str())?),
            true,
            &tiles,
            |tile| {
                let mut node = tree_item_desc(tile.id.clone(), ui_label(&tile.name)?, Some(format!("x={:.3} y={:.3} w={:.3} h={:.3}", tile.crop.x, tile.crop.y, tile.crop.width, tile.crop.height)))?;
                if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
                    props.granularity = Some(crate::editor::animate::ui_text(PRESENTATION_INTERACTION_GRANULARITY)?);
                }
                Ok(node)
            },
            ui_label(labels.no_tiles.as_str())?,
        )?
        .interaction_domain(PRESENTATION_PLAY_APP_ID, PRESENTATION_INTERACTION_DOMAIN)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
