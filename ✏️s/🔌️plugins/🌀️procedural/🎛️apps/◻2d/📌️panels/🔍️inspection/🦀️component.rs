//! 🔍️ Procedural2d play app panel — the selection inspector.

use crate::apps::procedural2d::config::Procedural2dConfig;
use crate::apps::procedural2d::terminology::Procedural2dLabels;
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use semio_framework_plugin::{
    ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_readonly_field, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiInspectorFieldGroup, UiNode, UiPresence, UiSectionNode, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

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

//#region 🔖️Render
pub fn render(document: &Procedural2dSnapshot, config: &Procedural2dConfig, labels: &Procedural2dLabels) -> UiNode {
    if config.selected_ids.is_empty() {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "procedural2d-play-inspector.empty".into(),
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
            default_open: Some(true),
            children: vec![
                semio_framework_plugin::ui_text(Label::data(format!("{} flow.fixture", labels.schema_prefix.as_str()))),
                semio_framework_plugin::ui_text(Label::data(format!("{} {}", labels.widgets_prefix.as_str(), document.fixture.widgets.len()))),
                semio_framework_plugin::ui_text(Label::data(format!("{} {}", labels.show_mode_prefix.as_str(), config.show_mode))),
            ],
            presence: UiPresence::default(),
            menu: None}]);
    }
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        presence: UiPresence::default(),
        id: "procedural2d-play-inspector.selection".into(),
        label: labels.selection.into(),
        default_open: Some(true),
        fields: vec![ui_inspector_readonly_field("procedural2d-play-inspector.ids", labels.ids, config.selected_ids.join(", "))]}])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural2d::testkit::{app, dispatch, render as render_body};
    use crate::apps::procedural2d::Procedural2dCommand;

    #[test]
    fn procedural2d_labels_translate_catalogue_and_inspector_in_german() {
        let mut app = app();
        dispatch(&mut app, Procedural2dCommand::SetLocale(crate::apps::procedural2d::commands::set_locale::SetLocale { value: "de".into() }));
        let inspector_json = render_body(&mut app, PROCEDURAL2D_PLAY_BODY_INSPECTION);
        assert!(inspector_json.contains("Elemente:"));
    }
}
//#endregion 🧪️Tests
