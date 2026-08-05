//! 🔍️ VCS play app panel — the inspector: title/counter/status/notes/tags fields for the document.

use crate::apps::vcs::terminology::VcsPlayLabels;
use crate::apps::vcs::vcs_action;
use crate::artifacts::vcs::VcsDemoProjection;
use semio_framework_plugin::{
    ui_inspector_groups_to_tree, ui_inspector_readonly_field, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiFieldNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiPresence, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde_json::json;

//#region 🔖️Constants
pub const VCS_PLAY_BODY_INSPECTION: &str = "vcs.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(VCS_PLAY_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(projection: &VcsDemoProjection, labels: &VcsPlayLabels) -> UiNode {
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "vcs-play-inspector".into(),
        label: labels.title.into(),
        default_open: Some(true),
        presence: UiPresence::default(),
        fields: vec![
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "vcs-play-inspector.title".into(),
                label: labels.title.into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    presence: UiPresence::default(),
                    id: "vcs-play-inspector.title.input".into(),
                    input_kind: "text".into(),
                    value: projection.title.clone(),
                    placeholder: None,
                    commit: Some("blur".into()),
                    on_change: vcs_action("patchProjection", Some(json!({ "field": "title" }))),
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
                id: "vcs-play-inspector.counter".into(),
                label: labels.counter.into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    presence: UiPresence::default(),
                    id: "vcs-play-inspector.counter.input".into(),
                    input_kind: "number".into(),
                    value: projection.counter.to_string(),
                    placeholder: None,
                    commit: Some("blur".into()),
                    on_change: vcs_action("patchProjection", Some(json!({ "field": "counter" }))),
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
                id: "vcs-play-inspector.status".into(),
                label: labels.status.into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    presence: UiPresence::default(),
                    id: "vcs-play-inspector.status.input".into(),
                    input_kind: "text".into(),
                    value: projection.status.clone(),
                    placeholder: None,
                    commit: Some("blur".into()),
                    on_change: vcs_action("patchProjection", Some(json!({ "field": "status" }))),
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
                id: "vcs-play-inspector.notes".into(),
                label: labels.notes.into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    presence: UiPresence::default(),
                    id: "vcs-play-inspector.notes.input".into(),
                    input_kind: "text".into(),
                    value: projection.notes.clone(),
                    placeholder: None,
                    commit: Some("blur".into()),
                    on_change: vcs_action("patchProjection", Some(json!({ "field": "notes" }))),
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
            ui_inspector_readonly_field("vcs-play-inspector.tags", labels.tags, projection.tags.join(", ")),
        ],
    }])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::vcs::testkit::{app, render as render_body};

    #[test]
    fn vcs_labels_resolve_native_english_by_default() {
        let mut instance = app();
        let json = render_body(&mut instance, VCS_PLAY_BODY_INSPECTION);
        assert!(json.contains("Title"));
        assert!(json.contains("Status"));
        assert!(json.contains("Notes"));
        assert!(json.contains("Tags"));
    }
}
//#endregion 🧪️Tests
