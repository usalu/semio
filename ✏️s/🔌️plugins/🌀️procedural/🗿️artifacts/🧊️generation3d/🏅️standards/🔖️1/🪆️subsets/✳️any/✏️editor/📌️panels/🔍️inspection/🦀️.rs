//! 🔍️ Generation3d play app panel — the selection inspector.

use crate::editor::generation3d::terminology::Generation3dLabels;
use crate::editor::generation3d::GENERATION_3D_PLAY_APP_ID;
use crate::widget_id;
use semio_framework_artifact_flow_flow::{FlowHostSnapshot, Widget};
use semio_framework_artifact_flow_flow::neural::Value;
use dsl::ToValue as _;
use semio_framework_plugin::plugin_app_close_prelude::{input, Buildable, HasBase, HasChildren, InputKind, Trigger, UiAssemblyResult, UiListBuilder, UiValue};
use semio_framework_plugin::tree_item;
use semio_framework_plugin::ActionFactory;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::PanelGroup;
use semio_framework_plugin::PanelTabDefinition;
use semio_framework_plugin::PanelTabKind;
use semio_framework_plugin::PanelTreeBuilder;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_ID;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_LABEL;

//#region 🔖️Constants
pub const GENERATION_3D_PLAY_BODY_INSPECTION: &str = "procedural.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(GENERATION_3D_PLAY_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(host_snapshot: &FlowHostSnapshot, selected_node_ids: &[String], labels: &Generation3dLabels) -> UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let Some(selected_id) = selected_node_ids.first() else {
        return PanelTreeBuilder::new("procedural-play-inspector")?
            .section(
                "procedural-play-inspector.empty",
                Some(crate::ui_label(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)?),
                true,
                semio_framework_plugin::ui_node_list([
                    tree_item("procedural-play-inspector.schema", format!("{} {}", labels.schema_prefix.as_str(), host_snapshot.schema)),
                    tree_item("procedural-play-inspector.widgets", format!("{} {}", labels.widgets_prefix.as_str(), host_snapshot.widgets.len())),
                ])?,
            )?
            .build();
    };
    let Some(widget) = host_snapshot.widgets.iter().find(|entry| widget_id(entry) == selected_id) else {
        return PanelTreeBuilder::new("procedural-play-inspector")?
            .section("procedural-play-inspector.empty", Some(crate::ui_label(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)?), true, semio_framework_plugin::ui_node_list([tree_item("procedural-play-inspector.none", labels.no_selection.as_str())])?)?
            .build();
    };
    let mut fields = semio_framework_plugin::UiFixedList::default();
    fields
        .try_push(tree_item("procedural-play-inspector.id", format!("{}: {}", labels.id_field.as_str(), widget_id(widget)))?)
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
    if let Widget::InputSlider { value, min, max, .. } = widget {
        let widget_ids = ui_value_list([crate::ui_value_text(selected_id)?])?;
        let action_args = crate::ui_value_map([("field", crate::ui_value_text("value")?), ("widgetIds", widget_ids)])?;
        let (action, args) = ActionFactory::new(GENERATION_3D_PLAY_APP_ID).action("patchFlowWidgets", Some(action_args))?;
        let control = input(InputKind::Number)
            .value(crate::ui_text(value.to_string())?)
            .try_id("procedural-play-inspector.value.input")
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.control-id", "fixed UI inspector admission failed"))?;
        let control = match args {
            Some(args) => control.try_on_with(Trigger::Change, action, args),
            None => control.try_on(Trigger::Change, action),
        };
        let control = control
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.control-binding", "fixed UI inspector admission failed"))?
            .try_build()
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.control", "fixed UI inspector admission failed"))?;
        // 🪪️ The editable control rides as the CHILD OF A TREE ROW, not as a bare `field` beside the
        // rows. A panel section keeps only `treeItem` children (`collectTreeItems`,
        // `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx`);
        // a row's NON-`treeItem` children are what the renderer mounts as that row's inline controls
        // (`collectTreeItemControls`, same file). Authored as a sibling `field`, the one editable
        // control this panel owns was dropped on the way to the DOM: the panel painted `Id: height`
        // and `Range: 0..10` with no number input at all, so a slider parameter could not be edited
        // from the inspector (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `🐍️react-gap-probe.mjs`
        // step `inspection-edit`).
        let row = semio_framework_ui_contract::tree_item(crate::ui_label(labels.value_field.as_str())?)
            .try_id("procedural-play-inspector.value")
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.field-id", "fixed UI inspector admission failed"))?
            .try_children([control])
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.field-child", "fixed UI inspector admission failed"))?
            .try_build()
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.field", "fixed UI inspector admission failed"))?;
        fields.try_push(row).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
        fields
            .try_push(tree_item("procedural-play-inspector.range", format!("{}: {min}..{max}", labels.range_field.as_str()))?)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
    }
    if let Widget::InputNote { text, .. } = widget {
        fields.try_push(if text.len() <= semio_framework_ui_contract::UI_TEXT_MAX_BYTES { editable_input("procedural-play-inspector.note", labels.value_field.as_str(), selected_id, "text", None, text, Some(InputKind::LongText))? } else { tree_item("procedural-play-inspector.note", labels.input_text_too_long.as_str())? }).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
    }
    if let Widget::Neuron { neuron_kind, params, .. } = widget {
        fields
            .try_push(tree_item("procedural-play-inspector.neuron-kind", crate::editor::generation3d::terminology::generation3d_catalogue_name(labels, neuron_kind, neuron_kind))?)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
        let infos = semio_framework_os_flow::flow_neuron_kind_info_map();
        if let Some(info) = infos.get(neuron_kind) {
            for port in &info.inputs {
                let key = format!("procedural-play-inspector.input.{}", port.name);
                let label = crate::editor::generation3d::terminology::generation3d_input_name(labels, &port.name, port.label.as_deref().unwrap_or(&port.name));
                if let Some(connection) = host_snapshot.synapses.iter().find(|synapse| synapse.to == *selected_id && synapse.to_port == port.name) {
                    fields.try_push(tree_item(&key, format!("{label}: {} ({}.{})", labels.input_connected.as_str(), connection.from, connection.from_port))?).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
                    continue;
                }
                let current = params.get(&port.name).or(port.default.as_ref());
                let schema = current.and_then(Value::as_dictionary).and_then(|dictionary| dictionary.schema()).or_else(|| port.value_types.first().map(String::as_str)).unwrap_or("");
                let value = current.map(|value| value.to_value());
                if !port.cardinality.is_collection() && matches!(schema, "point" | "vector") {
                    for axis in ["x", "y", "z"] {
                        let number = value.as_ref().and_then(|value| value.get(axis)).and_then(dsl::DslValue::as_f64).unwrap_or(0.0);
                        fields.try_push(editable_input(&format!("{key}.{axis}"), &format!("{label} {axis}"), selected_id, &port.name, Some(axis), &number.to_string(), Some(InputKind::Number))?).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
                    }
                } else if !port.cardinality.is_collection() && matches!(schema, "number" | "text" | "boolean") {
                    let atom = value.as_ref().and_then(|value| value.get("value"));
                    let text = atom.map(|value| value.as_str().map(str::to_string).unwrap_or_else(|| dsl::json::to_json_string(value))).unwrap_or_default();
                    if text.len() <= semio_framework_ui_contract::UI_TEXT_MAX_BYTES {
                        fields.try_push(editable_input(&key, label, selected_id, &port.name, None, &text, if schema == "boolean" { None } else { Some(if schema == "number" { InputKind::Number } else { InputKind::LongText }) })?).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
                    } else {
                        fields.try_push(tree_item(&key, format!("{label}: {}", labels.input_text_too_long.as_str()))?).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
                    }
                } else {
                    fields.try_push(tree_item(&key, format!("{label}: {}", labels.input_connect_hint.as_str()))?).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
                }
            }
        }
    }
    if let Widget::Variable { name, schema, .. } = widget {
        fields
            .try_push(tree_item("procedural-play-inspector.variable-name", format!("{}: {name}", labels.value_field.as_str()))?)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
        fields
            .try_push(tree_item("procedural-play-inspector.variable-schema", format!("{}: {schema}", labels.range_field.as_str()))?)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
    }
    if let Widget::OutputAction { action, .. } = widget {
        fields
            .try_push(tree_item("procedural-play-inspector.action", format!("{}: {action}", labels.value_field.as_str()))?)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
    }
    if let Widget::OutputExport { format, .. } = widget {
        fields
            .try_push(tree_item("procedural-play-inspector.export-format", format!("{}: {format}", labels.value_field.as_str()))?)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
    }
    PanelTreeBuilder::new("procedural-play-inspector")?.section("procedural-play-inspector.widget", Some(crate::ui_label(labels.widget_group.as_str())?), true, fields)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests

