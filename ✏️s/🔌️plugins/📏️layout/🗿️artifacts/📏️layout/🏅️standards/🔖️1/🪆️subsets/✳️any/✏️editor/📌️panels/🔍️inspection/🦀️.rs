//! 🔍️ Layout play app panel — the inspector: a document summary (was field editors for the current
//! selection; see `render`'s doc comment for why that's gone).

use crate::editor::layout::modes::edit::windows::blueprint::config::LayoutWindowConfig;
use crate::editor::layout::terminology::LayoutLabels;
use crate::editor::layout::ui_label;
use crate::{LayoutSnapshot, LAYOUT_DOCUMENT_SCHEMA};
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
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM known gap: this used to switch on
/// `config.selected_ids` to show one field-editor group per selected page/frame (name, bounds,
/// fill/stroke, story content, wrap mode, link path). Selection is now framework-owned
/// (`InteractionView`, threaded only into `handle`/`copy_fragment`/`cut_operations`) and
/// `ArtifactApp::render` never gained that parameter, so this panel has no live selection to render
/// against and always falls through to the document summary below — the same gap gis2d's and
/// puzzle3d's inspection panels flag (see this ticket's w3b-summary.md). Not fixed here (framework
/// file, out of this crate's remit).
pub fn render(doc: &LayoutSnapshot, config: &LayoutWindowConfig, labels: &LayoutLabels) -> UiAssemblyResult<BuiltNode> {
    let items = ui_node_list([
        tree_item_desc("layout-play-inspector.schema", ui_label(labels.schema.as_str())?, Some(LAYOUT_DOCUMENT_SCHEMA.into())),
        tree_item_desc("layout-play-inspector.name", ui_label(labels.name.as_str())?, Some(doc.name.clone())),
        tree_item_desc("layout-play-inspector.pages", ui_label(labels.pages.as_str())?, Some(doc.pages.len().to_string())),
        tree_item_desc("layout-play-inspector.active-page", ui_label(labels.active_page.as_str())?, Some(config.active_page_id.clone())),
    ])?;
    PanelTreeBuilder::new("layout-play-inspector")?
        .section("layout-play-inspector.summary", Some(ui_label(labels.inspection.as_str())?), true, items)?
        .build()
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
