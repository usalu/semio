//! 📚️ Architect catalogue panel — the action shortcuts and the register index.

use crate::editor::architect::architect_action;
use crate::editor::architect::catalog::REGISTER_IDS;
use crate::editor::architect::chrome::{tree_item_with_action, tree_node, tree_section};
use crate::artifacts::program::ProgramSnapshot;
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};
use serde_json::json;

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
pub fn render() -> UiNode {
    let register_items: Vec<UiTreeItemNode> =
        REGISTER_IDS.iter().map(|register| tree_item_with_action(format!("architect-catalogue.register.{register}"), *register, None, architect_action("selectRegister", Some(json!({ "registerId": register }))))).collect();
    tree_node(vec![
        tree_section(
            "architect-catalogue.actions",
            Some(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL.into()),
            vec![
                tree_item_with_action("architect-catalogue.add-item", "Add Register Item", None, architect_action("addRegisterItem", Some(json!({ "registerId": "elements", "template": null })))),
                tree_item_with_action("architect-catalogue.validate", "Run Validation", None, architect_action("runValidation", None)),
                tree_item_with_action("architect-catalogue.analysis", "Run Analysis", None, architect_action("runAnalysis", Some(json!({ "analysisKind": "gap" })))),
                tree_item_with_action("architect-catalogue.report", "Run Report", None, architect_action("runReport", Some(json!({ "reportKind": "executiveSummary" })))),
                tree_item_with_action("architect-catalogue.export", "Export ProgramSnapshot", None, architect_action("exportProgram", None)),
                tree_item_with_action("architect-catalogue.import", "Import ProgramSnapshot", None, architect_action("importProgramRequest", None)),
                tree_item_with_action("architect-catalogue.export-csv", "Export Registers CSV", None, architect_action("exportRegistersCsv", None)),
                tree_item_with_action("architect-catalogue.import-csv", "Import Registers CSV", None, architect_action("importRegistersCsv", Some(json!({ "csv": "", "strategy": "upsert" })))),
                tree_item_with_action("architect-catalogue.apply-template", "Apply Template", None, architect_action("applyTemplate", Some(json!({ "templateId": "" })))),
                tree_item_with_action("architect-catalogue.search", "Search ProgramSnapshot", None, architect_action("search", Some(json!({ "query": "" })))),
            ],
        ),
        tree_section("architect-catalogue.registers", Some("Registers".into()), register_items),
    ])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_tab_is_the_framework_catalogue_tab_bound_to_this_apps_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key.as_deref(), Some(ARCHITECT_BODY_CATALOGUE));
        assert!(matches!(definition.group, PanelGroup::Workbench));
    }

    #[test]
    fn every_register_id_gets_a_catalogue_row() {
        let json = serde_json::to_string(&render()).expect("json");
        for register in REGISTER_IDS {
            assert!(json.contains(&format!("architect-catalogue.register.{register}")), "missing catalogue row for {register}");
        }
    }

    #[test]
    fn the_action_shortcuts_are_present() {
        let json = serde_json::to_string(&render()).expect("json");
        for id in ["architect-catalogue.validate", "architect-catalogue.analysis", "architect-catalogue.report", "architect-catalogue.search"] {
            assert!(json.contains(id), "missing shortcut {id}");
        }
    }
}
//#endregion 🧪️Tests
