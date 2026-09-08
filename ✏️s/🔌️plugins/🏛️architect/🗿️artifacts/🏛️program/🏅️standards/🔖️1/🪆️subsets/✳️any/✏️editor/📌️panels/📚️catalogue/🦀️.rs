//! 📚️ Architect catalogue panel — the action shortcuts and the register index.

use crate::editor::architect::catalog::REGISTER_IDS;
use crate::editor::architect::{architect_action, ui_value_map, ui_value_text};
use crate::editor::architect::ui_label;
use semio_framework_plugin::{
    tree_item_with_action,  LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiFixedList, UiValue, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
};

//#region 🔖️Constants
pub const ARCHITECT_BODY_CATALOGUE: &str = "architect.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🏛️ Stitched into the app manifest by `crate::editor::architect::create_architect_app`.
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(ARCHITECT_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render() -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let specs = [
        ("architect-catalogue.add-item", "Add Register Item", "addRegisterItem", Some(ui_value_map([("registerId", ui_value_text("elements")?), ("template", UiValue::Null)])?)),
        ("architect-catalogue.validate", "Run Validation", "runValidation", None),
        ("architect-catalogue.analysis", "Run Analysis", "runAnalysis", Some(ui_value_map([("analysisKind", ui_value_text("gap")?)])?)),
        ("architect-catalogue.report", "Run Report", "runReport", Some(ui_value_map([("reportKind", ui_value_text("executiveSummary")?)])?)),
        ("architect-catalogue.export", "Export ProgramSnapshot", "exportProgram", None),
        ("architect-catalogue.import", "Import ProgramSnapshot", "importProgramRequest", None),
        ("architect-catalogue.export-csv", "Export Registers CSV", "exportRegistersCsv", None),
        ("architect-catalogue.import-csv", "Import Registers CSV", "importRegistersCsv", Some(ui_value_map([("csv", ui_value_text("")?), ("strategy", ui_value_text("upsert")?)])?)),
        ("architect-catalogue.apply-template", "Apply Template", "applyTemplate", Some(ui_value_map([("templateId", ui_value_text("")?)])?)),
        ("architect-catalogue.search", "Search ProgramSnapshot", "search", Some(ui_value_map([("query", ui_value_text("")?)])?)),
    ];
    let mut actions = UiFixedList::default();
    for (id, label, action, args) in specs {
        let item = tree_item_with_action(id, ui_label(label)?, None, architect_action(action, args)?)?;
        actions.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "architect catalogue action admission failed"))?;
    }
    let mut tree = PanelTreeBuilder::new("architect-catalogue")?
        .section("architect-catalogue.actions", Some(ui_label(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL)?), true, actions)?;
    for (page, registers) in REGISTER_IDS.chunks(semio_framework_ui_contract::UI_FIXED_LIST_ITEMS).enumerate() {
        let mut items = UiFixedList::default();
        for register in registers {
            let args = ui_value_map([("registerId", ui_value_text(register)?)])?;
            let item = tree_item_with_action(format!("architect-catalogue.register.{register}"), ui_label(*register)?, None, architect_action("selectRegister", Some(args))?)?;
            items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "architect catalogue register admission failed"))?;
        }
        let start = page * semio_framework_ui_contract::UI_FIXED_LIST_ITEMS + 1;
        let end = start + registers.len() - 1;
        tree = tree.section(format!("architect-catalogue.registers.{page}"), Some(ui_label(format!("Registers {start}–{end}"))?), true, items)?;
    }
    tree.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
