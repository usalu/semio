//! 🔍️ Procedural2d play app panel — the selection inspector.

use crate::artifacts::procedural2d::Procedural2dSnapshot;
use crate::editor::procedural2d::config::Procedural2dConfig;
use crate::editor::procedural2d::terminology::Procedural2dLabels;
use semio_framework_plugin::{tree_item, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

//#region 🔖️Constants
pub const PROCEDURAL2D_PLAY_BODY_INSPECTION: &str = "procedural2d.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(PROCEDURAL2D_PLAY_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

/// 🕹️ `render` carries no `InteractionView` (ArtifactApp's breaking pass only added it to
/// `handle`/`copy_fragment`/`cut_operations` — see ticket 26/08/14's w3b-summary.md), so the
/// selected-widget-details view degrades to its "no selection" default until a future wave threads
/// interaction into render. Flagged as a discovered framework gap, not worked around here.
pub fn render(document: &Procedural2dSnapshot, config: &Procedural2dConfig, labels: &Procedural2dLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let items = crate::ui_node_list([
        tree_item("procedural2d-play-inspector.schema", format!("{} flow.fixture", labels.schema_prefix.as_str())),
        tree_item("procedural2d-play-inspector.widgets", format!("{} {}", labels.widgets_prefix.as_str(), document.fixture.widgets.len())),
        tree_item("procedural2d-play-inspector.show-mode", format!("{} {}", labels.show_mode_prefix.as_str(), config.show_mode)),
    ])?;
    PanelTreeBuilder::new("procedural2d-play-inspector")?.section("procedural2d-play-inspector.empty", Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()), true, items)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedural2d::testkit::{app, dispatch, render as render_body};
    use crate::editor::procedural2d::Procedural2dCommand;

    #[test]
    fn procedural2d_labels_translate_catalogue_and_inspector_in_german() {
        let mut app = app();
        dispatch(&mut app, Procedural2dCommand::SetLocale(crate::editor::procedural2d::commands::set_locale::SetLocale { value: "de".into() }));
        let inspector_json = render_body(&mut app, PROCEDURAL2D_PLAY_BODY_INSPECTION);
        assert!(inspector_json.contains("Elemente:"));
    }
}
//#endregion 🧪️Tests
