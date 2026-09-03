//! 🛍️ Animate presentation app panel — the catalogue: tile-seeding templates and the active figure source.

use crate::artifacts::presentation::PresentationSnapshot;
use crate::editor::animate::terminology::AnimatePresentationLabels;
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};
use semio_framework_ui_contract::{button, column, field, input, section, text, ActionId, Buildable, BuiltNode, HasBase, HasChildren, InputKind, Label, Trigger, UiValue};

//#region 🔖️Constants
pub const PRESENTATION_PLAY_BODY_CATALOGUE: &str = "animate.presentation.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(PRESENTATION_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn ui_value(value: dsl::DslValue) -> Option<UiValue> {
    Some(match value {
        dsl::DslValue::Null => UiValue::Null,
        dsl::DslValue::Bool(value) => UiValue::Bool(value),
        dsl::DslValue::Number(value) => UiValue::Number(value.as_f64()),
        dsl::DslValue::String(value) => UiValue::Text(semio_framework_ui_contract::UiText::try_from_string(value).ok()?),
        dsl::DslValue::Array(values) => {
            if !values.is_empty() {
                return None;
            }
            UiValue::List(semio_framework_ui_contract::UiList::default())
        }
        dsl::DslValue::Object(values) => {
            if !values.is_empty() {
                return None;
            }
            UiValue::Map(semio_framework_ui_contract::UiMap::default())
        }
    })
}

fn catalogue_button(id: &str, label: &str, action: &str, args: Option<dsl::DslValue>) -> BuiltNode {
    let builder = button(Label::from(label)).id(id).icon("plus");
    match args {
        Some(args) => match ui_value(args) {
            Some(args) => builder.on_with(Trigger::Activate, ActionId::v1(crate::editor::animate::PRESENTATION_PLAY_APP_ID, action), args).build(),
            None => builder.on(Trigger::Activate, ActionId::v1(crate::editor::animate::PRESENTATION_PLAY_APP_ID, action)).build(),
        },
        None => builder.on(Trigger::Activate, ActionId::v1(crate::editor::animate::PRESENTATION_PLAY_APP_ID, action)).build(),
    }
}

pub fn render(deck: &PresentationSnapshot, labels: &AnimatePresentationLabels) -> BuiltNode {
    let (source, _) = crate::artifacts::presentation::presentation_working_scene(deck);
    let templates = section(Label::from(labels.catalogue_tile_templates.as_str()))
        .id("animate.presentation.play.catalogue.templates")
        .default_open(true)
        .children(vec![
            text(Label::from(labels.catalogue_seed_desc.as_str())).build(),
            catalogue_button("animate.presentation.play.catalogue.seed-2x2", labels.catalogue_seed_2x2.as_str(), "seedGrid", Some(dsl::DslValue::object([("rows".to_string(), dsl::DslValue::uint(2)), ("columns".to_string(), dsl::DslValue::uint(2))]))),
            catalogue_button("animate.presentation.play.catalogue.seed-3x5", labels.catalogue_seed_3x5.as_str(), "seedGrid", Some(dsl::DslValue::object([("rows".to_string(), dsl::DslValue::uint(3)), ("columns".to_string(), dsl::DslValue::uint(5))]))),
            catalogue_button("animate.presentation.play.catalogue.add-tile", labels.catalogue_add_tile.as_str(), "addTile", None),
            catalogue_button("animate.presentation.play.catalogue.clear", labels.catalogue_clear_tiles.as_str(), "clearTiles", None),
        ])
        .build();
    let source_input = input(InputKind::Text).id("animate.presentation.play.catalogue.figure.src.readonly").value(source.src.clone()).on(Trigger::Change, ActionId::v1(crate::editor::animate::PRESENTATION_PLAY_APP_ID, "noMutation")).build();
    let figure = section(Label::from(labels.catalogue_figure_templates.as_str()))
        .id("animate.presentation.play.catalogue.figure")
        .default_open(true)
        .children(vec![
            catalogue_button(
                "animate.presentation.play.catalogue.figure.catalogue",
                labels.catalogue_use_figure.as_str(),
                "setSource",
                Some(dsl::ToValue::to_value(&crate::artifacts::presentation::presentation_working_scene(&crate::artifacts::presentation::default_presentation_snapshot()).0)),
            ),
            field(Label::from(labels.catalogue_active_source.as_str())).id("animate.presentation.play.catalogue.figure.src").child(source_input).build(),
            text(Label::from(format!("{}: {}", labels.catalogue_media_kind.as_str(), source.kind))).build(),
        ])
        .build();
    column().id("animate.presentation.play.catalogue").children(vec![templates, figure]).build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::animate::testkit::{presentation_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_the_framework_catalogue_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_CATALOGUE_ID);
        assert_eq!(definition.body_key.as_deref(), Some(PRESENTATION_PLAY_BODY_CATALOGUE));
    }

    #[semio_framework_async_macros::async_test]
    async fn catalogue_lists_templates() {
        let mut app = presentation_app().await;
        assert!(render_body(&mut app, PRESENTATION_PLAY_BODY_CATALOGUE).await.contains("animate.presentation.play.catalogue.templates"));
    }
}
//#endregion 🧪️Tests
