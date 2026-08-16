//! 🔍️ Forms play app panel — the document-wide summary (schema, step count, question count).

use crate::artifacts::forms::{forms_steps, FormsSnapshot};
use semio_framework_plugin::{ui_declarative_sections_to_tree, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode, UiPresence, UiSectionNode, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

//#region 🔖️Constants
pub const FORMS_PLAY_BODY_INSPECTION: &str = "forms.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(FORMS_PLAY_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ `ArtifactEditor::render` carries no `InteractionView` (a known SDK gap — matches `gis2d`'s and
/// `note`'s inspection panel precedent, ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), so
/// this panel can no longer tell which question(s) are selected — it always shows the document-wide
/// summary now; the per-selected-question kind editor (label/kind/required/options/vector fields/…,
/// driven by `patchQuestions`/`patchQuestionOptions`/`patchVectorField`) that used to read
/// `cfg.selected_ids` is gone with it.
pub fn render(spec: &FormsSnapshot) -> UiNode {
    ui_declarative_sections_to_tree(&[UiSectionNode {
        id: "forms-play-inspector.summary".into(),
        label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
        default_open: Some(true),
        children: vec![
            ui_text(Label::data(format!("Schema: {}", crate::artifacts::forms::FORMS_DOCUMENT_SCHEMA))),
            ui_text(Label::data(format!("Steps: {}", forms_steps(spec).len()))),
            ui_text(Label::data(format!("Questions: {}", crate::artifacts::forms::schema::flatten_questions(spec).len()))),
        ],
        presence: UiPresence::default(),
        menu: None,
    }])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::forms::testkit::{forms_app, render as render_body};
    use crate::editor::forms::FORMS_PLAY_BODY_INSPECTION as BODY_INSPECTION;

    #[test]
    fn the_inspector_always_shows_the_document_summary() {
        let mut app = forms_app();
        let json = render_body(&mut app, BODY_INSPECTION);
        assert!(json.contains("forms-play-inspector.summary"));
    }

    #[test]
    fn definition_binds_the_framework_inspection_tab_to_this_body() {
        let definition = definition();
        assert_eq!(definition.body_key.as_deref(), Some(FORMS_PLAY_BODY_INSPECTION));
        assert!(matches!(definition.group, PanelGroup::Details));
    }
}
//#endregion 🧪️Tests