/// 🎛️ One labeled input commits typed edits through the retained document command.
fn editable_input(key: &str, label: &str, widget: &str, channel: &str, component: Option<&str>, value: &str, kind: Option<InputKind>) -> UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut fields = vec![("widgetId", crate::ui_value_text(widget)?), ("channel", crate::ui_value_text(channel)?)];
    if let Some(component) = component { fields.push(("component", crate::ui_value_text(component)?)); }
    let (action, args) = ActionFactory::new(GENERATION_3D_PLAY_APP_ID).action("setWidgetInput", Some(crate::ui_value_map(fields)?))?;
    let control = if kind.is_none() {
        let control = semio_framework_ui_contract::toggle(value == "true").appearance(semio_framework_ui_contract::ToggleAppearance::Checkbox).try_id(format!("{key}.input")).and_then(|control| control.try_label(label)).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.control", "fixed UI input admission failed"))?;
        match args { Some(args) => control.try_on_with(Trigger::Change, action, args), None => control.try_on(Trigger::Change, action) }.map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.binding", "fixed UI input admission failed"))?.try_build().map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.control", "fixed UI input admission failed"))?
    } else {
    let control = input(kind.unwrap()).value(crate::ui_text(value.to_string())?).commit(crate::ui_text("blur")?).try_id(format!("{key}.input")).and_then(|control| control.try_label(label)).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.control", "fixed UI input admission failed"))?;
    let control = match args { Some(args) => control.try_on_with(Trigger::Commit, action, args), None => control.try_on(Trigger::Commit, action) }.map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.binding", "fixed UI input admission failed"))?.try_build().map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.control", "fixed UI input admission failed"))?;
    control
    };
    semio_framework_ui_contract::tree_item(crate::ui_label(label)?).try_id(key).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.field-id", "fixed UI field admission failed"))?.try_children([control]).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.field", "fixed UI field admission failed"))?.try_build().map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.field", "fixed UI field admission failed"))
}

//#endregion 🧪️Tests

fn ui_value_list(values: impl IntoIterator<Item = UiValue>) -> UiAssemblyResult<UiValue> {
    let mut builder = UiListBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.value.list", "fixed UI admission failed"))?;
    for value in values {
        builder.push(value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.value.list.item", "fixed UI admission failed"))?;
    }
    Ok(UiValue::List(builder.finish()))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
