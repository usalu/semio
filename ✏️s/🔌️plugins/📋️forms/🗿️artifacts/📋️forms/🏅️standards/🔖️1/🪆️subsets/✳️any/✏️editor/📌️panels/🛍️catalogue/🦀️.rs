//! 🛍️ Forms play app panel — the catalogue: the draggable question-kind palette plus quick actions.

use crate::editor::forms::config::FormsConfig;
use crate::editor::forms::terminology::FormsLabels;
use crate::editor::forms::{catalogue_kinds, forms_action, parse_contributions};
use semio_framework_plugin::{tree_item_with_action, tree_item_with_action_draggable, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};
use serde_json::json;

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
    let mut kind_items = semio_framework_plugin::UiFixedList::default();
    for (kind, label, icon) in catalogue_kinds(&contributions, labels) {
        let args = crate::editor::forms::ui_value_map([("kind", crate::editor::forms::ui_value_text(&kind)?)])?;
        let drag_data = json!({ FORMS_QUESTION_DRAG_MIME: json!({ "kind": kind }).to_string() });
        let mut item = tree_item_with_action_draggable(format!("forms-play-catalogue.{kind}"), Label::data(label), Some(kind.clone()), forms_action("addQuestion", Some(args))?, &drag_data)?;
        if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
            props.icon = Some(icon);
        }
        kind_items.try_push(item).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.kinds", "fixed question-kind catalogue admission failed"))?;
    }
    let text_args = crate::editor::forms::ui_value_map([("kind", crate::editor::forms::ui_value_text("text")?)])?;
    let mut add_step = tree_item_with_action("forms-play-catalogue.add-step", labels.add_step, None, forms_action("addStep", None)?)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut add_step.component {
        props.icon = Some(semio_framework_plugin::UiText::try_from_str("plus").ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.icon", "fixed catalogue icon admission failed"))?);
    }
    let mut add_question = tree_item_with_action("forms-play-catalogue.add-question", labels.add_text_question, None, forms_action("addQuestion", Some(text_args))?)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut add_question.component {
        props.icon = Some(semio_framework_plugin::UiText::try_from_str("type").ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.icon", "fixed catalogue icon admission failed"))?);
    }
    let mut action_items = semio_framework_plugin::UiFixedList::default();
    action_items.try_push(add_step).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.actions", "fixed catalogue action admission failed"))?;
    action_items.try_push(add_question).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.actions", "fixed catalogue action admission failed"))?;
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
