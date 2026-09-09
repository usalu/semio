//! 📄️ Generation2d play app panel — the document tree: widgets of the current fixture.

use crate::editor::generation2d::config::Generation2dConfig;
use crate::editor::generation2d::terminology::Generation2dLabels;
use crate::{widget_id, Generation2dSnapshot};
use semio_framework_plugin::{tree_item, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const GENERATION2D_PLAY_BODY_DOCUMENT: &str = "generation2d.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(GENERATION2D_PLAY_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ Item ids are the RAW widget id (no namespace prefix) — they must equal the `graph` interaction
/// domain's target ids one-for-one so `.interaction_domain("graph")?`'s post-render presence stamping
/// (`ui_tree_stamp_presence`) can match them by plain string membership (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM). Clicks/selection are the framework's now — no
/// per-item action needed, and `_config` is unused (kept for call-site symmetry with `inspection`).
pub fn render(document: &Generation2dSnapshot, _config: &Generation2dConfig, labels: &Generation2dLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let widget_items = crate::ui_node_list(document.fixture.widgets.iter().map(|widget| tree_item(widget_id(widget), widget_id(widget).to_string())))?;
    PanelTreeBuilder::new("procedural2d-play-document")?
        .section_or_placeholder("procedural2d-play-document.widgets", Some(crate::ui_label(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL)?), true, widget_items, labels.none.as_str())?
        .interaction_domain("graph")?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
