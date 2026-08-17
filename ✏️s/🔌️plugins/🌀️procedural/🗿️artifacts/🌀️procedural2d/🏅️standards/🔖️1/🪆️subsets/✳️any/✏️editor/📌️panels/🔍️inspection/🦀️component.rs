//! 🔍️ Procedural2d play app panel — the selection inspector.

use crate::editor::procedural2d::config::Procedural2dConfig;
use crate::editor::procedural2d::terminology::Procedural2dLabels;
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use semio_framework_plugin::{ui_declarative_sections_to_tree, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode, UiPresence, UiSectionNode, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

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
        children: Vec::new()}
}
//#endregion 🔖️Definition

/// 🕹️ `render` carries no `InteractionView` (ArtifactApp's breaking pass only added it to
/// `handle`/`copy_fragment`/`cut_operations` — see ticket 26/08/14's w3b-summary.md), so the
/// selected-widget-details view degrades to its "no selection" default until a future wave threads
/// interaction into render. Flagged as a discovered framework gap, not worked around here.
pub fn render(document: &Procedural2dSnapshot, config: &Procedural2dConfig, labels: &Procedural2dLabels) -> UiNode {
    ui_declarative_sections_to_tree(&[UiSectionNode {
        id: "procedural2d-play-inspector.empty".into(),
        label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
        default_open: Some(true),
        children: vec![
            semio_framework_plugin::ui_text(Label::data(format!("{} flow.fixture", labels.schema_prefix.as_str()))),
            semio_framework_plugin::ui_text(Label::data(format!("{} {}", labels.widgets_prefix.as_str(), document.fixture.widgets.len()))),
            semio_framework_plugin::ui_text(Label::data(format!("{} {}", labels.show_mode_prefix.as_str(), config.show_mode))),
        ],
        presence: UiPresence::default(),
        menu: None}])
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
