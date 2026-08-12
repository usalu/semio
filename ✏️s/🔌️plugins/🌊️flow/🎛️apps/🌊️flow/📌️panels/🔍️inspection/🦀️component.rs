//! 🔍️ Flow play app panel — the inspector: per-widget fields for the current selection.

use crate::apps::flow::flow_action;
use crate::apps::flow::terminology::FlowPlayLabels;
use crate::artifacts::flow::schema::{widget_id, widget_kind_label};
use crate::artifacts::flow::FlowSnapshot;
use flow::Widget;
use semio_framework_plugin::{
    ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_mixed_number, ui_inspector_mixed_text, ui_inspector_readonly_field, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiFieldNode,
    UiInputNode, UiInspectorFieldGroup, UiNode, UiPresence, UiSectionNode, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, UI_INSPECTOR_MIXED_PLACEHOLDER,
};
use serde_json::json;

//#region 🔖️Constants
pub const FLOW_PLAY_BODY_INSPECTOR: &str = "flow.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(FLOW_PLAY_BODY_INSPECTOR.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(fixture: &FlowSnapshot, selected: &[String], labels: &FlowPlayLabels) -> UiNode {
    if selected.is_empty() {
        return placeholder_tree("flow-play-inspector.empty", ui_text(labels.no_selection));
    }
    let widgets: Vec<&Widget> = selected.iter().filter_map(|id| fixture.widgets.iter().find(|widget| widget_id(widget) == id)).collect();
    if widgets.is_empty() {
        return placeholder_tree("flow-play-inspector.missing", ui_text(labels.widget_not_found));
    }
    let widget_ids: Vec<String> = widgets.iter().map(|widget| widget_id(widget).to_string()).collect();
    let mut groups: Vec<UiInspectorFieldGroup> = Vec::new();
    if widgets.iter().all(|widget| matches!(widget, Widget::InputSlider { .. })) {
        let mixed = ui_inspector_mixed_number(
            &widgets
                .iter()
                .map(|widget| match widget {
                    Widget::InputSlider { value, .. } => *value,
                    _ => 0.0,
                })
                .collect::<Vec<_>>(),
        );
        groups.push(UiInspectorFieldGroup {
            presence: UiPresence::default(),
            id: "flow-play-inspector.kind.inputSlider".into(),
            label: Label::data("inputSlider"),
            default_open: None,
            fields: vec![UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "flow-play-inspector.slider-value".into(),
                label: labels.value.into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    presence: UiPresence::default(),
                    id: "flow-play-inspector.slider-value.input".into(),
                    input_kind: "number".into(),
                    value: if mixed.uniform { mixed.value.to_string() } else { String::new() },
                    placeholder: if mixed.uniform { None } else { Some(Label::data(UI_INSPECTOR_MIXED_PLACEHOLDER)) },
                    commit: None,
                    on_change: flow_action("patchFlowWidgets", Some(json!({ "widgetIds": widget_ids, "field": "value" }))),
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
            })],
        });
    }
    if widgets.iter().all(|widget| matches!(widget, Widget::InputNote { .. })) {
        let mixed = ui_inspector_mixed_text(
            &widgets
                .iter()
                .map(|widget| match widget {
                    Widget::InputNote { text, .. } => text.clone(),
                    _ => String::new(),
                })
                .collect::<Vec<_>>(),
        );
        groups.push(UiInspectorFieldGroup {
            presence: UiPresence::default(),
            id: "flow-play-inspector.kind.inputNote".into(),
            label: Label::data("inputNote"),
            default_open: None,
            fields: vec![UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "flow-play-inspector.note-text".into(),
                label: labels.text.into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    presence: UiPresence::default(),
                    id: "flow-play-inspector.note-text.input".into(),
                    input_kind: "text".into(),
                    value: mixed.value,
                    placeholder: mixed.placeholder.map(Label::data),
                    commit: Some("blur".into()),
                    on_change: flow_action("patchFlowWidgets", Some(json!({ "widgetIds": widget_ids, "field": "text" }))),
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
            })],
        });
    }
    let kind_mixed = ui_inspector_mixed_text(&widgets.iter().map(|widget| widget_kind_label(widget).to_string()).collect::<Vec<_>>());
    let mut base_fields = vec![ui_inspector_readonly_field("flow-play-inspector.kind", labels.kind, if kind_mixed.placeholder.is_none() { widget_kind_label(widgets[0]).to_string() } else { "—".into() })];
    if widget_ids.len() == 1 {
        base_fields.insert(
            0,
            UiNode::Field(UiFieldNode {
                presence: UiPresence::default(),
                id: "flow-play-inspector.id".into(),
                label: labels.id.into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    presence: UiPresence::default(),
                    id: "flow-play-inspector.id.input".into(),
                    input_kind: "text".into(),
                    value: widget_ids[0].clone(),
                    placeholder: None,
                    commit: Some("blur".into()),
                    on_change: flow_action("renameFlowWidget", Some(json!({ "oldId": widget_ids[0] }))),
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
        );
    }
    groups.push(UiInspectorFieldGroup { presence: UiPresence::default(), id: "flow-play-inspector.base".into(), label: labels.widget.into(), default_open: None, fields: base_fields });
    ui_inspector_groups_to_tree(&groups)
}

fn placeholder_tree(id: &str, child: UiNode) -> UiNode {
    ui_declarative_sections_to_tree(&[UiSectionNode { presence: UiPresence::default(), id: id.into(), label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)), default_open: Some(true), children: vec![child], menu: None }])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::flow::commands::selection::set_selection::SetSelection;
    use crate::apps::flow::testkit::{dispatch, flow_app, render as render_body};
    use crate::apps::flow::FlowCommand;

    #[test]
    fn empty_inspector_no_longer_shows_canvas_settings() {
        let mut app = flow_app();
        let json = render_body(&mut app, FLOW_PLAY_BODY_INSPECTOR);
        assert!(!json.contains("flow-play-inspector.lod-mode"));
        assert!(json.contains("flow-play-inspector.empty"));
    }

    #[test]
    fn a_single_selected_widget_exposes_a_rename_field() {
        let mut app = flow_app();
        dispatch(&mut app, FlowCommand::SetSelection(SetSelection { ids: vec!["slider".into()], edge_ids: Vec::new(), handle_ids: Vec::new() }));
        let json = render_body(&mut app, FLOW_PLAY_BODY_INSPECTOR);
        assert!(json.contains("flow-play-inspector.id.input"), "single selection exposes the id field: {json}");
        assert!(json.contains("renameFlowWidget"));
    }

    #[test]
    fn an_unknown_selection_falls_back_to_the_missing_placeholder() {
        let mut app = flow_app();
        dispatch(&mut app, FlowCommand::SetSelection(SetSelection { ids: vec!["ghost".into()], edge_ids: Vec::new(), handle_ids: Vec::new() }));
        assert!(render_body(&mut app, FLOW_PLAY_BODY_INSPECTOR).contains("flow-play-inspector.missing"));
    }
}
//#endregion 🧪️Tests
