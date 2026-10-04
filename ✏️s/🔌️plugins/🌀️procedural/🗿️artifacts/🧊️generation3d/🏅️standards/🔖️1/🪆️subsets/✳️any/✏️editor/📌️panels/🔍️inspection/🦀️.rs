//! 🔍️ Generation3d play app panel — the selection inspector.

use crate::editor::generation3d::terminology::Generation3dLabels;
use crate::editor::generation3d::GENERATION_3D_PLAY_APP_ID;
use crate::widget_id;
use semio_framework_artifact_flow_flow::{FlowHostSnapshot, Widget};
use semio_framework_artifact_flow_flow::neural::Value;
use semio_framework_value::ToValue as _;
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
pub fn render(host_snapshot: &FlowHostSnapshot, selected_node_ids: &[String], labels: &Generation3dLabels, windows: &semio_framework_plugin::TreeWindows<'_>, locale: semio_framework_ui_locale::Locale, terminology: semio_framework_ui_locale::Terminology, meshes: &[semio_framework_plugin::MeshData], eval_json: &str, status_json: &str, export_ready: bool) -> UiAssemblyResult<semio_framework_plugin::BuiltNode> {
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
            .value(crate::ui_text(value.to_string())?).commit(crate::ui_text("blur")?).try_label(labels.value_field.as_str()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.control-label", "fixed UI input admission failed"))?
            .try_id("procedural-play-inspector.value.input")
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.control-id", "fixed UI inspector admission failed"))?;
        let control = match args {
            Some(args) => control.try_on_with(Trigger::Commit, action, args),
            None => control.try_on(Trigger::Commit, action),
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
        let row = if crate::editor::generation3d::commands::set_widget_input::mesh_source_text(host_snapshot, selected_id, "text").is_ok() { mesh_source_input(windows, selected_id, "text", text, labels)? }
        else if text.len() <= semio_framework_ui_contract::UI_TEXT_MAX_BYTES { editable_input("procedural-play-inspector.note", labels.value_field.as_str(), selected_id, "text", None, text, Some(InputKind::LongText))? }
        else { tree_item("procedural-play-inspector.note", labels.input_text_too_long.as_str())? };
        fields.try_push(row).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
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
                if neuron_kind == "brep.mesh.construct" && port.name == "data" {
                    let row = match crate::editor::generation3d::commands::set_widget_input::mesh_source_text(host_snapshot, selected_id, "data") {
                        Ok((_, _, text)) => mesh_source_input(windows, selected_id, "data", &text, labels)?,
                        Err(_) => tree_item(&key, labels.input_connect_hint.as_str())?,
                    };
                    fields.try_push(row).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
                    continue;
                }
                if let Some(connection) = host_snapshot.synapses.iter().find(|synapse| synapse.to == *selected_id && synapse.to_port == port.name) {
                    fields.try_push(tree_item(&key, format!("{label}: {} ({}.{})", labels.input_connected.as_str(), connection.from, connection.from_port))?).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
                    continue;
                }
                let current = params.get(&port.name).or(port.default.as_ref());
                let schema = current.and_then(Value::as_dictionary).and_then(|dictionary| dictionary.schema()).or_else(|| port.value_types.first().map(String::as_str)).unwrap_or("");
                let value = current.map(|value| value.to_value());
                if neuron_kind == "brep.brep" && matches!(port.name.as_str(), "sourceHandle" | "edgeLabels" | "faceLabels") {
                    let text = value.as_ref().and_then(|value| value.get("value")).and_then(semio_framework_value::DslValue::as_str);
                    let row = if port.name == "sourceHandle" {
                        tree_item(&key, format!("{label}: {}", text.filter(|text| !text.is_empty() && text.len() <= semio_framework_ui_contract::UI_TEXT_MAX_BYTES / 2).unwrap_or(labels.input_value_unavailable.as_str())))?
                    } else if let Some(value) = text.and_then(|text| semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()).filter(|value| value.as_array().is_some()) {
                        reflected_output(windows, &key, label, &value, 0, labels)?
                    } else { tree_item(&key, format!("{label}: {}", labels.input_value_unavailable.as_str()))? };
                    fields.try_push(row).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
                    continue;
                }
                if port.cardinality.is_collection() {
                    let schema = value.as_ref().and_then(|value| value.get("0")).and_then(|value| value.get("$schema")).and_then(semio_framework_value::DslValue::as_str).filter(|schema| port.item_types.iter().any(|declared| declared == schema)).or_else(|| port.item_types.first().map(String::as_str)).unwrap_or("");
                    let items = match value.as_ref() {
                        Some(value) => crate::standards::v1::subsets::any::schema::mutations::change_widget_input::WidgetInputValue::of_literal_as(value, schema).and_then(|value| value.items()),
                        None => crate::standards::v1::subsets::any::schema::mutations::change_widget_input::WidgetInputValue::collection(schema, Vec::new()).and_then(|value| value.items()),
                    };
                    let row = if let Some(items) = items { collection_input(windows, &key, label, selected_id, &port.name, &items, &port.cardinality, labels)? } else { tree_item(&key, format!("{label}: {}", labels.input_connect_hint.as_str()))? };
                    fields.try_push(row).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
                } else if matches!(schema, "point" | "vector") {
                    for axis in ["x", "y", "z"] {
                        let number = value.as_ref().and_then(|value| value.get(axis)).and_then(semio_framework_value::DslValue::as_f64).unwrap_or(0.0);
                        fields.try_push(editable_input(&format!("{key}.{axis}"), &format!("{label} {axis}"), selected_id, &port.name, Some(axis), &number.to_string(), Some(InputKind::Number))?).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
                    }
                } else if !port.cardinality.is_collection() && matches!(schema, "number" | "text" | "boolean") {
                    let atom = value.as_ref().map(|value| value.get("value").unwrap_or(value));
                    let text = atom.map(|value| value.as_str().map(str::to_string).unwrap_or_else(|| semio_framework_pack_json::to_json_string(value))).unwrap_or_default();
                    if text.len() <= semio_framework_ui_contract::UI_TEXT_MAX_BYTES {
                        fields.try_push(editable_input(&key, label, selected_id, &port.name, None, &text, if schema == "boolean" { None } else { Some(if schema == "number" { InputKind::Number } else { InputKind::LongText }) })?).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
                    } else {
                        fields.try_push(tree_item(&key, format!("{label}: {}", labels.input_text_too_long.as_str()))?).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
                    }
                } else {
                    fields.try_push(tree_item(&key, format!("{label}: {}", labels.input_connect_hint.as_str()))?).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
                }
            }
            let evaluation = semio_framework_pack_json::parse(eval_json, semio_framework_pack_json::JsonMemberPolicy::Reject).ok();
            let statuses = semio_framework_pack_json::parse(status_json, semio_framework_pack_json::JsonMemberPolicy::Reject).ok();
            let entry = evaluation.as_ref().and_then(|value| value.get(selected_id));
            let status = statuses.as_ref().and_then(|value| value.get(selected_id)).and_then(|value| value.get("status")).and_then(semio_framework_pack_json::Value::as_str);
            let error = entry.and_then(|entry| entry.get("error")).and_then(semio_framework_pack_json::Value::as_str);
            for port in &info.outputs {
                let key = format!("procedural-play-inspector.output.{}", port.name);
                let label = crate::editor::generation3d::terminology::generation3d_input_name(labels, &port.name, port.label.as_deref().unwrap_or(&port.name));
                let value = entry.and_then(|entry| entry.get("out")).and_then(|outputs| outputs.get(&port.name));
                let row = if error.is_some() || status == Some("error") { tree_item(&key, format!("{label}: {}", labels.mesh_output_error.as_str()))? }
                else if matches!(status, Some("stale" | "queued" | "computing" | "blocked")) { tree_item(&key, format!("{label}: {}", labels.mesh_output_stale.as_str()))? }
                else if let Some(value) = value { reflected_output(windows, &key, label, value, 0, labels)? }
                else { tree_item(&key, format!("{label}: {}", labels.input_value_unavailable.as_str()))? };
                fields.try_push(row).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.outputs", "fixed UI output admission failed"))?;
            }
        }
    }
    if let Widget::Variable { name, schema, .. } = widget {
        fields
            .try_push(editable_item_with_facet("procedural-play-inspector.variable-name", labels.input_variable_name.as_str(), selected_id, "name", None, None, Some("variableName"), name, Some(InputKind::Text))?)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
        fields
            .try_push(editable_item_with_facet("procedural-play-inspector.variable-schema", labels.input_variable_type.as_str(), selected_id, "schema", None, None, Some("variableSchema"), schema, Some(InputKind::Text))?)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
        let evaluation = semio_framework_pack_json::parse(eval_json, semio_framework_pack_json::JsonMemberPolicy::Reject).ok();
        let value = evaluation.as_ref().and_then(|evaluation| evaluation.get(selected_id)).and_then(|entry| entry.get("out")).and_then(semio_framework_pack_json::Value::as_object).and_then(|outputs| outputs.iter().next().map(|(_, value)| value)).and_then(reflected_value).unwrap_or_else(|| labels.input_value_unavailable.as_str().to_string());
        fields.try_push(tree_item("procedural-play-inspector.variable-value", format!("{}: {value}", labels.value_field.as_str()))?).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
    }
    if let Widget::OutputAction { action, .. } = widget {
        fields
            .try_push(tree_item("procedural-play-inspector.action", format!("{}: {action}", labels.value_field.as_str()))?)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
    }
    if let Widget::OutputExport { format, .. } = widget {
        fields
            .try_push(export_format_input(selected_id, format, labels, locale, terminology)?)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
        fields.try_push(export_connected_button(selected_id, format, labels, export_ready)?).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
        if !export_ready { fields.try_push(tree_item("procedural-play-inspector.export-pending", labels.input_export_pending.as_str())?).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?; }
        for diagnostic in crate::standards::v1::subsets::any::io::document_io::format_diagnostics(meshes, format) {
            fields.try_push(tree_item(&format!("procedural-play-inspector.export-loss.{}", diagnostic.code), diagnostic.label.resolve(terminology, locale).to_string())?).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.fields", "fixed UI inspector admission failed"))?;
        }
    }
    PanelTreeBuilder::new("procedural-play-inspector")?.section("procedural-play-inspector.widget", Some(crate::ui_label(labels.widget_group.as_str())?), true, fields)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests

