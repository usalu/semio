//! 🔍️ Shooting play app panel — the inspector: fields for the selected shot (falling back to the active
//! shot, then a schema summary). Per-asset fields dropped — see this file's `render` doc comment.

use crate::artifacts::shooting::{ShootingShot, ShootingSnapshot, SHOOTING_DOCUMENT_SCHEMA};
use crate::editor::shooting::config::ShootingConfig;
use crate::editor::shooting::terminology::ShootingLabels;
use semio_framework_plugin::{
    ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_mixed_number, ui_inspector_readonly_field, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiFieldNode, UiInspectorFieldGroup, UiNode,
    UiPresence, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};

//#region 🔖️Constants
pub const SHOOTING_PLAY_BODY_INSPECTION: &str = "shooting.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(SHOOTING_PLAY_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
async fn shot_inspector_group(shot: &ShootingShot, labels: &ShootingLabels) -> UiInspectorFieldGroup {
    let width_mixed = ui_inspector_mixed_number(&[shot.width as f64]);
    let height_mixed = ui_inspector_mixed_number(&[shot.height as f64]);
    UiInspectorFieldGroup {
        id: "shooting-play-inspector.shot".into(),
        label: labels.shot.into(),
        default_open: None,
        presence: UiPresence::default(),
        fields: vec![
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "shooting-play-inspector.shot.label".into(),
                label: labels.field_label.into(),
                child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
                    presence: UiPresence::default(),
                    id: "shooting-play-inspector.shot.label.input".into(),
                    input_kind: "text".into(),
                    value: shot.label.clone(),
                    placeholder: None,
                    commit: None,
                    on_change: crate::editor::shooting::shooting_action("patchShot", Some(serde_json::json!({ "shotId": shot.id, "field": "label" }))),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
            ui_inspector_readonly_field("shooting-play-inspector.shot.format", labels.field_format, &shot.format),
            ui_inspector_readonly_field("shooting-play-inspector.shot.shape", labels.field_shape, &shot.shape),
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "shooting-play-inspector.shot.width".into(),
                label: labels.field_width.into(),
                child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
                    presence: UiPresence::default(),
                    id: "shooting-play-inspector.shot.width.input".into(),
                    input_kind: "number".into(),
                    value: width_mixed.value.to_string(),
                    placeholder: None,
                    commit: None,
                    on_change: crate::editor::shooting::shooting_action("patchShot", Some(serde_json::json!({ "shotId": shot.id, "field": "width" }))),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "shooting-play-inspector.shot.height".into(),
                label: labels.field_height.into(),
                child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
                    presence: UiPresence::default(),
                    id: "shooting-play-inspector.shot.height.input".into(),
                    input_kind: "number".into(),
                    value: height_mixed.value.to_string(),
                    placeholder: None,
                    commit: None,
                    on_change: crate::editor::shooting::shooting_action("patchShot", Some(serde_json::json!({ "shotId": shot.id, "field": "height" }))),
                    min: None,
                    max: None,
                    step: None,
                    accept: None,
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                menu: None,
            }),
        ],
    }
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the asset-selection branch this used
/// to have (`if !cfg.selected_asset_ids.is_empty() { ... an asset field group ... }`) — and the field
/// group it rendered — are DELETED: asset selection is the framework-owned `"assets"` interaction
/// domain now, and `render` has no `InteractionView` parameter (unlike `handle`/`copy_fragment`/
/// `cut_operations`), so it is unreachable here. Documented reduced-fidelity gap.
pub async fn render(snapshot: &ShootingSnapshot, cfg: &ShootingConfig, labels: &ShootingLabels) -> UiNode {
    if !cfg.selected_shot_ids.is_empty() {
        let shot_id = &cfg.selected_shot_ids[0];
        if let Some(shot) = snapshot.shots.iter().find(|entry| &entry.id == shot_id) {
            return ui_inspector_groups_to_tree(&[shot_inspector_group(shot, labels)]);
        }
    }
    if let Some(shot) = crate::artifacts::shooting::schema::active_shot(snapshot) {
        return ui_inspector_groups_to_tree(&[shot_inspector_group(shot, labels)]);
    }
    ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
        id: "shooting-play-inspector.empty".into(),
        label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
        default_open: Some(true),
        children: vec![ui_text(Label::data(format!("Schema: {SHOOTING_DOCUMENT_SCHEMA}"))), ui_text(Label::data(format!("Shots: {}", snapshot.shots.len()))), ui_text(Label::data(format!("Assets: {}", snapshot.assets.len())))],
        presence: UiPresence::default(),
        menu: None,
    }])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::shooting::testkit::{render as render_body, shooting_app};

    #[semio_framework_async_macros::async_test]
    async fn inspector_falls_back_to_the_active_shot() {
        let mut app = shooting_app();
        let json = render_body(&mut app, SHOOTING_PLAY_BODY_INSPECTION);
        assert!(json.contains("Shot"));
    }
}
//#endregion 🧪️Tests
