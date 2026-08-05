//! 📄️ Forms play app panel — the document tree: steps and their questions.

use crate::apps::forms::forms_action;
use crate::apps::forms::terminology::FormsLabels;
use crate::artifacts::forms::engine::forms_play_step_tree_id;
use crate::artifacts::forms::FormSpec;
use semio_framework_plugin::{tree_item_with_action, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL};
use serde_json::json;

//#region 🔖️Constants
pub const FORMS_PLAY_BODY_DOCUMENT: &str = "forms.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_DOCUMENT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(FORMS_PLAY_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(spec: &FormSpec, selected_ids: &[String], labels: &FormsLabels) -> UiNode {
    let step_items: Vec<UiTreeItemNode> = spec
        .steps
        .iter()
        .map(|step| {
            let question_items: Vec<UiTreeItemNode> = step
                .blocks
                .iter()
                .map(|question| UiTreeItemNode {
                    icon_id: Some("help-circle".into()),
                    draggable: Some(true),
                    menu: None,
                    ..tree_item_with_action(question.id.clone(), Label::data(question.label.clone()), Some(question.kind.clone()), forms_action("setSelection", Some(json!({ "ids": [question.id.clone()] }))))
                })
                .collect();
            UiTreeItemNode {
                icon_id: Some("list-tree".into()),
                default_open: Some(true),
                draggable: Some(true),
                items: Some(question_items),
                menu: None,
                ..tree_item_with_action(forms_play_step_tree_id(&step.id), Label::data(step.title.clone()), Some(format!("{} questions", step.blocks.len())), forms_action("setSelection", Some(json!({ "ids": [] }))))
            }
        })
        .collect();
    PanelTreeBuilder::new("forms-play-document")
        .section_or_placeholder("forms-play-document.steps", Some(Label::data(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL)), true, step_items, labels.no_steps_tree_item)
        .selected(selected_ids.to_vec())
        .selection_change(forms_action("setSelection", None))
        .drop_action(forms_action("dropQuestionKind", None))
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::forms::testkit::{forms_app, render as render_body};
    use crate::apps::forms::FORMS_PLAY_BODY_DOCUMENT as BODY_DOCUMENT;

    #[test]
    fn document_tree_declares_drop_action() {
        let mut app = forms_app();
        let json = render_body(&mut app, BODY_DOCUMENT);
        assert!(json.contains(r#""dropAction""#));
        assert!(json.contains("dropQuestionKind"));
    }

    #[test]
    fn document_lists_steps() {
        let mut app = forms_app();
        let json = render_body(&mut app, BODY_DOCUMENT);
        assert!(json.contains("forms-play-document.steps"));
    }

    #[test]
    fn definition_binds_the_framework_document_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_DOCUMENT_ID);
        assert_eq!(definition.body_key.as_deref(), Some(FORMS_PLAY_BODY_DOCUMENT));
    }
}
//#endregion 🧪️Tests