/// 🎛️ One labeled input commits typed edits through the retained document command.
fn editable_input(key: &str, label: &str, widget: &str, channel: &str, component: Option<&str>, value: &str, kind: Option<InputKind>) -> UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    editable_item(key, label, widget, channel, component, None, value, kind)
}

/// 🎚️ A scalar or ordered item commits through the same absolute input mutation.
fn editable_item(key: &str, label: &str, widget: &str, channel: &str, component: Option<&str>, index: Option<usize>, value: &str, kind: Option<InputKind>) -> UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    editable_item_with_facet(key, label, widget, channel, component, index, None, value, kind)
}

/// 🧬️ Explicit widget facets keep metadata separate from operator port semantics.
fn editable_item_with_facet(key: &str, label: &str, widget: &str, channel: &str, component: Option<&str>, index: Option<usize>, facet: Option<&str>, value: &str, kind: Option<InputKind>) -> UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut fields = vec![("widgetId", crate::ui_value_text(widget)?), ("channel", crate::ui_value_text(channel)?)];
    if let Some(component) = component { fields.push(("component", crate::ui_value_text(component)?)); }
    if let Some(index) = index { fields.push(("index", UiValue::Number(index as f64))); fields.push(("operation", crate::ui_value_text("set")?)); }
    if let Some(facet) = facet { fields.push(("facet", crate::ui_value_text(facet)?)); }
    editable_field(key, label, value, kind, fields)
}

