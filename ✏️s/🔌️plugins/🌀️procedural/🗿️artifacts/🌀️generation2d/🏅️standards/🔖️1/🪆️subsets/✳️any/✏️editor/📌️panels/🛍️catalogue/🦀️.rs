//! 🛍️ Generation2d play app panel — the flow-node catalogue and show-mode toggles.
//!
//! 🪟 Each group is one window over `flow_palette_catalogue_sections`, the same roster generation3d
//! lists. A closed group stamps its total and materialises nothing. Component rows drag
//! `application/x-flow-widget`.

use crate::editor::generation2d::terminology::Generation2dLabels;
use crate::editor::generation2d::GENERATION2D_PLAY_APP_ID;
use semio_framework_plugin::plugin_app_close_prelude::Component;
use semio_framework_plugin::{
    tree_item_with_action, tree_item_with_action_draggable, tree_window_item, ActionFactory, HasBase, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
};

//#region 🔖️Constants
pub const GENERATION2D_PLAY_BODY_CATALOGUE: &str = "generation2d.play.catalogue";
pub const GENERATION2D_PLAY_CATALOGUE_SECTION: &str = "procedural2d-play-catalogue.widgets";
/// 🖱️ MIME `tree_item_with_action_draggable` stores the widget descriptor under.
const FLOW_WIDGET_DRAG_MIME: &str = "application/x-flow-widget";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(GENERATION2D_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn catalogue_error(scope: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.catalogue", scope)
}

fn item_key(item: &semio_framework_os_flow::CatalogueItem) -> String {
    let variant = match item.kind.as_str() {
        "neuron" => item.neuron_kind.as_deref(),
        "outputExport" => item.format.as_deref(),
        "outputAction" => item.action.as_deref(),
        _ => None,
    };
    match variant {
        Some(variant) => format!("procedural2d-play-catalogue.{}.{}", item.kind, variant),
        None => format!("procedural2d-play-catalogue.{}", item.kind),
    }
}

/// 🖱️ Widget descriptor JSON: kind, neuronKind for neurons, and format or action when the item has them.
fn widget_descriptor(item: &semio_framework_os_flow::CatalogueItem) -> dsl::os_pack::json::Value {
    let kind = ("kind".to_string(), dsl::os_pack::json::Value::String(item.kind.clone()));
    if item.kind == "neuron" {
        let neuron = item.neuron_kind.clone().unwrap_or_else(|| "math.add".into());
        return dsl::os_pack::json::object([kind, ("neuronKind".to_string(), dsl::os_pack::json::Value::String(neuron))]);
    }
    if let Some(format) = item.format.clone() {
        return dsl::os_pack::json::object([kind, ("format".to_string(), dsl::os_pack::json::Value::String(format))]);
    }
    if let Some(action) = item.action.clone() {
        return dsl::os_pack::json::object([kind, ("action".to_string(), dsl::os_pack::json::Value::String(action))]);
    }
    dsl::os_pack::json::object([kind])
}

fn widget_drag_json(item: &semio_framework_os_flow::CatalogueItem) -> dsl::os_pack::json::Value {
    let descriptor = widget_descriptor(item);
    dsl::os_pack::json::object([(FLOW_WIDGET_DRAG_MIME.to_string(), dsl::os_pack::json::Value::String(descriptor.to_string()))])
}

fn click_args(item: &semio_framework_os_flow::CatalogueItem) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut pairs = vec![("kind", crate::ui_value_text(if item.kind == "neuron" { "neuron" } else { item.kind.as_str() })?)];
    if item.kind == "neuron" {
        pairs.push(("neuronKind", crate::ui_value_text(item.neuron_kind.as_deref().unwrap_or("math.add"))?));
    }
    if let Some(format) = item.format.as_deref() {
        pairs.push(("format", crate::ui_value_text(format)?));
    }
    if let Some(action) = item.action.as_deref() {
        pairs.push(("action", crate::ui_value_text(action)?));
    }
    crate::ui_value_map(pairs)
}

fn catalogue_row(item: &semio_framework_os_flow::CatalogueItem) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let drag = widget_drag_json(item);
    let icon = if item.icon.starts_with("emoji:") { "box" } else { item.icon.as_str() };
    let mut node = tree_item_with_action_draggable(item_key(item), item.name.clone(), None, ActionFactory::new(GENERATION2D_PLAY_APP_ID).action("addWidget", Some(click_args(item)?))?, &drag)?;
    if let Component::TreeItem(props) = &mut node.component {
        props.icon = Some(crate::ui_text(icon)?);
    }
    Ok(node)
}

fn group_id(section: &semio_framework_os_flow::CatalogueSection) -> String {
    format!("procedural2d-play-catalogue.{}", section.id)
}

/// 🗂️ A palette group whose children are that group's own window. The group row is not a pick target.
fn group_row(windows: &TreeWindows<'_>, section: &semio_framework_os_flow::CatalogueSection, default_open: bool) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let id = group_id(section);
    let item = semio_framework_ui_contract::tree_item(crate::ui_label(section.title.as_str())?).try_id(&id).map_err(|_| catalogue_error("group.id"))?;
    tree_window_item(windows, item, &id, default_open, &section.items, catalogue_row)
}

pub fn render(labels: &Generation2dLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let sections = semio_framework_os_flow::flow_palette_catalogue_sections();
    let first_group = sections.first().map(|section| section.id.clone()).unwrap_or_default();
    let modes = ["preview", "generate", "wire"];
    PanelTreeBuilder::new("procedural2d-play-catalogue")?
        .window_section(windows, GENERATION2D_PLAY_CATALOGUE_SECTION, Some(crate::ui_label(labels.components.as_str())?), true, &sections, |section| group_row(windows, section, section.id == first_group))?
        .window_section(windows, "procedural2d-play-catalogue.modes", Some(crate::ui_label(labels.show_mode_section.as_str())?), false, &modes, |mode| {
            let args = crate::ui_value_map([("value", crate::ui_value_text(mode)?)])?;
            tree_item_with_action(format!("procedural2d-play-catalogue.mode.{mode}"), format!("{} {mode}", labels.show_prefix.as_str()), None, ActionFactory::new(GENERATION2D_PLAY_APP_ID).action("setShowMode", Some(args))?)
        })?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
