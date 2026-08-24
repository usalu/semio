//! 📄️ Forms play app panel — the document tree: steps and their questions.

use crate::artifacts::forms::schema::forms_play_step_tree_id;
use crate::artifacts::forms::{forms_steps, FormsSnapshot};
use crate::editor::forms::terminology::FormsLabels;
use crate::editor::forms::{forms_action, FORMS_INTERACTION_FIELDS};
use semio_framework_plugin::{tree_item_desc, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const FORMS_PLAY_BODY_DOCUMENT: &str = "forms.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(FORMS_PLAY_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: item ids are the SAME canonical
/// `fields` domain target ids `FormsPlayApp::interaction_topology` declares — steps at the "section"
/// granularity via `forms_play_step_tree_id`, questions at the "field" granularity via their own raw
/// id — the framework stamps this tree's selection/hover presence from that domain
/// (`.interaction_domain`) and prunes stale ids through that same topology, so no per-item click
/// action is declared here anymore (clicks are translated into `interactionSelect` generically)?.
pub async fn render(spec: &FormsSnapshot, labels: &FormsLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let step_items: Vec<UiTreeItemNode> = forms_steps(spec)
        .iter()
        .map(|step| {
            let question_items: Vec<UiTreeItemNode> = step
                .blocks
                .iter()
                .map(|question| UiTreeItemNode { icon_id: Some("help-circle".into()), draggable: Some(true), menu: None, ..tree_item_desc(question.id.clone(), Label::data(question.label.clone()), Some(question.kind.clone()))? })
                .collect();
            UiTreeItemNode {
                icon_id: Some("list-tree".into()),
                default_open: Some(true),
                draggable: Some(true),
                items: Some(question_items),
                menu: None,
                ..tree_item_desc(forms_play_step_tree_id(&step.id), Label::data(step.title.clone()), Some(format!("{} questions", step.blocks.len())))?
            }
        })
        .collect();
    PanelTreeBuilder::new("forms-play-document")?
        .section_or_placeholder("forms-play-document.steps", Some(Label::data(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL)), true, step_items, labels.no_steps_tree_item)?
        .interaction_domain(FORMS_INTERACTION_FIELDS)?
        .drop_action(forms_action("dropQuestionKind", None))
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::forms::testkit::{forms_app, render as render_body};
    use crate::editor::forms::FORMS_PLAY_BODY_DOCUMENT as BODY_DOCUMENT;

    #[semio_framework_async_macros::async_test]
    async fn document_tree_declares_drop_action() {
        let mut app = forms_app();
        let json = render_body(&mut app, BODY_DOCUMENT);
        assert!(json.contains(r#""dropAction""#));
        assert!(json.contains("dropQuestionKind"));
    }

    #[semio_framework_async_macros::async_test]
    async fn document_lists_steps() {
        let mut app = forms_app();
        let json = render_body(&mut app, BODY_DOCUMENT);
        assert!(json.contains("forms-play-document.steps"));
    }

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_the_framework_document_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
        assert_eq!(definition.body_key.as_deref(), Some(FORMS_PLAY_BODY_DOCUMENT));
    }
}
//#endregion 🧪️Tests