/// 🎛️ One retained action binding drives the existing labeled input control.
fn editable_field(key: &str, label: &str, value: &str, kind: Option<InputKind>, fields: Vec<(&'static str, UiValue)>) -> UiAssemblyResult<semio_framework_plugin::BuiltNode> {
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

/// 📤️ Export format options come from the artifact's owning document codecs.
fn export_format_input(widget: &str, format: &str, labels: &Generation3dLabels, locale: semio_framework_ui_locale::Locale, terminology: semio_framework_ui_locale::Terminology) -> UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let key = "procedural-play-inspector.export-format";
    let args = crate::ui_value_map([("widgetId", crate::ui_value_text(widget)?), ("channel", crate::ui_value_text("format")?), ("facet", crate::ui_value_text("exportFormat")?)])?;
    let (action, args) = ActionFactory::new(GENERATION_3D_PLAY_APP_ID).action("setWidgetInput", Some(args))?;
    let mut select = semio_framework_ui_contract::select(crate::ui_text(format)?).try_id(format!("{key}.input")).and_then(|select| select.try_label(labels.input_export_format.as_str())).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.export-select", "fixed UI select admission failed"))?;
    for format in crate::standards::v1::subsets::any::io::document_io::EXPORT_FORMATS {
        let label = LocalizedLabel::native(format.label_en, format.label_de).resolve(terminology, locale).to_string();
        select = select.try_item(crate::ui_text(format.id)?, crate::ui_label(&label)?).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.export-option", "fixed UI select admission failed"))?;
    }
    let select = match args { Some(args) => select.try_on_with(Trigger::Change, action, args), None => select.try_on(Trigger::Change, action) }.map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.export-binding", "fixed UI select admission failed"))?.try_build().map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.export-select", "fixed UI select admission failed"))?;
    semio_framework_ui_contract::tree_item(crate::ui_label(labels.input_export_format.as_str())?).try_id(key).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.export-row", "fixed UI row admission failed"))?.try_children([select]).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.export-row", "fixed UI row admission failed"))?.try_build().map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.export-row", "fixed UI row admission failed"))
}

/// 📤️ A connected export uses the existing artifact-owned command and codec roster.
fn export_connected_button(widget: &str, format: &str, labels: &Generation3dLabels, ready: bool) -> UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let args = crate::ui_value_map([("widgetId", crate::ui_value_text(widget)?), ("format", crate::ui_value_text(format)?)])?;
    let (action, args) = ActionFactory::new(GENERATION_3D_PLAY_APP_ID).action("exportDocument", Some(args))?;
    let button = semio_framework_ui_contract::button(crate::ui_label(labels.input_export_connected.as_str())?).disabled(!ready).try_id("procedural-play-inspector.export-connected.button").map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.export-button", "fixed UI button admission failed"))?;
    let button = match args { Some(args) => button.try_on_with(Trigger::Activate, action, args), None => button.try_on(Trigger::Activate, action) }.map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.export-binding", "fixed UI button admission failed"))?.try_build().map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.export-button", "fixed UI button admission failed"))?;
    semio_framework_ui_contract::tree_item(crate::ui_label(labels.input_export_connected.as_str())?).try_id("procedural-play-inspector.export-connected").map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.export-row", "fixed UI row admission failed"))?.try_children([button]).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.export-row", "fixed UI row admission failed"))?.try_build().map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.export-row", "fixed UI row admission failed"))
}

/// 🪞️ Reflected values come from the retained evaluation, bounded to readable scalar or coordinate text.
fn reflected_value(value: &semio_framework_pack_json::Value) -> Option<String> {
    let atom = value.get("value").unwrap_or(value);
    let text = if let Some(text) = atom.as_str() { text.to_string() }
    else if let Some(number) = atom.as_f64() { number.to_string() }
    else if let Some(boolean) = atom.as_bool() { boolean.to_string() }
    else if matches!(value.get("$schema").and_then(semio_framework_pack_json::Value::as_str), Some("point" | "vector")) { format!("({}, {}, {})", value.get("x")?.as_f64()?, value.get("y")?.as_f64()?, value.get("z")?.as_f64()?) }
    else { return None; };
    (text.len() <= semio_framework_ui_contract::UI_TEXT_MAX_BYTES / 2).then_some(text)
}

