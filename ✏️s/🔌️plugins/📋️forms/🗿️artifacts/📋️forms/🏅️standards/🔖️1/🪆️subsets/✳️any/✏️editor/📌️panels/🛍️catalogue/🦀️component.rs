//! 🛍️ Forms play app panel — the catalogue: the draggable question-kind palette plus quick actions.

use crate::editor::forms::config::FormsConfig;
use crate::editor::forms::terminology::FormsLabels;
use crate::editor::forms::{catalogue_kinds, forms_action, parse_contributions};
use semio_framework_plugin::{tree_item_with_action, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};
use serde_json::json;
use std::collections::HashMap;

//#region 🔖️Constants
pub const FORMS_PLAY_BODY_CATALOGUE: &str = "forms.play.catalogue";
const FORMS_QUESTION_DRAG_MIME: &str = "application/x-semio-forms-question-kind";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(FORMS_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub async fn render(config: &FormsConfig, labels: &FormsLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let contributions = parse_contributions(config);
    let kind_items: Vec<UiTreeItemNode> = catalogue_kinds(&contributions, labels)
        .into_iter()
        .map(|(kind, label, icon)| {
            let mut drag_data = HashMap::new();
            drag_data.insert(FORMS_QUESTION_DRAG_MIME.into(), json!({ "kind": kind }).to_string());
            UiTreeItemNode {
                icon_id: Some(icon),
                draggable: Some(true),
                drag_data: Some(drag_data),
                menu: None,
                ..tree_item_with_action(format!("forms-play-catalogue.{kind}"), Label::data(label), Some(kind.clone()), forms_action("addQuestion", Some(json!({ "kind": kind }))))?
            }
        })
        .collect();
    let action_items = vec![
        UiTreeItemNode { icon_id: Some("plus".into()), menu: None, ..tree_item_with_action("forms-play-catalogue.add-step", labels.add_step, None, forms_action("addStep", None))? },
        UiTreeItemNode { icon_id: Some("type".into()), menu: None, ..tree_item_with_action("forms-play-catalogue.add-question", labels.add_text_question, None, forms_action("addQuestion", Some(json!({ "kind": "text" }))))? },
    ];
    PanelTreeBuilder::new("forms-play-catalogue")?
        .section("forms-play-catalogue.kinds", Some(Label::data(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL)), true, kind_items)?
        .section("forms-play-catalogue.actions", Some(labels.actions.into()), true, action_items)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::forms::testkit::{forms_app, render as render_body};
    use crate::editor::forms::FORMS_PLAY_BODY_CATALOGUE as BODY_CATALOGUE;

    #[semio_framework_async_macros::async_test]
    async fn catalogue_lists_question_kinds() {
        let mut app = forms_app();
        let json = render_body(&mut app, BODY_CATALOGUE);
        assert!(json.contains("forms-play-catalogue.text"));
        assert!(json.contains("forms-play-catalogue.add-step"));
    }

    #[semio_framework_async_macros::async_test]
    async fn catalogue_items_export_the_question_kind_drag_mime() {
        let mut app = forms_app();
        let json = render_body(&mut app, BODY_CATALOGUE);
        assert!(json.contains(FORMS_QUESTION_DRAG_MIME));
        assert!(json.contains(r#""draggable":true"#) || json.contains(r#""draggable": true"#));
    }
}
//#endregion 🧪️Tests
