//! 🛍️ Animate present app panel — the catalogue: tile-seeding templates and the active figure source.

use crate::artifacts::present::PresentSnapshot;
use crate::editor::animate::terminology::AnimatePresentLabels;
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};
use semio_framework_ui_contract::{button, column, field, input, section, text, ActionId, Buildable, BuiltNode, HasBase, HasChildren, InputKind, Label, Trigger, UiValue};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const PRESENT_PLAY_BODY_CATALOGUE: &str = "animate.present.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(PRESENT_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn ui_value(value: Value) -> Option<UiValue> {
    Some(match value {
        Value::Null => UiValue::Null,
        Value::Bool(value) => UiValue::Bool(value),
        Value::Number(value) => UiValue::Number(value.as_f64().unwrap_or_default()),
        Value::String(value) => UiValue::Text(semio_framework_ui_contract::UiText::try_from_string(value).ok()?),
        Value::Array(values) => {
            if !values.is_empty() {
                return None;
            }
            UiValue::List(semio_framework_ui_contract::UiList::default())
        }
        Value::Object(values) => {
            if !values.is_empty() {
                return None;
            }
            UiValue::Map(semio_framework_ui_contract::UiMap::default())
        }
    })
}

fn catalogue_button(id: &str, label: &str, action: &str, args: Option<Value>) -> BuiltNode {
    let builder = button(Label::from(label)).id(id).icon("plus");
    match args {
        Some(args) => match ui_value(args) {
            Some(args) => builder.on_with(Trigger::Activate, ActionId::v1(crate::editor::animate::PRESENT_PLAY_APP_ID, action), args).build(),
            None => builder.on(Trigger::Activate, ActionId::v1(crate::editor::animate::PRESENT_PLAY_APP_ID, action)).build(),
        },
        None => builder.on(Trigger::Activate, ActionId::v1(crate::editor::animate::PRESENT_PLAY_APP_ID, action)).build(),
    }
}

pub fn render(deck: &PresentSnapshot, labels: &AnimatePresentLabels) -> BuiltNode {
    let (source, _) = crate::artifacts::present::present_working_scene(deck);
    let templates = section(Label::from(labels.catalogue_tile_templates.as_str()))
        .id("animate.present.play.catalogue.templates")
        .default_open(true)
        .children(vec![
            text(Label::from(labels.catalogue_seed_desc.as_str())).build(),
            catalogue_button("animate.present.play.catalogue.seed-2x2", labels.catalogue_seed_2x2.as_str(), "seedGrid", Some(json!({ "rows": 2, "columns": 2 }))),
            catalogue_button("animate.present.play.catalogue.seed-3x5", labels.catalogue_seed_3x5.as_str(), "seedGrid", Some(json!({ "rows": 3, "columns": 5 }))),
            catalogue_button("animate.present.play.catalogue.add-tile", labels.catalogue_add_tile.as_str(), "addTile", None),
            catalogue_button("animate.present.play.catalogue.clear", labels.catalogue_clear_tiles.as_str(), "clearTiles", None),
        ])
        .build();
    let source_input = input(InputKind::Text).id("animate.present.play.catalogue.figure.src.readonly").value(source.src.clone()).on(Trigger::Change, ActionId::v1(crate::editor::animate::PRESENT_PLAY_APP_ID, "noMutation")).build();
    let figure = section(Label::from(labels.catalogue_figure_templates.as_str()))
        .id("animate.present.play.catalogue.figure")
        .default_open(true)
        .children(vec![
            catalogue_button(
                "animate.present.play.catalogue.figure.catalogue",
                labels.catalogue_use_figure.as_str(),
                "setSource",
                Some(dsl::ToValue::to_value(&crate::artifacts::present::present_working_scene(&crate::artifacts::present::default_present_snapshot()).0).into()),
            ),
            field(Label::from(labels.catalogue_active_source.as_str())).id("animate.present.play.catalogue.figure.src").child(source_input).build(),
            text(Label::from(format!("{}: {}", labels.catalogue_media_kind.as_str(), source.kind))).build(),
        ])
        .build();
    column().id("animate.present.play.catalogue").children(vec![templates, figure]).build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::animate::testkit::{present_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_the_framework_catalogue_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_CATALOGUE_ID);
        assert_eq!(definition.body_key.as_deref(), Some(PRESENT_PLAY_BODY_CATALOGUE));
    }

    #[semio_framework_async_macros::async_test]
    async fn catalogue_lists_templates() {
        let mut app = present_app().await;
        assert!(render_body(&mut app, PRESENT_PLAY_BODY_CATALOGUE).await.contains("animate.present.play.catalogue.templates"));
    }
}
//#endregion 🧪️Tests