/// 🪞️ Declared evaluated outputs use read-only retained windows without geometry recomputation.
fn reflected_output(windows: &semio_framework_plugin::TreeWindows<'_>, key: &str, label: &str, value: &semio_framework_pack_json::Value, depth: usize, labels: &Generation3dLabels) -> UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    use semio_framework_pack_json::Value;
    if let Some(text) = reflected_value(value) { return tree_item(key, format!("{label}: {text}")); }
    if depth >= 16 || matches!(value.get("$schema").and_then(Value::as_str), Some("geometry" | "mesh")) { return tree_item(key, format!("{label}: {}", value.get("$schema").and_then(Value::as_str).unwrap_or(labels.input_value_unavailable.as_str()))); }
    let array = value.as_array(); let object: Vec<_> = value.as_object().map(|fields| fields.iter().filter(|(name, _)| *name != "$schema").collect()).unwrap_or_default();
    if array.is_none() && value.as_object().is_none() { return tree_item(key, format!("{label}: {}", labels.input_value_unavailable.as_str())); }
    let count = array.map_or(object.len(), Vec::len); let error = || semio_framework_plugin::PluginAssemblyError::new("ui.inspection.output", "fixed UI output admission failed");
    let row = semio_framework_ui_contract::tree_item(crate::ui_label(label)?).try_id(key).map_err(|_| error())?;
    semio_framework_plugin::tree_window_indexed_item(windows, row, key, true, count, |index| {
        let (name, value) = if let Some(array) = array { ((index + 1).to_string(), &array[index]) } else { let (name, value) = object[index]; (name.to_string(), value) };
        let name = if name.len() <= semio_framework_ui_contract::UI_TEXT_MAX_BYTES / 2 { name } else { (index + 1).to_string() };
        reflected_output(windows, &format!("{key}.{index}"), &name, value, depth + 1, labels)
    })
}

/// 🥽️ Indexed polygon data shares the owning text port and retained tree windows.
fn mesh_source_input(windows: &semio_framework_plugin::TreeWindows<'_>, widget: &str, channel: &str, text: &str, labels: &Generation3dLabels) -> UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let key = "procedural-play-inspector.mesh";
    let Ok(source) = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject) else { return tree_item(key, labels.mesh_invalid.as_str()); };
    let error = || semio_framework_plugin::PluginAssemblyError::new("ui.inspection.mesh", "fixed UI mesh admission failed");
    let mut group = semio_framework_ui_contract::tree_item(crate::ui_label(labels.mesh_source.as_str())?).try_id(key).map_err(|_| error())?;
    let empty_attributes = semio_framework_pack_json::Value::Object(Default::default());
    for (index, name, label) in [(0, "vertices", labels.input_name_vertices.as_str()), (1, "faces", labels.input_name_faces.as_str()), (2, "attributes", labels.mesh_attributes.as_str()), (3, "materials", labels.mesh_materials.as_str()), (4, "textures", labels.mesh_textures.as_str())] {
        if let Some(value) = source.get(name).or_else(|| matches!(name, "attributes" | "materials" | "textures").then_some(&empty_attributes)) { group = group.try_child(mesh_source_node(windows, &format!("{key}.{index}"), label, widget, channel, &[name.into()], value, &source, labels)?).map_err(|_| error())?; }
    }
    group.try_build().map_err(|_| error())
}

