//! 🛍️ Flow play app panel — the catalogue: draggable widget/operator palette plus the extension sections.

use crate::artifacts::flow::FlowSnapshot;
use crate::editor::flow::commands::run_extension_action::FLOW_AUTOMATIONS;
use crate::editor::flow::config::FlowConfig;
use crate::editor::flow::{flow_action, ui_node_list, ui_value_bool, ui_value_map, ui_value_text};
use crate::editor::flow::host_from_snapshot;
use crate::editor::flow::terminology::{flow_extension_action_title_label, flow_extension_label, FlowPlayLabels};
use flow::FlowEvalSession;
use semio_framework_plugin::plugin_app_close_prelude::Label;
use semio_framework_plugin::{
    tree_item_with_action, tree_item_with_action_draggable, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
};
use serde_json::Value;

/// 🏷️ Converts catalogue/section titles into the panel builder's `Label`.
fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<Label> {
    Label::try_from(value.as_ref().to_string()).map_err(|error| PluginAssemblyError::new("ui.catalogue", error))
}

//#region 🔖️Constants
pub const FLOW_PLAY_BODY_CATALOGUE: &str = "flow.play.catalogue";
/// 🖱️ MIME key `tree_item_with_action_draggable` reads its drag payload under — see
/// [`flow_widget_drag_json`].
pub const FLOW_WIDGET_DRAG_MIME: &str = "application/x-flow-widget";
//#endregion 🔖️Constants

//#region 🔖️WidgetDescriptors
pub fn flow_widget_descriptor(kind: &str, neuron_kind: Option<&str>) -> dsl::os_pack::json::Value {
    if kind == "neuron" {
        dsl::os_pack::json::object([("kind".to_string(), dsl::os_pack::json::Value::String("neuron".to_string())), ("neuronKind".to_string(), dsl::os_pack::json::Value::String(neuron_kind.unwrap_or(kind).to_string()))])
    } else {
        dsl::os_pack::json::object([("kind".to_string(), dsl::os_pack::json::Value::String(kind.to_string()))])
    }
}

