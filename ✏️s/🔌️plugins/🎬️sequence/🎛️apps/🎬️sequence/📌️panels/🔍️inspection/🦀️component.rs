//! 🔍️ Sequence play app panel — inspection: the selected step's kind and params.

use crate::apps::sequence::terminology::SequenceLabels;
use crate::artifacts::sequence::{SequenceFixture, SequenceStep};
use semio_framework_plugin::{
    ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_text, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiInspectorFieldGroup, UiNode, UiPresence, UiSectionNode, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use semio_framework_plugin::Label;

//#region 🔖️Constants
pub const SEQUENCE_PLAY_BODY_INSPECTOR: &str = "sequence.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(SEQUENCE_PLAY_BODY_INSPECTOR.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(fixture: &SequenceFixture, selected: &[String], labels: &SequenceLabels) -> UiNode {
    if selected.is_empty() {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "sequence-play-inspector.empty".into(),
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
            default_open: Some(true),
            presence: UiPresence::default(),
            children: vec![ui_text(labels.select_prompt)],
            menu: None,
        }]);
    }
    let steps: Vec<&SequenceStep> = selected.iter().filter_map(|id| fixture.steps.iter().find(|step| &step.id == id)).collect();
    if steps.is_empty() {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "sequence-play-inspector.missing".into(),
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
            default_open: Some(true),
            presence: UiPresence::default(),
            children: vec![ui_text(labels.step_not_found)],
            menu: None,
        }]);
    }
    let step_ids: Vec<String> = steps.iter().map(|step| step.id.clone()).collect();
    let mut fields = vec![
        ui_inspector_readonly_field("sequence-play-inspector.kind", labels.kind, steps[0].kind.clone()),
        ui_inspector_readonly_field("sequence-play-inspector.params", labels.params, serde_json::to_string(&steps[0].params).unwrap_or_else(|_| "{}".into())),
    ];
    if step_ids.len() == 1 {
        fields.insert(0, ui_inspector_readonly_field("sequence-play-inspector.id", labels.id, step_ids[0].clone()));
    }
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { presence: UiPresence::default(), id: "sequence-play-inspector.step".into(), label: labels.step.into(), default_open: None, fields }])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::sequence::config::SequenceConfig;
    use crate::apps::sequence::terminology::sequence_play_labels;
    use crate::apps::sequence::testkit::{new_app, render as render_body};
    use semio_framework_plugin::PluginApp;

    #[test]
    fn inspection_shows_prompt_when_nothing_selected() {
        let mut app = new_app();
        assert!(render_body(&mut app, SEQUENCE_PLAY_BODY_INSPECTOR).contains("Select a step"));
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `ArtifactApp::render` carries no
    /// `InteractionView` (only `handle`/`copy_fragment`/`cut_operations` gained one — see this
    /// ticket's `w3b-summary.md`), so the live app can never feed this panel a real selection today —
    /// `SequencePlayApp::render` always calls this with an empty slice, a documented framework gap
    /// (the same one `space`'s node-graph canvas rendering and context menu carry). This exercises
    /// `render`'s own selected-step branch directly instead of through the app's dispatch/render loop.
    #[test]
    fn inspection_shows_selected_step_kind() {
        let mut app = new_app();
        let fixture = app.snapshot().expect("projection").to_fixture();
        let labels = sequence_play_labels(&SequenceConfig::default());
        let node = render(&fixture, &["step-1".to_string()], labels);
        assert!(serde_json::to_string(&node).unwrap().contains("state.set"));
    }
}
//#endregion 🧪️Tests