/// 🌳️ Primitive geometry fields bind their exact schema path without exposing JSON text.
fn mesh_source_node(windows: &semio_framework_plugin::TreeWindows<'_>, key: &str, label: &str, widget: &str, channel: &str, path: &[String], value: &semio_framework_pack_json::Value, source: &semio_framework_pack_json::Value, labels: &Generation3dLabels) -> UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let error = || semio_framework_plugin::PluginAssemblyError::new("ui.inspection.mesh", "fixed UI mesh admission failed");
    if path.len() > 16 { return tree_item(key, labels.input_text_too_long.as_str()); }
    if matches!(path[0].as_str(), "materials" | "textures") && path.len() == 2 { return mesh_asset_input(windows, key, label, widget, channel, path, value, source, labels); }
    let object_children: Vec<_> = value.as_object().map(|object| object.iter().collect()).unwrap_or_default();
    if value.as_array().is_some() || value.as_object().is_some() {
        let array = value.as_array();
        let editable_array = array.is_some() && (path.len() == 1 || path[0] == "faces" && path.len() == 2 || path[0] == "attributes" && path.last().is_some_and(|name| matches!(name.as_str(), "values" | "indices")));
        let count = array.map_or(object_children.len(), Vec::len);
        let mut group = semio_framework_ui_contract::tree_item(crate::ui_label(label)?).try_id(key).map_err(|_| error())?;
        if path == ["textures"] && count < 256 && windows.is_open(key, true) {
            let name = (1..=257).map(|index| format!("texture{index}")).find(|name| value.get(name).is_none()).unwrap();
            group = group.try_child(mesh_texture_button(&format!("{key}.import"), labels.mesh_import_texture.as_str(), widget, channel, &name)?).map_err(|_| error())?;
        }
        if path == ["materials"] && count < 10_000 && windows.is_open(key, true) {
            let name = (1..=10_001).map(|index| format!("material{index}")).find(|name| value.get(name).is_none()).unwrap();
            group = group.try_child(mesh_source_button(&format!("{key}.add"), labels.mesh_add_material.as_str(), widget, channel, &["materials".into(), name], "add", None, "")?).map_err(|_| error())?;
        }
        if path == ["attributes"] && count < 64 && windows.is_open(key, true) {
            let mut actions = semio_framework_ui_contract::tree_item(crate::ui_label(format!("{}: {}", labels.input_list_add.as_str(), labels.mesh_attributes.as_str()))?).try_id(format!("{key}.actions")).map_err(|_| error())?;
            for (preset, action_label) in [("uv", labels.mesh_add_uv.as_str()), ("normal", labels.mesh_add_normal.as_str()), ("color", labels.mesh_add_color.as_str()), ("number", labels.mesh_add_number.as_str()), ("text", labels.mesh_add_text.as_str()), ("boolean", labels.mesh_add_boolean.as_str()), ("vector", labels.mesh_add_vector.as_str())] {
                let name = (1..=65).map(|index| if index == 1 { preset.into() } else { format!("{preset}{index}") }).find(|name: &String| value.get(name).is_none()).unwrap_or_else(|| preset.into());
                actions = actions.try_child(mesh_source_button(&format!("{key}.add.{preset}"), action_label, widget, channel, &["attributes".into(), name], "add", None, preset)?).map_err(|_| error())?;
            }
            for (preset, action_label) in [("number", labels.mesh_add_number.as_str()), ("text", labels.mesh_add_text.as_str()), ("boolean", labels.mesh_add_boolean.as_str()), ("vector", labels.mesh_add_vector.as_str())] {
                for (domain, domain_label) in [("face", labels.mesh_face_domain.as_str()), ("edge", labels.mesh_edge_domain.as_str()), ("corner", labels.mesh_corner_domain.as_str())] {
                    let name = (1..=65).map(|index| format!("{preset}{domain}{index}")).find(|name| value.get(name).is_none()).unwrap();
                    actions = actions.try_child(mesh_source_button(&format!("{key}.add.{preset}.{domain}"), &format!("{action_label}: {domain_label}"), widget, channel, &["attributes".into(), name], "add", None, &format!("{preset}/{domain}"))?).map_err(|_| error())?;
                }
            }
            if let Some(materials) = source.get("materials").and_then(semio_framework_pack_json::Value::as_object) {
                let names: Vec<_> = materials.iter().map(|(name, _)| name).collect();
                let assignments_key = format!("{key}.assign");
                let assignments = semio_framework_ui_contract::tree_item(crate::ui_label(labels.mesh_material_assignment.as_str())?).try_id(&assignments_key).map_err(|_| error())?;
                actions = actions.try_child(semio_framework_plugin::tree_window_indexed_item(windows, assignments, &assignments_key, false, names.len(), |index| {
                    let material = names[index]; let name = (1..=65).map(|index| format!("material{index}")).find(|name| value.get(name).is_none()).unwrap();
                    let button = mesh_source_button(&format!("{key}.assign.{index}"), &format!("{}: {material}", labels.mesh_material_assignment.as_str()), widget, channel, &["attributes".into(), name], "add", None, &format!("material/{material}"))?;
                    semio_framework_ui_contract::tree_item(crate::ui_label(material)?).try_id(format!("{key}.assign.{index}.row")).map_err(|_| error())?.try_child(button).map_err(|_| error())?.try_build().map_err(|_| error())
                })?).map_err(|_| error())?;
            }
            group = group.try_child(actions.try_build().map_err(|_| error())?).map_err(|_| error())?;
        }
        if path.first().is_some_and(|name| name == "attributes") && path.len() == 2 {
            let mut name_path = path.to_vec(); name_path.push("name".into());
            group = group.try_child(editable_field(&format!("{key}.name"), labels.input_variable_name.as_str(), &path[1], Some(InputKind::Text), mesh_source_fields(widget, channel, &name_path)?)?).map_err(|_| error())?;
            group = group.try_child(mesh_source_button(&format!("{key}.remove"), &format!("{label}: {}", labels.input_list_remove.as_str()), widget, channel, path, "remove", None, "")?).map_err(|_| error())?;
        }
        if editable_array && count < if path.len() == 1 { 100_000 } else { 600_000 } { group = group.try_child(mesh_source_button(&format!("{key}.add"), &format!("{label}: {}", labels.input_list_add.as_str()), widget, channel, path, "add", None, "")?).map_err(|_| error())?; }
        return semio_framework_plugin::tree_window_indexed_item(windows, group, key, true, count, |index| {
            let (name, value) = if let Some(array) = array { (index.to_string(), &array[index]) } else { let (name, value) = object_children[index]; (name.to_string(), value) };
            let mut child_path = path.to_vec(); child_path.push(name.clone());
            let name_label = match name.as_str() { "domain" => labels.mesh_domain.as_str(), "semantic" => labels.mesh_semantic.as_str(), "interpolation" => labels.mesh_interpolation.as_str(), "values" => labels.mesh_values.as_str(), "indices" => labels.mesh_indices.as_str(), _ => &name };
            let child_label = if let Ok(index) = name.parse::<usize>() { format!("{label} {} {}", labels.input_list_item.as_str(), index + 1) } else { format!("{label}: {name_label}") };
            let item_key = format!("{key}.{index}");
            let node = mesh_source_node(windows, &format!("{item_key}.field"), &child_label, widget, channel, &child_path, value, source, labels)?;
            if !editable_array { return Ok(node); }
            let mut row = semio_framework_ui_contract::tree_item(crate::ui_label(&child_label)?).try_id(&item_key).map_err(|_| error())?.try_child(node).map_err(|_| error())?;
            let minimum = if path == ["vertices"] || path[0] == "faces" && path.len() == 2 { 3 } else if path == ["faces"] { 1 } else { 0 };
            if count > minimum { row = row.try_child(mesh_source_button(&format!("{item_key}.remove"), &format!("{child_label}: {}", labels.input_list_remove.as_str()), widget, channel, &child_path, "remove", None, "")?).map_err(|_| error())?; }
            for (suffix, action_label, destination) in [("up", labels.input_list_up.as_str(), index.checked_sub(1)), ("down", labels.input_list_down.as_str(), (index + 1 < count).then_some(index + 1))] {
                if let Some(destination) = destination { row = row.try_child(mesh_source_button(&format!("{item_key}.{suffix}"), &format!("{child_label}: {action_label}"), widget, channel, &child_path, "move", Some(destination), "")?).map_err(|_| error())?; }
            }
            row.try_build().map_err(|_| error())
        });
    }
    if value.as_str().is_none() && value.as_f64().is_none() && value.as_bool().is_none() { return tree_item(key, label); }
    let text = value.as_str().map(str::to_owned).unwrap_or_else(|| value.to_string());
    if text.len() > semio_framework_ui_contract::UI_TEXT_MAX_BYTES || label.len() > semio_framework_ui_contract::UI_TEXT_MAX_BYTES { return tree_item(key, labels.input_text_too_long.as_str()); }
    let fields = mesh_source_fields(widget, channel, path)?;
    editable_field(key, label, &text, if value.as_bool().is_some() { None } else { Some(if value.as_f64().is_some() { InputKind::Number } else { InputKind::Text }) }, fields)
}

