//! 📄️ Generation2d play app panel — the document tree: widgets of the current fixture.

use crate::editor::generation2d::config::Generation2dConfig;
use crate::editor::generation2d::terminology::Generation2dLabels;
use crate::editor::generation2d::GENERATION2D_PLAY_APP_ID;
use crate::{widget_id, Generation2dSnapshot};
use semio_framework_plugin::{tree_item, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, TreeWindows, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const GENERATION2D_PLAY_BODY_ARTIFACT: &str = "generation2d.play.artifact";
pub const GENERATION2D_PLAY_DOCUMENT_WIDGETS: &str = "procedural2d-play-document.widgets";
/// 🕹️ The `graph` domain this tree picks into, and the granularity a widget row is a target at.
const GENERATION2D_INTERACTION_DOMAIN: &str = "graph";
const GENERATION2D_GRANULARITY_NODE: &str = "node";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"),
        group: PanelGroup::Workbench,
        body_key: Some(GENERATION2D_PLAY_BODY_ARTIFACT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ Item ids are the RAW widget id (no namespace prefix) — they must equal the `graph` interaction
/// domain's target ids one-for-one, which is what lets the row declare a `granularity` and pick
/// through the tree's single `interactionSelect` binding instead of carrying an argument map of its
/// own (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM, windowed 26/09/16). `_config` is
/// unused (kept for call-site symmetry with `inspection`).
pub fn render(document: &Generation2dSnapshot, _config: &Generation2dConfig, labels: &Generation2dLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    PanelTreeBuilder::new("procedural2d-play-document")?
        .window_section_or_placeholder(
            windows,
            GENERATION2D_PLAY_DOCUMENT_WIDGETS,
            Some(crate::ui_label(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL)?),
            true,
            &document.host_snapshot.widgets,
            |widget| {
                let mut node = tree_item(widget_id(widget), widget_id(widget).to_string())?;
                if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
                    props.granularity = Some(crate::ui_text(GENERATION2D_GRANULARITY_NODE)?);
                }
                Ok(node)
            },
            crate::ui_label(labels.none.as_str())?,
        )?
        .interaction_domain(GENERATION2D_PLAY_APP_ID, GENERATION2D_INTERACTION_DOMAIN)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
