//! 🔍️ Sequence play app panel — inspection: the selected step's kind and params.

use crate::apps::sequence::terminology::SequenceLabels;
use crate::artifacts::sequence::{SequenceSnapshot, SequenceStep};
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
pub fn render(fixture: &SequenceSnapshot, selected: &[String], labels: &SequenceLabels) -> UiNode {
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
    use crate::apps::sequence::testkit::{dispatch, new_app, render as render_body};
    use crate::apps::sequence::commands::selection::set_selection::SetSelection;
    use crate::apps::sequence::SequenceCommand;

    #[test]
    fn inspection_shows_prompt_when_nothing_selected() {
        let mut app = new_app();
        assert!(render_body(&mut app, SEQUENCE_PLAY_BODY_INSPECTOR).contains("Select a step"));
    }

    #[test]
    fn inspection_shows_selected_step_kind() {
        let mut app = new_app();
        dispatch(&mut app, SequenceCommand::SetSelection(SetSelection { step_ids: vec!["step-1".into()] }));
        assert!(render_body(&mut app, SEQUENCE_PLAY_BODY_INSPECTOR).contains("state.set"));
    }
}
//#endregion 🧪️Tests