/// 🎨️ Asset fields remain reachable through the same bounded tree window authority.
fn mesh_asset_input(windows: &semio_framework_plugin::TreeWindows<'_>, key: &str, label: &str, widget: &str, channel: &str, path: &[String], asset: &semio_framework_pack_json::Value, source: &semio_framework_pack_json::Value, labels: &Generation3dLabels) -> UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    use semio_framework_pack_json::Value;
    let error = || semio_framework_plugin::PluginAssemblyError::new("ui.inspection.asset", "fixed UI asset admission failed");
    let row = semio_framework_ui_contract::tree_item(crate::ui_label(label)?).try_id(key).map_err(|_| error())?;
    let fields = [("baseColor", labels.mesh_base_color.as_str(), Value::Array(vec![Value::from(1); 4])), ("metallic", labels.mesh_metallic.as_str(), Value::from(0)), ("roughness", labels.mesh_roughness.as_str(), Value::from(1)), ("emissive", labels.mesh_emissive.as_str(), Value::Array(vec![Value::from(0); 3])), ("normalScale", labels.mesh_normal_scale.as_str(), Value::Array(vec![Value::from(1); 2])), ("occlusionStrength", labels.mesh_occlusion_strength.as_str(), Value::from(1)), ("alphaCutoff", labels.mesh_alpha_cutoff.as_str(), Value::from(0.5)), ("doubleSided", labels.mesh_double_sided.as_str(), Value::from(false))];
    let roles = [("baseColorTexture", labels.mesh_base_color_texture.as_str()), ("metallicRoughnessTexture", labels.mesh_metallic_roughness_texture.as_str()), ("normalTexture", labels.mesh_normal_texture.as_str()), ("occlusionTexture", labels.mesh_occlusion_texture.as_str()), ("emissiveTexture", labels.mesh_emissive_texture.as_str())];
    let texture = path[0] == "textures";
    semio_framework_plugin::tree_window_indexed_item(windows, row, key, true, if texture { 5 } else { 16 }, |index| {
        if index == 0 { let mut name_path = path.to_vec(); name_path.push("name".into()); return editable_field(&format!("{key}.name"), labels.mesh_asset_name.as_str(), &path[1], Some(InputKind::Text), mesh_source_fields(widget, channel, &name_path)?); }
        if index == 1 { return mesh_source_button(&format!("{key}.remove"), labels.input_list_remove.as_str(), widget, channel, path, "remove", None, ""); }
        if texture { return match index {
            2 => mesh_texture_button(&format!("{key}.replace"), labels.mesh_replace_texture.as_str(), widget, channel, &path[1]),
            3 => tree_item(&format!("{key}.mime"), format!("MIME: {}", asset.get("mime").and_then(Value::as_str).unwrap_or(""))),
            _ => tree_item(&format!("{key}.bytes"), format!("{}: {} B", labels.mesh_values.as_str(), asset.get("bytes").and_then(Value::as_array).map_or(0, Vec::len))),
        }; }
        if index < 10 { let (field, field_label, default) = &fields[index - 2]; let mut field_path = path.to_vec(); field_path.push((*field).into()); return mesh_source_node(windows, &format!("{key}.{field}"), field_label, widget, channel, &field_path, asset.get(field).unwrap_or(default), source, labels); }
        if index == 10 { let mut field_path = path.to_vec(); field_path.push("alphaMode".into()); return mesh_source_select(&format!("{key}.alphaMode"), labels.mesh_alpha_mode.as_str(), widget, channel, &field_path, asset.get("alphaMode").and_then(Value::as_str).unwrap_or("OPAQUE"), &[("OPAQUE".into(), labels.mesh_alpha_opaque.as_str().into()), ("MASK".into(), labels.mesh_alpha_mask.as_str().into()), ("BLEND".into(), labels.mesh_alpha_blend.as_str().into())]); }
        let (role, role_label) = roles[index - 11];
        mesh_texture_role_input(windows, &format!("{key}.{role}.fields"), role_label, widget, channel, path, role, asset, source, labels)
    })
}

