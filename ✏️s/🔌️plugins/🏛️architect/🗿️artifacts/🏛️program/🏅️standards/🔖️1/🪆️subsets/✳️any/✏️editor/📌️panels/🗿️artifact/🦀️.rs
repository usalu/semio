//! 📄️ Architect document panel — program meta, per-register counts and the element list.

use crate::editor::architect::ui_label;
use crate::editor::architect::ARCHITECT_INTERACTION_PROGRAM;
use crate::editor::architect::{architect_action, ui_value_map, ui_value_text};
use crate::standards::v1::subsets::any::schema::inferences::status_summary;
use crate::ProgramSnapshot;
use semio_framework_plugin::{
    tree_item_desc, tree_item_with_action, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiFixedList, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
};

//#region 🔖️Constants
pub const ARCHITECT_BODY_DOCUMENT: &str = "architect.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🏛️ Stitched into the app manifest by `crate::editor::architect::create_architect_app`.
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(ARCHITECT_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: element rows are the "program"
/// interaction domain's sole real pick surface — bare items (no `.action`) whose id IS the
/// `InteractionTarget` id (`EntityId`s are already globally unique, unlike note's nested block ids,
/// so no row-id prefix/mapping is needed); clicks translate into the framework's injected
/// `interactionSelect` generically. Register rows keep their own `selectRegister` action (switching
/// the active register is unrelated to entity selection) and sit in the SAME tree, unaffected —
/// mirrors note's document panel (`action_rows` + bare `block_items` coexisting under one
/// `.interaction_domain(...)?`).
pub fn render(program: &ProgramSnapshot) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let summary = status_summary(program);
    let mut element_items = UiFixedList::default();
    for element in &program.elements {
        let item = tree_item_desc(element.header.id.to_string(), ui_label(format!("{} ({:?})", element.header.name, element.kind))?, Some(element.header.id.to_string()))?;
        element_items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "architect element row admission failed"))?;
    }
    let mut meta = UiFixedList::default();
    for item in [
        tree_item_desc("architect-document.meta.title", ui_label(format!("Title: {}", program.meta.title))?, None)?,
        tree_item_desc("architect-document.meta.project", ui_label(format!("Project: {} ({})", program.project.client_name, program.project.code))?, None)?,
        tree_item_desc("architect-document.meta.entities", ui_label(format!("Entities tracked: {}", summary.total_entities))?, None)?,
    ] {
        meta.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "architect metadata row admission failed"))?;
    }
    let mut tree = PanelTreeBuilder::new("architect-document")?.section("architect-document.meta", Some(ui_label("ProgramSnapshot")?), true, meta)?;
    for (page, registers) in summary.by_register.chunks(semio_framework_ui_contract::UI_FIXED_LIST_ITEMS).enumerate() {
        let mut items = UiFixedList::default();
        for row in registers {
            let args = ui_value_map([("registerId", ui_value_text(&row.register)?)])?;
            let item = tree_item_with_action(format!("architect-document.register.{}", row.register), ui_label(format!("{} ({})", row.register, row.count))?, None, architect_action("selectRegister", Some(args))?)?;
            items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "architect register row admission failed"))?;
        }
        let start = page * semio_framework_ui_contract::UI_FIXED_LIST_ITEMS + 1;
        let end = start + registers.len() - 1;
        tree = tree.section(format!("architect-document.registers.{page}"), Some(ui_label(format!("Registers {start}–{end}"))?), true, items)?;
    }
    tree.section_or_placeholder("architect-document.elements", Some(ui_label("Elements")?), true, element_items, ui_label("(none)")?)?.interaction_domain(ARCHITECT_INTERACTION_PROGRAM)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
