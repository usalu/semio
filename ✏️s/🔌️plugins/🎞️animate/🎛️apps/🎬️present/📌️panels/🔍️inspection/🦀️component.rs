//! 🔍️ Animate present app panel — the inspector: field editors for the selected tile(s).

use crate::apps::present::animate_present_action;
use crate::apps::present::terminology::AnimatePresentLabels;
use crate::artifacts::present::{FigureTileDraft, PresentSnapshot};
use semio_framework_plugin::{
    ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_mixed_number, ui_inspector_mixed_text, ui_inspector_readonly_field, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiButtonNode, UiFieldNode,
    UiInputNode, UiInspectorFieldGroup, UiNode, UiPresence, UiSectionNode, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use serde_json::json;

//#region 🔖️Constants
pub const PRESENT_PLAY_BODY_DETAILS: &str = "animate.present.play.details";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()), label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"), group: PanelGroup::Details, body_key: Some(PRESENT_PLAY_BODY_DETAILS.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn inspector_crop_field(tile_ids: &[String], field: &str, label: impl Into<Label>, values: &[f64]) -> UiNode {
    let mixed = ui_inspector_mixed_number(values);
    UiNode::Field(UiFieldNode {
        id: format!("animate.present.play.tile.crop.{field}"),
        label: label.into(),
        child: Box::new(UiNode::Input(UiInputNode {
            id: format!("animate.present.play.tile.crop.{field}.input"),
            input_kind: "number".into(),
            value: if mixed.uniform { format!("{:.6}", values.first().copied().unwrap_or(0.0)) } else { String::new() },
            placeholder: if mixed.uniform { None } else { Some(Label::data(UI_INSPECTOR_MIXED_PLACEHOLDER)) },
            commit: Some("blur".into()),
            on_change: animate_present_action("patchTileCrops", Some(json!({ "ids": tile_ids, "field": field }))),
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
    })
}

pub fn render(deck: &PresentSnapshot, selected: &[String], labels: &AnimatePresentLabels) -> UiNode {
    if selected.is_empty() {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "animate.present.play.details.empty".into(),
            presence: UiPresence::default(),
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
            default_open: Some(true),
            children: vec![ui_text(labels.details_select_tile)],
            menu: None,
        }]);
    }
    let tiles: Vec<&FigureTileDraft> = selected.iter().filter_map(|id| deck.tiles.iter().find(|tile| &tile.id == id)).collect();
    if tiles.is_empty() {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "animate.present.play.details.not-found".into(),
            presence: UiPresence::default(),
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
            default_open: Some(true),
            children: vec![ui_text(labels.details_tile_not_found)],
            menu: None,
        }]);
    }
    let tile_ids: Vec<String> = tiles.iter().map(|tile| tile.id.clone()).collect();
    let name_mixed = ui_inspector_mixed_text(&tiles.iter().map(|tile| tile.name.clone()).collect::<Vec<_>>());
    let mut identity_fields: Vec<UiNode> = vec![UiNode::Field(UiFieldNode {
        id: "animate.present.play.tile.name".into(),
        label: labels.field_name.into(),
        child: Box::new(UiNode::Input(UiInputNode {
            id: "animate.present.play.tile.name.input".into(),
            input_kind: "text".into(),
            value: name_mixed.value,
            placeholder: name_mixed.placeholder.map(Label::data),
            commit: Some("blur".into()),
            on_change: animate_present_action("renameTiles", Some(json!({ "ids": tile_ids }))),
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
    })];
    identity_fields.push(ui_inspector_readonly_field(
        "animate.present.play.tile.id",
        labels.field_id,
        if tile_ids.len() == 1 { tile_ids.first().cloned().unwrap_or_default() } else { format!("{} {}", tile_ids.len(), labels.selected_suffix.as_str()) },
    ));
    if tile_ids.len() == 1 {
        identity_fields.push(UiNode::Button(UiButtonNode {
            id: Some(format!("animate.present.play.tile.{}.delete", tile_ids[0])),
            icon_id: "trash-2".into(),
            label: labels.delete_tile.into(),
            action: animate_present_action("deleteTile", Some(json!({ "id": tile_ids[0] }))),
            style: None,
            presence: UiPresence::default(),
            menu: None,
        }));
    }
    identity_fields.push(UiNode::Button(UiButtonNode {
        id: Some("animate.present.play.details.delete-selection".into()),
        icon_id: "trash-2".into(),
        label: labels.delete_selection.into(),
        action: animate_present_action("deleteSelection", None),
        style: None,
        presence: UiPresence::default(),
        menu: None,
    }));
    let groups = vec![
        UiInspectorFieldGroup {
            id: "animate.present.play.details.crop".into(),
            label: labels.group_crop.into(),
            default_open: None,
            presence: UiPresence::default(),
            fields: vec![
                inspector_crop_field(&tile_ids, "x", labels.field_x, &tiles.iter().map(|tile| tile.crop.x).collect::<Vec<_>>()),
                inspector_crop_field(&tile_ids, "y", labels.field_y, &tiles.iter().map(|tile| tile.crop.y).collect::<Vec<_>>()),
                inspector_crop_field(&tile_ids, "width", labels.field_width, &tiles.iter().map(|tile| tile.crop.width).collect::<Vec<_>>()),
                inspector_crop_field(&tile_ids, "height", labels.field_height, &tiles.iter().map(|tile| tile.crop.height).collect::<Vec<_>>()),
            ],
        },
        UiInspectorFieldGroup { id: "animate.present.play.details.identity".into(), label: labels.group_identity.into(), default_open: None, presence: UiPresence::default(), fields: identity_fields },
    ];
    ui_inspector_groups_to_tree(&groups)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::present::testkit::{present_app, render as render_body};

    #[test]
    fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
        assert_eq!(definition.body_key.as_deref(), Some(PRESENT_PLAY_BODY_DETAILS));
    }

    #[test]
    fn empty_selection_prompts_to_select_a_tile() {
        let mut app = present_app();
        assert!(render_body(&mut app, PRESENT_PLAY_BODY_DETAILS).contains("Select a tile"));
    }
}
//#endregion 🧪️Tests