/// 🧵️ A texture role exposes its bounded choices only when its retained window is expanded.
fn mesh_texture_role_input(windows: &semio_framework_plugin::TreeWindows<'_>, key: &str, label: &str, widget: &str, channel: &str, path: &[String], role: &str, asset: &semio_framework_pack_json::Value, source: &semio_framework_pack_json::Value, labels: &Generation3dLabels) -> UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    use semio_framework_pack_json::Value;
    let error = || semio_framework_plugin::PluginAssemblyError::new("ui.inspection.texture-role", "fixed UI texture role admission failed");
    let row = semio_framework_ui_contract::tree_item(crate::ui_label(label)?).try_id(key).map_err(|_| error())?;
    semio_framework_plugin::tree_window_indexed_item(windows, row, key, false, 7, |index| {
        let mut role_path = path.to_vec(); role_path.push(role.into());
        let selected = asset.get(role).and_then(Value::as_str).unwrap_or("");
        if index == 0 { let mut options = vec![(String::new(), labels.mesh_no_texture.as_str().into())]; if let Some(textures) = source.get("textures").and_then(Value::as_object) { options.extend(textures.iter().take(semio_framework_ui_contract::UI_FIXED_LIST_ITEMS - 1).map(|(name, _)| (name.into(), name.into()))); } if !options.iter().any(|(value, _)| value == selected) { options.pop(); options.push((selected.into(), selected.into())); } return mesh_source_select(&format!("{key}.choice"), label, widget, channel, &role_path, selected, &options); }
        if index == 1 { return editable_field(&format!("{key}.name"), labels.mesh_asset_name.as_str(), selected, Some(InputKind::Text), mesh_source_fields(widget, channel, &role_path)?); }
        if index == 2 { let mut uv_path = path.to_vec(); uv_path.extend(["textureCoordinates".into(), role.into()]); let uv = asset.get("textureCoordinates").and_then(|value| value.get(role)).and_then(Value::as_u64).unwrap_or(0).to_string(); return editable_field(&format!("{key}.uv"), labels.mesh_uv_set.as_str(), &uv, Some(InputKind::Number), mesh_source_fields(widget, channel, &uv_path)?); }
        let (field, field_label, default, values) = [("wrapS", labels.mesh_wrap_s.as_str(), 10497, &[33071,33648,10497][..]), ("wrapT", labels.mesh_wrap_t.as_str(), 10497, &[33071,33648,10497][..]), ("magFilter", labels.mesh_mag_filter.as_str(), 9729, &[9728,9729][..]), ("minFilter", labels.mesh_min_filter.as_str(), 9729, &[9728,9729,9984,9985,9986,9987][..])][index - 3];
        let mut sampler_path = path.to_vec(); sampler_path.extend(["textureSamplers".into(), role.into(), field.into()]);
        let selected = asset.get("textureSamplers").and_then(|value| value.get(role)).and_then(|value| value.get(field)).and_then(Value::as_u64).unwrap_or(default).to_string();
        let options: Vec<_> = values.iter().map(|value| (value.to_string(), match value { 33071 => labels.mesh_wrap_clamp.as_str(), 33648 => labels.mesh_wrap_mirror.as_str(), 10497 => labels.mesh_wrap_repeat.as_str(), 9728 => labels.mesh_filter_nearest.as_str(), 9729 => labels.mesh_filter_linear.as_str(), 9984 => labels.mesh_filter_nearest_nearest.as_str(), 9985 => labels.mesh_filter_linear_nearest.as_str(), 9986 => labels.mesh_filter_nearest_linear.as_str(), _ => labels.mesh_filter_linear_linear.as_str() }.into())).collect();
        mesh_source_select(&format!("{key}.{field}"), field_label, widget, channel, &sampler_path, &selected, &options)
    })
}

/// 🎛️ Canonical choices publish through the same structured mesh input command.
fn mesh_source_select(key: &str, label: &str, widget: &str, channel: &str, path: &[String], value: &str, options: &[(String, String)]) -> UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let error = || semio_framework_plugin::PluginAssemblyError::new("ui.inspection.asset-select", "fixed UI asset select admission failed");
    let (action, args) = ActionFactory::new(GENERATION_3D_PLAY_APP_ID).action("setWidgetInput", Some(crate::ui_value_map(mesh_source_fields(widget, channel, path)?)?))?;
    let mut select = semio_framework_ui_contract::select(crate::ui_text(value)?).try_id(format!("{key}.input")).and_then(|select| select.try_label(label)).map_err(|_| error())?;
    for (value, label) in options { select = select.try_item(crate::ui_text(value)?, crate::ui_label(label)?).map_err(|_| error())?; }
    let select = match args { Some(args) => select.try_on_with(Trigger::Change, action, args), None => select.try_on(Trigger::Change, action) }.map_err(|_| error())?.try_build().map_err(|_| error())?;
    semio_framework_ui_contract::tree_item(crate::ui_label(label)?).try_id(key).map_err(|_| error())?.try_child(select).map_err(|_| error())?.try_build().map_err(|_| error())
}

/// 🔘 A source list action publishes through the same absolute text input mutation.
fn mesh_source_button(key: &str, label: &str, widget: &str, channel: &str, path: &[String], operation: &str, destination: Option<usize>, value: &str) -> UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let error = || semio_framework_plugin::PluginAssemblyError::new("ui.inspection.mesh-button", "fixed UI mesh button admission failed");
    let mut fields = mesh_source_fields(widget, channel, path)?;
    fields.push(("operation", crate::ui_value_text(operation)?)); fields.push(("value", crate::ui_value_text(value)?));
    if let Some(destination) = destination { fields.push(("destination", UiValue::Number(destination as f64))); }
    let (action, args) = ActionFactory::new(GENERATION_3D_PLAY_APP_ID).action("setWidgetInput", Some(crate::ui_value_map(fields)?))?;
    let button = semio_framework_ui_contract::button(crate::ui_label(label)?).try_id(key).map_err(|_| error())?;
    match args { Some(args) => button.try_on_with(Trigger::Activate, action, args), None => button.try_on(Trigger::Activate, action) }.map_err(|_| error())?.try_build().map_err(|_| error())
}

