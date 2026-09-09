//! 🛍️ Generation3d play app panel — the flow-node catalogue.
//!
//! 🧾️ The listing is **virtualised**, not a row per operator. With the real `brep`/`math` operator sets
//! installed `flow_palette_catalogue_sections` offers hundreds of entries, while one built node admits
//! `UI_BUILT_CHILDREN_MAX` children and every interactive row costs `UI_VALUE_ROW_COLLECTIONS`/
//! `UI_VALUE_ROW_ITEMS` of the process-wide argument arena. The previous flat build pushed every entry
//! into one `UiFixedList` and refused at the 33rd with `ui.catalogue.items: fixed UI catalogue admission
//! failed`, taking the whole panel with it (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1). This build
//! materialises ONE bounded page — `semio_framework_plugin::panel_page_rows` rows across every catalogue
//! group together — and closes each truncated group with a continuation row naming the omitted count.
//! The unbounded browse surface is the canvas spotlight, which reads the app-static catalogue published
//! on the reserved `framework.section.catalogue` retained surface.

use crate::editor::generation3d::terminology::Generation3dLabels;
use crate::editor::generation3d::GENERATION_3D_PLAY_APP_ID;
use semio_framework_plugin::plugin_app_close_prelude::Component;
use semio_framework_plugin::{
    paged_panel_section, panel_continuation_row, panel_page_rows, tree_group, tree_item_with_action, ActionFactory, LocalizedLabel, PanelGroup, PanelRowBudget, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError,
    UiFixedList,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
};

//#region 🔖️Constants
pub const GENERATION_3D_PLAY_BODY_CATALOGUE: &str = "procedural.play.catalogue";
/// 🗂️ Rows one catalogue group materialises before it truncates — a built node admits
/// `UI_BUILT_CHILDREN_MAX` (32) children and the last of them carries the group's continuation row.
const CATALOGUE_GROUP_ROWS: usize = 31;
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

fn catalogue_row(item: &semio_framework_os_flow::CatalogueItem) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let action_kind = if item.kind == "neuron" { format!("neuron|{}", item.neuron_kind.as_deref().unwrap_or("math.add")) } else { item.kind.clone() };
    let icon = if item.icon.starts_with("emoji:") { "box" } else { item.icon.as_str() };
    let args = crate::ui_value_map([("kind", crate::ui_value_text(&action_kind)?)])?;
    let mut node = tree_item_with_action(
        identity::item_key(&item.kind, item.neuron_kind.as_deref(), item.format.as_deref(), item.action.as_deref()),
        item.name.clone(),
        None,
        ActionFactory::new(GENERATION_3D_PLAY_APP_ID).action("addWidget", Some(args))?,
    )?;
    if let Component::TreeItem(props) = &mut node.component {
        props.icon = Some(crate::ui_text(icon)?);
    }
    Ok(node)
}

pub fn render(labels: &Generation3dLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let sections = semio_framework_os_flow::flow_palette_catalogue_sections();
    let budget = &mut PanelRowBudget::new(panel_page_rows());
    let mut groups = UiFixedList::default();
    let total = sections.len();
    for (index, section) in sections.iter().enumerate() {
        let group_id = format!("procedural-play-catalogue.{}", section.id);
        let items = budget.nested(total.saturating_sub(index + 1), |share| paged_panel_section(&group_id, &section.items, CATALOGUE_GROUP_ROWS, share, |item, _| catalogue_row(item)))?;
        if items.is_empty() {
            continue;
        }
        let group = tree_group(&group_id, section.title.clone(), index == 0, items)?;
        if groups.try_push(group).is_err() {
            break;
        }
    }
    // 🛟️ A spent page is still a correct page: when the process-wide argument arena has no credit left
    // (another panel in this same process already holds it), the listing degrades to a single
    // continuation row naming the omitted count instead of refusing the whole panel — the law
    // `ui_value_headroom`'s own docstring states.
    if groups.is_empty() {
        let omitted = sections.iter().map(|section| section.items.len()).sum::<usize>();
        groups.try_push(panel_continuation_row("procedural-play-catalogue.widgets", omitted)?).map_err(|_| PluginAssemblyError::new("ui.catalogue.items", "fixed UI catalogue admission failed"))?;
    }
    PanelTreeBuilder::new("procedural-play-catalogue")?.section("procedural-play-catalogue.widgets", Some(crate::ui_label(labels.widgets.as_str())?), true, groups)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
