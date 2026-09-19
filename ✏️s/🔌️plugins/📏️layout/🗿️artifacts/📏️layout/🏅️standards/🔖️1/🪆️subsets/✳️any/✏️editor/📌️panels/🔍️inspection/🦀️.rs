//! 🔍️ Layout play app panel — the inspector: document summary plus the live `"elements"` selection's
//! frame fields.

use crate::editor::layout::modes::edit::windows::blueprint::config::LayoutWindowConfig;
use crate::editor::layout::terminology::LayoutLabels;
use crate::editor::layout::{ui_label, LayoutInteractionSnapshot};
use crate::{Frame, LayoutSnapshot, LAYOUT_DOCUMENT_SCHEMA};
use semio_framework_plugin::{tree_item_desc, ui_node_list, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiAssemblyResult, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

//#region 🔖️Constants
pub const LAYOUT_PLAY_BODY_INSPECTION: &str = "layout.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(LAYOUT_PLAY_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn locate_frame<'a>(document: &'a LayoutSnapshot, id: &str) -> Option<&'a Frame> {
    document.pages.iter().flat_map(|page| page.frames.iter()).find(|frame| frame.id() == id)
}

fn frame_kind_label<'a>(frame: &Frame, labels: &'a LayoutLabels) -> &'a str {
    match frame.kind_str() {
        "rect" => labels.kind_rect.as_str(),
        "text" => labels.kind_text.as_str(),
        "image" => labels.kind_image.as_str(),
        _ => labels.kind.as_str(),
    }
}

/// 🕹️ Document summary plus the first selected frame's read-only fields when the `"elements"` domain
/// is non-empty; an empty selection renders the summary alone (the interaction-less `render` twin).
pub fn render(document: &LayoutSnapshot, config: &LayoutWindowConfig, interaction: &LayoutInteractionSnapshot, labels: &LayoutLabels) -> UiAssemblyResult<BuiltNode> {
    let items = ui_node_list([
        tree_item_desc("layout-play-inspector.schema", ui_label(labels.schema.as_str())?, Some(LAYOUT_DOCUMENT_SCHEMA.into())),
        tree_item_desc("layout-play-inspector.name", ui_label(labels.name.as_str())?, Some(document.name.clone())),
        tree_item_desc("layout-play-inspector.pages", ui_label(labels.pages.as_str())?, Some(document.pages.len().to_string())),
        tree_item_desc("layout-play-inspector.active-page", ui_label(labels.active_page.as_str())?, Some(config.active_page_id.clone())),
        tree_item_desc("layout-play-inspector.selected-count", ui_label(labels.selected.as_str())?, Some(interaction.ids.len().to_string())),
    ])?;
    let builder = PanelTreeBuilder::new("layout-play-inspector")?.section("layout-play-inspector.summary", Some(ui_label(labels.inspection.as_str())?), true, items)?;
    let Some(frame_id) = interaction.ids.first() else { return builder.build() };
    let Some(frame) = locate_frame(document, frame_id) else {
        let missing = ui_node_list([tree_item_desc("layout-play-inspector.missing", ui_label(labels.selection_not_found.as_str())?, None)])?;
        return builder.section("layout-play-inspector.frame", Some(ui_label(labels.group_frame.as_str())?), true, missing)?.build();
    };
    let bounds = frame.bounds();
    let frame_items = ui_node_list([
        tree_item_desc("layout-play-inspector.frame.id", ui_label(labels.id.as_str())?, Some(frame_id.clone())),
        tree_item_desc("layout-play-inspector.frame.kind", ui_label(labels.kind.as_str())?, Some(frame_kind_label(frame, labels).to_string())),
        tree_item_desc("layout-play-inspector.frame.x", ui_label(labels.x.as_str())?, Some(format!("{:.2}", bounds.x))),
        tree_item_desc("layout-play-inspector.frame.y", ui_label(labels.y.as_str())?, Some(format!("{:.2}", bounds.y))),
        tree_item_desc("layout-play-inspector.frame.width", ui_label(labels.width.as_str())?, Some(format!("{:.2}", bounds.width))),
        tree_item_desc("layout-play-inspector.frame.height", ui_label(labels.height.as_str())?, Some(format!("{:.2}", bounds.height))),
    ])?;
    builder.section("layout-play-inspector.frame", Some(ui_label(labels.group_frame.as_str())?), true, frame_items)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semantic-contract/🦀️.rs"]
mod semantic_contract;