/// 🖼️ File selection retains the exact source target in the existing import invocation.
fn mesh_texture_button(key: &str, label: &str, widget: &str, channel: &str, texture: &str) -> UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let error = || semio_framework_plugin::PluginAssemblyError::new("ui.inspection.texture-button", "fixed texture button admission failed");
    let fields = vec![("widgetId", crate::ui_value_text(widget)?), ("channel", crate::ui_value_text(channel)?), ("textureId", crate::ui_value_text(texture)?)];
    let (action, args) = ActionFactory::new(GENERATION_3D_PLAY_APP_ID).action("importDocumentRequest", Some(crate::ui_value_map(fields)?))?;
    let button = semio_framework_ui_contract::button(crate::ui_label(label)?).try_id(key).map_err(|_| error())?;
    match args { Some(args) => button.try_on_with(Trigger::Activate, action, args), None => button.try_on(Trigger::Activate, action) }.map_err(|_| error())?.try_build().map_err(|_| error())
}

/// 🧬️ Structured paths are retained action arguments on the existing input command.
fn mesh_source_fields(widget: &str, channel: &str, path: &[String]) -> UiAssemblyResult<Vec<(&'static str, UiValue)>> {
    Ok(vec![("widgetId", crate::ui_value_text(widget)?), ("channel", crate::ui_value_text(channel)?), ("facet", crate::ui_value_text("meshSource")?), ("path", ui_value_list(path.iter().map(|part| crate::ui_value_text(part)).collect::<UiAssemblyResult<Vec<_>>>()?)?)])
}

/// 📃️ Ordered typed inputs share the host's tree window and absolute command bindings.
fn collection_input(windows: &semio_framework_plugin::TreeWindows<'_>, key: &str, label: &str, widget: &str, channel: &str, items: &[crate::standards::v1::subsets::any::schema::mutations::change_widget_input::WidgetInputValue], cardinality: &semio_framework_os_flow::neural::Cardinality, labels: &Generation3dLabels) -> UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let error = || semio_framework_plugin::PluginAssemblyError::new("ui.inspection.collection", "fixed UI collection admission failed");
    let mut group = semio_framework_ui_contract::tree_item(crate::ui_label(label)?).try_id(key).map_err(|_| error())?;
    if items.len() < 1024 && cardinality.accepts(items.len() + 1) {
        group = group.try_child(collection_button(&format!("{key}.add"), &format!("{label}: {}", labels.input_list_add.as_str()), widget, channel, "add", items.len(), None)?).map_err(|_| error())?;
    }
    semio_framework_plugin::tree_window_indexed_item(windows, group, key, true, items.len(), |index| {
        let item_key = format!("{key}.{index}");
        let item_label = format!("{label} {} {}", labels.input_list_item.as_str(), index + 1);
        let literal = items[index].literal();
        let mut row = semio_framework_ui_contract::tree_item(crate::ui_label(&item_label)?).try_id(&item_key).map_err(|_| error())?;
        let schema = items[index].schema();
        if matches!(schema, "point" | "vector") {
            for axis in ["x", "y", "z"] {
                let value = literal.get(axis).and_then(semio_framework_value::DslValue::as_f64).unwrap_or(0.0).to_string();
                row = row.try_child(editable_item(&format!("{item_key}.{axis}"), &format!("{item_label} {axis}"), widget, channel, Some(axis), Some(index), &value, Some(InputKind::Number))?).map_err(|_| error())?;
            }
        } else {
            let value = literal.get("value").map(|value| value.as_str().map(str::to_owned).unwrap_or_else(|| semio_framework_pack_json::to_json_string(value))).unwrap_or_default();
            let control = if value.len() <= semio_framework_ui_contract::UI_TEXT_MAX_BYTES { editable_item(&format!("{item_key}.value"), &item_label, widget, channel, None, Some(index), &value, if schema == "boolean" { None } else { Some(if schema == "number" { InputKind::Number } else { InputKind::LongText }) })? } else { tree_item(&format!("{item_key}.value"), labels.input_text_too_long.as_str())? };
            row = row.try_child(control).map_err(|_| error())?;
        }
        let mut buttons = Vec::new();
        if cardinality.accepts(items.len() - 1) { buttons.push(("remove", labels.input_list_remove.as_str(), None)); }
        if index > 0 { buttons.push(("up", labels.input_list_up.as_str(), Some(index - 1))); }
        if index + 1 < items.len() { buttons.push(("down", labels.input_list_down.as_str(), Some(index + 1))); }
        for (suffix, action_label, destination) in buttons {
            row = row.try_child(collection_button(&format!("{item_key}.{suffix}"), &format!("{item_label}: {action_label}"), widget, channel, if destination.is_some() { "move" } else { "remove" }, index, destination)?).map_err(|_| error())?;
        }
        row.try_build().map_err(|_| error())
    })
}

/// 🔘 A list action publishes its exact index and destination through the retained command.
fn collection_button(key: &str, label: &str, widget: &str, channel: &str, operation: &str, index: usize, destination: Option<usize>) -> UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut fields = vec![("widgetId", crate::ui_value_text(widget)?), ("channel", crate::ui_value_text(channel)?), ("operation", crate::ui_value_text(operation)?), ("index", UiValue::Number(index as f64)), ("value", crate::ui_value_text("")?)];
    if let Some(destination) = destination { fields.push(("destination", UiValue::Number(destination as f64))); }
    let (action, args) = ActionFactory::new(GENERATION_3D_PLAY_APP_ID).action("setWidgetInput", Some(crate::ui_value_map(fields)?))?;
    let button = semio_framework_ui_contract::button(crate::ui_label(label)?).try_id(key).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.list-button", "fixed UI button admission failed"))?;
    match args { Some(args) => button.try_on_with(Trigger::Activate, action, args), None => button.try_on(Trigger::Activate, action) }.map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.list-binding", "fixed UI button admission failed"))?.try_build().map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.inspection.list-button", "fixed UI button admission failed"))
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
