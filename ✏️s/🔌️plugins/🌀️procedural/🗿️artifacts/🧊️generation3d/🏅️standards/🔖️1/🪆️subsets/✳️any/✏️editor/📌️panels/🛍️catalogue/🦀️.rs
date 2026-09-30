//! 🛍️ Generation3d play app panel — the flow-node catalogue.
//!
//! 🪟️ The listing is **windowed**, not paged. With the real `brep`/`math` operator sets installed
//! `flow_palette_catalogue_sections` offers hundreds of entries; each catalogue group is one windowed
//! container that reports its full `total` and materialises only the rows the host's viewport asked
//! for, so the scrollbar spans the whole roster and nothing is ever summarised as a `+n` the reader
//! cannot open (ticket 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING, replacing the bounded page of
//! ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1). Only the first group opens by default; the rest
//! stamp their `total` with zero children until the reader expands them.
//! The canvas spotlight keeps reading the same roster off the reserved `framework.section.catalogue`
//! retained surface — it is the search surface, no longer an overflow valve.

use crate::editor::generation3d::terminology::Generation3dLabels;
use crate::editor::generation3d::GENERATION_3D_PLAY_APP_ID;
use semio_framework_plugin::plugin_app_close_prelude::Component;
use semio_framework_plugin::{
    tree_item_with_action_draggable, tree_window_item, ActionFactory, HasBase, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
};

//#region 🔖️Constants
pub const GENERATION_3D_PLAY_BODY_CATALOGUE: &str = "procedural.play.catalogue";
pub const GENERATION_3D_PLAY_CATALOGUE_SECTION: &str = "procedural-play-catalogue.widgets";
const GENERATION_3D_WIDGET_DRAG_MIME: &str = "application/x-flow-widget";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(GENERATION_3D_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
#[path = "🪪️identity/🦀️.rs"]
mod identity;

fn catalogue_error(scope: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.catalogue", scope)
}

/// 🖱️ Drag payload the flow canvas already accepts: MIME `application/x-flow-widget` whose value is the widget descriptor JSON.
fn widget_drag_json(item: &semio_framework_os_flow::CatalogueItem) -> dsl::os_pack::json::Value {
    let payload = crate::editor::generation3d::commands::add_widget::AddWidget::from_catalogue(item);
    let descriptor = dsl::os_pack::json::object(payload.descriptor_fields().into_iter().map(|(name, value)| (name.into(), dsl::os_pack::json::Value::String(value.into()))));
    dsl::os_pack::json::object([(GENERATION_3D_WIDGET_DRAG_MIME.to_string(), dsl::os_pack::json::Value::String(descriptor.to_string()))])
}

fn catalogue_row(labels: &Generation3dLabels, item: &semio_framework_os_flow::CatalogueItem) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let payload = crate::editor::generation3d::commands::add_widget::AddWidget::from_catalogue(item);
    let icon = if item.icon.starts_with("emoji:") { "box" } else { item.icon.as_str() };
    let fields = payload.descriptor_fields().into_iter().map(|(name, value)| Ok((name, crate::ui_value_text(value)?))).collect::<semio_framework_plugin::UiAssemblyResult<Vec<_>>>()?;
    let args = crate::ui_value_map(fields)?;
    let name = crate::editor::generation3d::terminology::generation3d_catalogue_name(labels, item.neuron_kind.as_deref().unwrap_or(&item.kind), &item.name);
    let name = item.format.as_ref().map_or_else(|| name.to_string(), |format| format!("{name} {}", format.to_uppercase()));
    let mut node = tree_item_with_action_draggable(
        identity::item_key(&item.kind, item.neuron_kind.as_deref(), item.format.as_deref(), item.action.as_deref()),
        name,
        (!item.summary.is_empty()).then(|| item.summary.clone()),
        ActionFactory::new(GENERATION_3D_PLAY_APP_ID).action("addWidget", Some(args))?,
        &widget_drag_json(item),
    )?;
    if let Component::TreeItem(props) = &mut node.component {
        props.icon = Some(crate::ui_text(icon)?);
    }
    Ok(node)
}

fn group_id(section: &semio_framework_os_flow::CatalogueSection) -> String {
    format!("procedural-play-catalogue.{}", section.id)
}

/// 🗂️ One catalogue group: a bindingless row whose children are the group's own window. The group
/// row itself is never a pick target and carries no action — clicking it opens the window, which is
/// what makes the listing lazy: a closed group stamps its `total` and materialises nothing.
fn group_row(labels: &Generation3dLabels, windows: &TreeWindows<'_>, section: &semio_framework_os_flow::CatalogueSection, default_open: bool) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let id = group_id(section);
    let item = semio_framework_ui_contract::tree_item(crate::ui_label(crate::editor::generation3d::terminology::generation3d_catalogue_name(labels, &section.id, &section.title))?).try_id(&id).map_err(|_| catalogue_error("group.id"))?;
    tree_window_item(windows, item, &id, default_open, &section.items, |item| catalogue_row(labels, item))
}

pub fn render(labels: &Generation3dLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let sections = semio_framework_os_flow::flow_palette_catalogue_sections();
    let first_group = sections.first().map(|section| section.id.clone()).unwrap_or_default();
    PanelTreeBuilder::new("procedural-play-catalogue")?
        .window_section(windows, GENERATION_3D_PLAY_CATALOGUE_SECTION, Some(crate::ui_label(labels.widgets.as_str())?), true, &sections, |section| group_row(labels, windows, section, section.id == first_group))?
        .build()
}
//#endregion 🔖️Render


#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
