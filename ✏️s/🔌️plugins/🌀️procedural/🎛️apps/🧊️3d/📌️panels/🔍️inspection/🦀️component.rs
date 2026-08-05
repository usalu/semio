//! 🔍️ Procedural3d play app panel — the selection inspector.

use crate::apps::procedural3d::procedural3d_action;
use crate::apps::procedural3d::terminology::Procedural3dLabels;
use crate::artifacts::procedural3d::widget_id;
use flow_core::{FlowFixture, Widget};
use semio_framework_plugin::{
    ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_mixed_number, ui_inspector_readonly_field, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiFieldNode, UiInspectorFieldGroup, UiNode, UiPresence,
    UiSectionNode, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};

//#region 🔖️Constants
pub const PROCEDURAL_3D_PLAY_BODY_INSPECTION: &str = "procedural.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(PROCEDURAL_3D_PLAY_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(fixture: &FlowFixture, selected_node_ids: &[String], labels: &Procedural3dLabels) -> UiNode {
    let Some(selected_id) = selected_node_ids.first() else {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "procedural-play-inspector.empty".into(),
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
            default_open: Some(true),
            children: vec![ui_text(Label::data(format!("{} {}", labels.schema_prefix.as_str(), fixture.schema))), ui_text(Label::data(format!("{} {}", labels.widgets_prefix.as_str(), fixture.widgets.len())))],
            presence: UiPresence::default(),
            menu: None,
        }]);
    };
    let Some(widget) = fixture.widgets.iter().find(|entry| widget_id(entry) == selected_id) else {
        return ui_declarative_sections_to_tree(&[UiSectionNode {
            id: "procedural-play-inspector.empty".into(),
            label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
            default_open: Some(true),
            children: vec![ui_text(labels.no_selection)],
            presence: UiPresence::default(),
            menu: None,
        }]);
    };
    let mut fields = vec![ui_inspector_readonly_field("procedural-play-inspector.id", labels.id_field, widget_id(widget))];
    if let Widget::InputSlider { value, min, max, .. } = widget {
        let mixed = ui_inspector_mixed_number(&[*value]);
        fields.push(UiNode::Field(UiFieldNode {
            presence: UiPresence::default(),
            id: "procedural-play-inspector.value".into(),
            label: labels.value_field.into(),
            child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
                presence: UiPresence::default(),
                id: "procedural-play-inspector.value.input".into(),
                input_kind: "number".into(),
                value: mixed.value.to_string(),
                placeholder: None,
                commit: None,
                on_change: procedural3d_action("patchFlowWidgets", Some(serde_json::json!({ "widgetIds": [selected_id], "field": "value" }))),
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
        }));
        fields.push(ui_inspector_readonly_field("procedural-play-inspector.range", labels.range_field, &format!("{min}..{max}")));
    }
    if let Widget::InputNote { text, .. } = widget {
        fields.push(ui_inspector_readonly_field("procedural-play-inspector.note", labels.value_field, text));
    }
    if let Widget::Neuron { neuron_kind, .. } = widget {
        fields.push(ui_inspector_readonly_field("procedural-play-inspector.neuron-kind", labels.id_field, neuron_kind));
    }
    if let Widget::Variable { name, schema, .. } = widget {
        fields.push(ui_inspector_readonly_field("procedural-play-inspector.variable-name", labels.value_field, name));
        fields.push(ui_inspector_readonly_field("procedural-play-inspector.variable-schema", labels.range_field, schema));
    }
    if let Widget::OutputAction { action, .. } = widget {
        fields.push(ui_inspector_readonly_field("procedural-play-inspector.action", labels.value_field, action));
    }
    if let Widget::OutputExport { format, .. } = widget {
        fields.push(ui_inspector_readonly_field("procedural-play-inspector.export-format", labels.value_field, format));
    }
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { presence: UiPresence::default(), id: "procedural-play-inspector.widget".into(), label: labels.widget_group.into(), default_open: None, fields }])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural3d::testkit::{app, render as render_body};

    #[test]
    fn inspector_shows_no_selection_by_default() {
        let mut app = app();
        assert!(render_body(&mut app, PROCEDURAL_3D_PLAY_BODY_INSPECTION).contains("Schema:"));
    }
}
//#endregion 🧪️Tests
