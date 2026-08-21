//! 🛍️ Animate present app panel — the catalogue: tile-seeding templates and the active figure source.

use crate::artifacts::present::PresentSnapshot;
use crate::editor::animate::animate_present_action;
use crate::editor::animate::terminology::AnimatePresentLabels;
use semio_framework_plugin::{
    ui_declarative_sections_to_tree, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiButtonNode, UiFieldNode, UiInputNode, UiNode, UiPresence, UiSectionNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const PRESENT_PLAY_BODY_CATALOGUE: &str = "animate.present.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
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
async fn catalogue_button(id: &str, label: impl Into<Label>, action: &str, args: Option<Value>) -> UiNode {
    UiNode::Button(UiButtonNode { id: Some(id.into()), icon_id: "plus".into(), label: label.into(), action: animate_present_action(action, args), style: None, presence: UiPresence::default(), menu: None })
}

pub async fn render(deck: &PresentSnapshot, labels: &AnimatePresentLabels) -> UiNode {
    let (source, _) = crate::artifacts::present::present_working_scene(deck);
    ui_declarative_sections_to_tree(&[
        UiSectionNode {
            id: "animate.present.play.catalogue.templates".into(),
            presence: UiPresence::default(),
            label: Some(labels.catalogue_tile_templates.into()),
            default_open: Some(true),
            children: vec![
                ui_text(labels.catalogue_seed_desc),
                catalogue_button("animate.present.play.catalogue.seed-2x2", labels.catalogue_seed_2x2, "seedGrid", Some(json!({ "rows": 2, "columns": 2 }))),
                catalogue_button("animate.present.play.catalogue.seed-3x5", labels.catalogue_seed_3x5, "seedGrid", Some(json!({ "rows": 3, "columns": 5 }))),
                catalogue_button("animate.present.play.catalogue.add-tile", labels.catalogue_add_tile, "addTile", None),
                catalogue_button("animate.present.play.catalogue.clear", labels.catalogue_clear_tiles, "clearTiles", None),
            ],
            menu: None,
        },
        UiSectionNode {
            id: "animate.present.play.catalogue.figure".into(),
            presence: UiPresence::default(),
            label: Some(labels.catalogue_figure_templates.into()),
            default_open: Some(true),
            children: vec![
                catalogue_button("animate.present.play.catalogue.figure.catalogue", labels.catalogue_use_figure, "setSource", Some(json!(crate::artifacts::present::present_working_scene(&crate::artifacts::present::default_present_snapshot()).0))),
                UiNode::Field(UiFieldNode {
                    id: "animate.present.play.catalogue.figure.src".into(),
                    label: labels.catalogue_active_source.into(),
                    child: Box::new(UiNode::Input(UiInputNode {
                        id: "animate.present.play.catalogue.figure.src.readonly".into(),
                        input_kind: "text".into(),
                        value: source.src.clone(),
                        placeholder: None,
                        commit: None,
                        on_change: animate_present_action("noMutation", None),
                        min: None,
                        max: None,
                        step: None,
                        accept: None,
                        presence: UiPresence::default(),
                        menu: None,
                    })),
                    description: None,
                    required: None,
                    error: None,
                    presence: UiPresence::default(),
                    menu: None,
                }),
                ui_text(Label::data(format!("{}: {}", labels.catalogue_media_kind.as_str(), source.kind))),
            ],
            menu: None,
        },
    ])
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
        let mut app = present_app();
        assert!(render_body(&mut app, PRESENT_PLAY_BODY_CATALOGUE).contains("animate.present.play.catalogue.templates"));
    }
}
//#endregion 🧪️Tests
