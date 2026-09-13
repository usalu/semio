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

/// 🧮️ The operators this render materialised as rows, and the operators it did not. Their sum is
/// `flow_palette_catalogue_sections()`'s whole roster by construction, which
/// `the_catalogue_panel_accounts_for_every_registered_operator` asserts — a group the row budget
/// dropped WHOLE used to vanish with no count at all, so the panel silently understated the
/// catalogue by however many sections did not fit (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️audit-user-journey-gaps-2026-09-13.md` §9 item 11).
pub struct CataloguePage {
    pub shown: usize,
    pub omitted: usize,
}

/// 🧮️ [`render`]'s own accounting, without building any UI — the law reads this rather than
/// re-deriving the page's arithmetic from a rendered tree it would have to parse.
pub fn catalogue_page() -> CataloguePage {
    let sections = semio_framework_os_flow::flow_palette_catalogue_sections();
    let budget = &mut PanelRowBudget::new(panel_page_rows());
    let mut shown = 0;
    let total = sections.len();
    for (index, section) in sections.iter().enumerate() {
        let before = budget.remaining();
        let _ = budget.nested(total.saturating_sub(index + 1), |share| paged_panel_section(&group_id(section), &section.items, CATALOGUE_GROUP_ROWS, share, |item, _| catalogue_row(item)));
        shown += before.saturating_sub(budget.remaining());
    }
    let roster = sections.iter().map(|section| section.items.len()).sum::<usize>();
    CataloguePage { shown, omitted: roster.saturating_sub(shown) }
}

fn group_id(section: &semio_framework_os_flow::CatalogueSection) -> String {
    format!("procedural-play-catalogue.{}", section.id)
}

pub fn render(labels: &Generation3dLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let sections = semio_framework_os_flow::flow_palette_catalogue_sections();
    let roster = sections.iter().map(|section| section.items.len()).sum::<usize>();
    let budget = &mut PanelRowBudget::new(panel_page_rows());
    let mut groups = UiFixedList::default();
    let total = sections.len();
    let mut shown = 0;
    for (index, section) in sections.iter().enumerate() {
        let group_id = group_id(section);
        let before = budget.remaining();
        let items = budget.nested(total.saturating_sub(index + 1), |share| paged_panel_section(&group_id, &section.items, CATALOGUE_GROUP_ROWS, share, |item, _| catalogue_row(item)))?;
        let placed = before.saturating_sub(budget.remaining());
        if items.is_empty() {
            continue;
        }
        let group = tree_group(&group_id, section.title.clone(), index == 0, items)?;
        if groups.try_push(group).is_err() {
            break;
        }
        shown += placed;
    }
    // 🛟️ The page ALWAYS states what it left out, whether the arena trimmed a group's tail or dropped
    // whole sections (or the whole listing — another panel in this process may already hold the
    // argument arena's credit, which `ui_value_headroom`'s own docstring calls a correct page). A
    // truncated group carries its own `+n`; this row carries the panel total, so `shown + omitted`
    // is the registered roster and the reader is never shown a silently short catalogue. Every
    // omitted operator is still reachable: `flow_app_catalogue` publishes the FULL roster on the
    // reserved `framework.section.catalogue` surface the canvas spotlight browses, which
    // `the_spotlight_catalogue_offers_every_operator_the_panel_omits` asserts.
    let omitted = roster.saturating_sub(shown);
    if omitted > 0 {
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
