//! 🔍️ Lowpoly play app panel — the active object's inspector (name, smooth shading, selection summary,
//! transform utility, staged utility-param sliders).

use crate::editor::lowpoly::lowpoly_action;
use crate::editor::lowpoly::terminology::LowpolyLabels;
use crate::editor::lowpoly::view::{active_object, utility_params_value, LowpolyView};
use crate::artifacts::lowpoly::LOWPOLY_DOCUMENT_SCHEMA;
use semio_framework_plugin::{
    ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_stack_vertical, ui_text, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiFieldNode,
    UiInspectorFieldGroup, UiNode, UiPresence, UiToggleNode,
};
use serde_json::Value;

//#region 🔖️Constants
pub const LOWPOLY_PLAY_BODY_INSPECTION: &str = "lowpoly.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(LOWPOLY_PLAY_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn inspector_utility_param_field(id: &str, label: semio_framework_plugin::LabelText, key: &str, value: &Value) -> UiNode {
    UiNode::Field(UiFieldNode {
        presence: UiPresence::default(),
        id: format!("lowpoly-play-inspector.{id}"),
        label: label.into(),
        child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
            presence: UiPresence::default(),
            id: format!("lowpoly-play-inspector.{id}.input"),
            input_kind: "number".into(),
            value: value.get(key).map_or_else(|| "0".into(), |entry| entry.to_string()),
            placeholder: None,
            commit: None,
            on_change: lowpoly_action("setUtilityParam", Some(serde_json::json!({ "key": key }))),
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
    })
}

pub fn render(view: LowpolyView<'_>, active_utility: &str, labels: &LowpolyLabels) -> UiNode {
    let Some(object) = active_object(view) else {
        return ui_stack_vertical(vec![ui_text(Label::data(format!("Schema: {LOWPOLY_DOCUMENT_SCHEMA}"))), ui_text(Label::data("No active object"))]);
    };
    let config = view.config;
    let params = utility_params_value(config);
    ui_inspector_groups_to_tree(&[
        UiInspectorFieldGroup {
            id: "lowpoly-play-inspector.object".into(),
            label: labels.object.into(),
            default_open: None,
            presence: UiPresence::default(),
            fields: vec![
                UiNode::Field(UiFieldNode {
                    presence: UiPresence::default(),
                    id: "lowpoly-play-inspector.object.name".into(),
                    label: labels.name.into(),
                    child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
                        presence: UiPresence::default(),
                        id: "lowpoly-play-inspector.object.name.input".into(),
                        input_kind: "text".into(),
                        value: object.name.clone(),
                        placeholder: None,
                        commit: None,
                        on_change: lowpoly_action("patchObject", Some(serde_json::json!({ "objectId": object.id, "field": "name" }))),
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
                    id: "lowpoly-play-inspector.object.smooth".into(),
                    label: labels.smooth_shading.into(),
                    child: Box::new(UiNode::Toggle(UiToggleNode {
                        id: "lowpoly-play-inspector.object.smooth.toggle".into(),
                        icon_id: "sun".into(),
                        presence: UiPresence::selected(object.smooth_shading),
                        text: None,
                        on_change: lowpoly_action("patchObject", Some(serde_json::json!({ "objectId": object.id, "field": "smoothShading" }))),
                        menu: None,
                    })),
                    description: None,
                    required: None,
                    error: None,
                    menu: None,
                }),
                // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the selection summary/mode
                // rows used to read `LowpolyConfig`; the mesh domain's selection is framework-owned
                // `InteractionState` now, and `ArtifactApp::render` is not threaded an `InteractionView`
                // this wave — dropped rather than shown stale. Peer/self selection surfaces generically.
            ],
        },
        UiInspectorFieldGroup {
            presence: UiPresence::default(),
            id: "lowpoly-play-inspector.transform".into(),
            label: labels.transform.into(),
            default_open: None,
            fields: vec![ui_inspector_readonly_field("lowpoly-play-inspector.transform.utility", labels.utility, active_utility)],
        },
        UiInspectorFieldGroup {
            presence: UiPresence::default(),
            id: "lowpoly-play-inspector.utility-params".into(),
            label: labels.utility_params.into(),
            default_open: Some(true),
            fields: vec![
                inspector_utility_param_field("extrude", labels.extrude_distance, "extrudeDistance", &params),
                inspector_utility_param_field("inset", labels.inset_amount, "insetAmount", &params),
                inspector_utility_param_field("bevel", labels.bevel_amount, "bevelAmount", &params),
                inspector_utility_param_field("bevel-segments", labels.bevel_segments, "bevelSegments", &params),
                inspector_utility_param_field("loop-cuts", labels.loop_cuts, "loopCuts", &params),
                inspector_utility_param_field("decimate", labels.decimate_ratio, "decimateRatio", &params),
                inspector_utility_param_field("snap", labels.snap_grid, "snapGrid", &params),
                inspector_utility_param_field("mirror", labels.mirror_axis, "mirrorAxis", &params),
                inspector_utility_param_field("brush-size", labels.brush_size, "brushSize", &params),
                inspector_utility_param_field("brush-opacity", labels.brush_opacity, "brushOpacity", &params),
                inspector_utility_param_field("brush-hardness", labels.brush_hardness, "brushHardness", &params),
            ],
        },
    ])
}
//#endregion 🔖️Render