/// 🪢️ Wraps a widget descriptor into the `{mime: payload}` JSON shape `tree_item_with_action_draggable`
/// expects for its drag-data map.
pub fn flow_widget_drag_json(descriptor: &dsl::os_pack::json::Value) -> dsl::os_pack::json::Value {
    dsl::os_pack::json::object([(FLOW_WIDGET_DRAG_MIME.to_string(), dsl::os_pack::json::Value::String(descriptor.to_string()))])
}
//#endregion 🔖️WidgetDescriptors

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(FLOW_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(fixture: &FlowSnapshot, config: &FlowConfig, session: &FlowEvalSession, labels: &FlowPlayLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let host = host_from_snapshot(fixture, config, session);
    let raw = host.catalogue_json().map_err(|error| PluginAssemblyError::new("ui.catalogue", error.to_string()))?;
    let catalogue: Value = serde_json::from_str(&raw).map_err(|error| PluginAssemblyError::new("ui.catalogue", error.to_string()))?;
    let sections = catalogue.as_array().ok_or_else(|| PluginAssemblyError::new("ui.catalogue", "flow catalogue root must be an array"))?;
    if sections.is_empty() {
        return Err(PluginAssemblyError::new("ui.catalogue", "flow catalogue must contain at least one section"));
    }
    let mut builder = PanelTreeBuilder::new("flow-play-catalogue")?;
    for section in sections {
        let id = section.get("id").and_then(Value::as_str).ok_or_else(|| PluginAssemblyError::new("ui.catalogue", "flow catalogue section id is required"))?;
        let title = section.get("title").and_then(Value::as_str).unwrap_or(id);
        let entries = section.get("items").and_then(Value::as_array).ok_or_else(|| PluginAssemblyError::new("ui.catalogue", "flow catalogue section items are required"))?;
        let items = ui_node_list(entries.iter().map(|entry| {
            let kind = entry.get("kind").and_then(Value::as_str).ok_or_else(|| PluginAssemblyError::new("ui.catalogue", "flow catalogue item kind is required"))?;
            let label = entry.get("name").or_else(|| entry.get("abbreviation")).and_then(Value::as_str).unwrap_or(kind);
            let neuron_kind = (kind == "neuron").then(|| entry.get("neuronKind").and_then(Value::as_str)).flatten();
            let descriptor = flow_widget_descriptor(kind, neuron_kind);
            let action_args = match neuron_kind {
                Some(neuron_kind) => ui_value_map([("kind", ui_value_text("neuron")?), ("neuronKind", ui_value_text(neuron_kind)?)])?,
                None => ui_value_map([("kind", ui_value_text(kind)?)])?,
            };
            tree_item_with_action_draggable(
                format!("flow-play-catalogue.{id}.{kind}.{label}"),
                label,
                Some(kind.to_string()),
                flow_action("addWidget", Some(action_args))?,
                &flow_widget_drag_json(&descriptor),
            )
        }))?;
        builder = builder.section(format!("flow-play-catalogue.{id}"), Some(ui_label(title)?), true, items)?;
    }
    append_extension_sections(builder, config, labels)?.build()
}

/// 🧩️ Installed/enabled extension palette plus actions surfaced by active extensions.
fn append_extension_sections(mut builder: PanelTreeBuilder, config: &FlowConfig, labels: &FlowPlayLabels) -> semio_framework_plugin::UiAssemblyResult<PanelTreeBuilder> {
    let extension_enabled = config.automation_enabled();
    let installed = ui_node_list(FLOW_AUTOMATIONS.iter().map(|(id, name, _, _, _)| {
            let enabled = extension_enabled.get(*id).copied().unwrap_or(false);
            let args = ui_value_map([("enabled", ui_value_bool(!enabled)), ("id", ui_value_text(id)?)])?;
            tree_item_with_action(
                format!("flow-play-extensions.{id}"),
                flow_extension_label(id, name, labels).into_string(),
                Some(if enabled { "enabled".into() } else { "disabled".into() }),
                flow_action("toggleExtension", Some(args))?,
            )
        }))?;
    let actions = ui_node_list(FLOW_AUTOMATIONS
        .iter()
        .filter(|(id, ..)| extension_enabled.get(*id).copied().unwrap_or(false))
        .map(|(_, _, action_id, title, _)| {
            let args = ui_value_map([("actionId", ui_value_text(action_id)?)])?;
            tree_item_with_action(format!("flow-play-extensions.action.{action_id}"), flow_extension_action_title_label(action_id, title, labels).into_string(), Some((*action_id).into()), flow_action("runExtensionAction", Some(args))?)
        }))?;
    builder = builder.section("flow-play-extensions.installed", Some(ui_label(labels.extensions.as_str())?), false, installed)?;
    if !actions.is_empty() {
        builder = builder.section("flow-play-extensions.actions", Some(ui_label(labels.extension_actions.as_str())?), false, actions)?;
    }
    Ok(builder)
}

/// 🛟️ Used when the host catalogue is empty (a fresh/offline session) — a minimal hand-authored palette.
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::flow::testkit::{flow_app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn flow_widget_drag_json_wraps_descriptor_under_drag_mime() {
        let descriptor = flow_widget_descriptor("neuron", Some("math.add"));
        let drag = flow_widget_drag_json(&descriptor);
        assert!(drag.get(FLOW_WIDGET_DRAG_MIME).is_some());
    }

    #[semio_framework_async_macros::async_test]
    async fn catalogue_lists_module_operators() {
        let mut app = flow_app();
        let json = render_body(&mut app, FLOW_PLAY_BODY_CATALOGUE);
        assert!(json.contains("flow-play-catalogue.math"), "expected math module section: {json}");
        assert!(json.contains("math.add"), "expected math.add operator: {json}");
    }

    #[semio_framework_async_macros::async_test]
    async fn catalogue_items_export_flow_widget_drag_payload() {
        let mut app = flow_app();
        let json = render_body(&mut app, FLOW_PLAY_BODY_CATALOGUE);
        assert!(json.contains(FLOW_WIDGET_DRAG_MIME), "missing drag mime: {json}");
        assert!(json.contains(r#""draggable":true"#) || json.contains(r#""draggable": true"#));
    }

    #[semio_framework_async_macros::async_test]
    async fn every_built_in_extension_is_listed_in_the_installed_section() {
        let mut app = flow_app();
        let json = render_body(&mut app, FLOW_PLAY_BODY_CATALOGUE);
        for (id, ..) in FLOW_AUTOMATIONS {
            assert!(json.contains(&format!("flow-play-extensions.{id}")), "extension {id} missing: {json}");
        }
    }
}
//#endregion 🧪️Tests
