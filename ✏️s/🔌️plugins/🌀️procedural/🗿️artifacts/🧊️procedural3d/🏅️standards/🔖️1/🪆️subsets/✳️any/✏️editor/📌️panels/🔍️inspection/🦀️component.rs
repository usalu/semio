//! 🔍️ Procedural3d play app panel — the selection inspector.

use crate::artifacts::procedural3d::widget_id;
use crate::editor::procedural3d::terminology::Procedural3dLabels;
use crate::editor::procedural3d::PROCEDURAL_3D_PLAY_APP_ID;
use flow::{FlowFixture, Widget};
use semio_framework_plugin::plugin_app_close_prelude::{field, input, Buildable, HasBase, HasChildren, InputKind, Trigger};
use semio_framework_plugin::{tree_item, ActionFactory, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

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
pub fn render(fixture: &FlowFixture, selected_node_ids: &[String], labels: &Procedural3dLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let Some(selected_id) = selected_node_ids.first() else {
        return PanelTreeBuilder::new("procedural-play-inspector")?
            .section(
                "procedural-play-inspector.empty",
                Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
                true,
                vec![
                    tree_item("procedural-play-inspector.schema", format!("{} {}", labels.schema_prefix.as_str(), fixture.schema))?,
                    tree_item("procedural-play-inspector.widgets", format!("{} {}", labels.widgets_prefix.as_str(), fixture.widgets.len()))?,
                ],
            )?
            .build();
    };
    let Some(widget) = fixture.widgets.iter().find(|entry| widget_id(entry) == selected_id) else {
        return PanelTreeBuilder::new("procedural-play-inspector")?
            .section("procedural-play-inspector.empty", Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()), true, vec![tree_item("procedural-play-inspector.none", labels.no_selection.as_str())?])?
            .build();
    };
    let mut fields = vec![tree_item("procedural-play-inspector.id", format!("{}: {}", labels.id_field.as_str(), widget_id(widget)))?];
    if let Widget::InputSlider { value, min, max, .. } = widget {
        let (action, args) = ActionFactory::new(PROCEDURAL_3D_PLAY_APP_ID).action("patchFlowWidgets", Some(serde_json::json!({ "widgetIds": [selected_id], "field": "value" })))?;
        let control = input(InputKind::Number).value(value.to_string()).id("procedural-play-inspector.value.input");
        let control = match args {
            Some(args) => control.on_with(Trigger::Change, action, args),
            None => control.on(Trigger::Change, action),
        };
        fields.push(field(labels.value_field.as_str()).id("procedural-play-inspector.value").child(control).build());
        fields.push(tree_item("procedural-play-inspector.range", format!("{}: {min}..{max}", labels.range_field.as_str()))?);
    }
    if let Widget::InputNote { text, .. } = widget {
        fields.push(tree_item("procedural-play-inspector.note", format!("{}: {text}", labels.value_field.as_str()))?);
    }
    if let Widget::Neuron { neuron_kind, .. } = widget {
        fields.push(tree_item("procedural-play-inspector.neuron-kind", format!("{}: {neuron_kind}", labels.id_field.as_str()))?);
    }
    if let Widget::Variable { name, schema, .. } = widget {
        fields.push(tree_item("procedural-play-inspector.variable-name", format!("{}: {name}", labels.value_field.as_str()))?);
        fields.push(tree_item("procedural-play-inspector.variable-schema", format!("{}: {schema}", labels.range_field.as_str()))?);
    }
    if let Widget::OutputAction { action, .. } = widget {
        fields.push(tree_item("procedural-play-inspector.action", format!("{}: {action}", labels.value_field.as_str()))?);
    }
    if let Widget::OutputExport { format, .. } = widget {
        fields.push(tree_item("procedural-play-inspector.export-format", format!("{}: {format}", labels.value_field.as_str()))?);
    }
    PanelTreeBuilder::new("procedural-play-inspector")?.section("procedural-play-inspector.widget", Some(labels.widget_group.as_str().into()), true, fields)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedural3d::testkit::{app, render as render_body};

    #[test]
    fn inspector_shows_no_selection_by_default() {
        let _serial = crate::editor::procedural3d::test_support::lock();
        let mut app = app();
        assert!(render_body(&mut app, PROCEDURAL_3D_PLAY_BODY_INSPECTION).contains("Schema:"));
    }
}
//#endregion 🧪️Tests
