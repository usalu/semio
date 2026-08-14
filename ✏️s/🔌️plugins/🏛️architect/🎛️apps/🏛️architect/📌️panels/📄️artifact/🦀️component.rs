//! 📄️ Architect document panel — program meta, per-register counts and the element list.

use crate::apps::architect::architect_action;
use crate::apps::architect::catalog::register_len;
use crate::apps::architect::chrome::{tree_item, tree_item_with_action};
use crate::apps::architect::config::{active_register, ArchitectConfig};
use crate::apps::architect::ARCHITECT_INTERACTION_PROGRAM;
use crate::artifacts::program::standards::v1::subsets::any::schema::inferences::status_summary;
use crate::artifacts::program::ProgramSnapshot;
use semio_framework_plugin::{tree_item_desc, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};
use serde_json::json;

//#region 🔖️Constants
pub const ARCHITECT_BODY_DOCUMENT: &str = "architect.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🏛️ Stitched into the app manifest by `crate::apps::architect::create_architect_app`.
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
/// `.interaction_domain(...)`).
pub fn render(program: &ProgramSnapshot, cfg: &ArchitectConfig) -> UiNode {
    let summary = status_summary(program);
    let element_items: Vec<UiTreeItemNode> = program.elements.iter().map(|element| tree_item_desc(element.header.id.to_string(), Label::data(format!("{} ({:?})", element.header.name, element.kind)), Some(element.header.id.to_string()))).collect();
    let register_items: Vec<UiTreeItemNode> = summary
        .by_register
        .iter()
        .map(|row| tree_item_with_action(format!("architect-document.register.{}", row.register), format!("{} ({})", row.register, row.count), None, architect_action("selectRegister", Some(json!({ "registerId": row.register })))))
        .collect();
    PanelTreeBuilder::new("architect-document")
        .section(
            "architect-document.meta",
            Some(Label::data("ProgramSnapshot")),
            true,
            vec![
                tree_item("architect-document.meta.title", format!("Title: {}", program.meta.title)),
                tree_item("architect-document.meta.project", format!("Project: {} ({})", program.project.client_name, program.project.code)),
                tree_item("architect-document.meta.entities", format!("Entities tracked: {} (active register: {} / {})", summary.total_entities, active_register(cfg), register_len(program, active_register(cfg)))),
            ],
        )
        .section("architect-document.registers", Some(Label::data("Registers")), true, register_items)
        .section_or_placeholder("architect-document.elements", Some(Label::data("Elements")), true, element_items, Label::data("(none)"))
        .interaction_domain(ARCHITECT_INTERACTION_PROGRAM)
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::{empty_plugin, sample_plugin};

    #[test]
    fn the_tab_is_the_framework_document_tab_bound_to_this_apps_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key.as_deref(), Some(ARCHITECT_BODY_DOCUMENT));
        assert!(matches!(definition.group, PanelGroup::Workbench));
    }

    #[test]
    fn the_tree_lists_program_meta_and_the_elements() {
        let program = sample_plugin();
        let json = serde_json::to_string(&render(&program, &ArchitectConfig::default())).expect("json");
        assert!(json.contains("Sample Clinic"));
        assert!(json.contains(&program.elements[0].header.id.to_string()));
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the document panel binds the
    /// "program" interaction domain — the framework auto-injects/stamps selection over its bare
    /// element rows (see `render`'s own doc comment).
    #[test]
    fn the_tree_binds_the_program_interaction_domain() {
        let json = serde_json::to_string(&render(&sample_plugin(), &ArchitectConfig::default())).expect("json");
        assert!(json.contains("\"interactionDomain\":\"program\""));
    }

    #[test]
    fn an_empty_program_renders_the_none_placeholder_row() {
        let json = serde_json::to_string(&render(&empty_plugin(), &ArchitectConfig::default())).expect("json");
        assert!(json.contains("architect-document.elements.empty"));
    }
}
//#endregion 🧪️Tests
