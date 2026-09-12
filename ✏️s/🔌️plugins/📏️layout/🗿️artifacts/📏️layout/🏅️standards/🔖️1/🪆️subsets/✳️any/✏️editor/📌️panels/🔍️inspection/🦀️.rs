//! 🔍️ Layout play app panel — the inspector: a document summary (was field editors for the current
//! selection; see `render`'s doc comment for why that's gone).

use crate::editor::layout::modes::edit::windows::blueprint::config::LayoutWindowConfig;
use crate::editor::layout::terminology::LayoutLabels;
use crate::editor::layout::ui_label;
use crate::{LayoutSnapshot, LAYOUT_DOCUMENT_SCHEMA};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PluginAssemblyError, UiAssemblyResult, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};
use semio_framework_ui_contract::{Buildable, HasBase, HasChildren};

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
    let mut section =
        semio_framework_ui_contract::section(ui_label(labels.inspection.as_str())?).default_open(true).try_id("layout-play-inspector.empty").map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout inspector id admission failed"))?;
    for (index, value) in [
        format!("{}: {}", labels.schema.as_str(), LAYOUT_DOCUMENT_SCHEMA),
        format!("{}: {}", labels.name.as_str(), doc.name),
        format!("{}: {}", labels.pages.as_str(), doc.pages.len()),
        format!("{}: {}", labels.active_page.as_str(), config.active_page_id),
    ]
    .into_iter()
    .enumerate()
    {
        let child = semio_framework_ui_contract::text(ui_label(value)?)
            .try_id(format!("layout-inspector.summary.{index}"))
            .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout summary key admission failed"))?
            .try_build()
            .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout summary text admission failed"))?;
        section = section.try_child(child).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout inspector child admission failed"))?;
    }
    section.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout inspector node admission failed"))
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
