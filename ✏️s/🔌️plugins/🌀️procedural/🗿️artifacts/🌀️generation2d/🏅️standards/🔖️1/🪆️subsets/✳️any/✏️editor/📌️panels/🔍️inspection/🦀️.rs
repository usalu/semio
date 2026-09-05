//! 🔍️ Generation2d play app panel — the selection inspector.

use crate::artifacts::generation2d::Generation2dSnapshot;
use crate::editor::generation2d::config::Generation2dConfig;
use crate::editor::generation2d::terminology::Generation2dLabels;
use semio_framework_plugin::{tree_item, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

//#region 🔖️Constants
pub const GENERATION2D_PLAY_BODY_INSPECTION: &str = "generation2d.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(GENERATION2D_PLAY_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

/// 🕹️ `render` carries no `InteractionView` (ArtifactApp's breaking pass only added it to
/// `handle`/`copy_fragment`/`cut_operations` — see ticket 26/08/14's w3b-summary.md), so the
/// selected-widget-details view degrades to its "no selection" default until a future wave threads
/// interaction into render. Flagged as a discovered framework gap, not worked around here.
pub fn render(document: &Generation2dSnapshot, config: &Generation2dConfig, labels: &Generation2dLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let items = crate::ui_node_list([
        tree_item("procedural2d-play-inspector.schema", format!("{} flow.fixture", labels.schema_prefix.as_str())),
        tree_item("procedural2d-play-inspector.widgets", format!("{} {}", labels.widgets_prefix.as_str(), document.fixture.widgets.len())),
        tree_item("procedural2d-play-inspector.show-mode", format!("{} {}", labels.show_mode_prefix.as_str(), config.show_mode)),
    ])?;
    PanelTreeBuilder::new("procedural2d-play-inspector")?.section("procedural2d-play-inspector.empty", Some(crate::ui_label(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)?), true, items)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::generation2d::testkit::{app, dispatch, render as render_body};
    use crate::editor::generation2d::Generation2dCommand;

    #[semio_framework_async_macros::async_test]
    async fn generation2d_labels_translate_catalogue_and_inspector_in_german() {
        let mut app = app().await;
        dispatch(&mut app, Generation2dCommand::SetLocale(crate::editor::generation2d::commands::set_locale::SetLocale { value: "de".into() })).await;
        let inspector_json = render_body(&mut app, GENERATION2D_PLAY_BODY_INSPECTION).await;
        assert!(inspector_json.contains("Elemente:"));
    }
}
//#endregion 🧪️Tests
