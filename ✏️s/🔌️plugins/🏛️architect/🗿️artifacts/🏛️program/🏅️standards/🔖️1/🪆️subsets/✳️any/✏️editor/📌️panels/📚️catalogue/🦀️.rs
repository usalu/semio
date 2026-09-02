//! 📚️ Architect catalogue panel — the action shortcuts and the register index.

use crate::editor::architect::catalog::REGISTER_IDS;
use crate::editor::architect::{architect_action, ui_value_map, ui_value_text};
use semio_framework_plugin::{
    tree_item_with_action, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiFixedList, UiValue, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
};

//#region 🔖️Constants
pub const ARCHITECT_BODY_CATALOGUE: &str = "architect.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🏛️ Stitched into the app manifest by `crate::editor::architect::create_architect_app`.
pub async fn definition() -> PanelTabDefinition {
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
pub async fn render() -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut register_items = UiFixedList::default();
    for register in REGISTER_IDS {
        let args = ui_value_map([("registerId", ui_value_text(register)?)])?;
        let item = tree_item_with_action(format!("architect-catalogue.register.{register}"), Label::data(*register), None, architect_action("selectRegister", Some(args))?)?;
        register_items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "architect catalogue register admission failed"))?;
    }
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
        let item = tree_item_with_action(id, Label::data(label), None, architect_action(action, args)?)?;
        actions.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "architect catalogue action admission failed"))?;
    }
    PanelTreeBuilder::new("architect-catalogue")?
        .section("architect-catalogue.actions", Some(Label::data(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL)), true, actions)?
        .section("architect-catalogue.registers", Some(Label::data("Registers")), true, register_items)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn the_tab_is_the_framework_catalogue_tab_bound_to_this_apps_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key.as_deref(), Some(ARCHITECT_BODY_CATALOGUE));
        assert!(matches!(definition.group, PanelGroup::Workbench));
    }

    #[semio_framework_async_macros::async_test]
    async fn every_register_id_gets_a_catalogue_row() {
        let json = serde_json::to_string(&render()).expect("json");
        for register in REGISTER_IDS {
            assert!(json.contains(&format!("architect-catalogue.register.{register}")), "missing catalogue row for {register}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn the_action_shortcuts_are_present() {
        let json = serde_json::to_string(&render()).expect("json");
        for id in ["architect-catalogue.validate", "architect-catalogue.analysis", "architect-catalogue.report", "architect-catalogue.search"] {
            assert!(json.contains(id), "missing shortcut {id}");
        }
    }
}
//#endregion 🧪️Tests
