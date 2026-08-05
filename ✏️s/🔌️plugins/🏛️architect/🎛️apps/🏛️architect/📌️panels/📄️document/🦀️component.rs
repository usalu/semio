//! 📄️ Architect document panel — program meta, per-register counts and the element list.

use crate::apps::architect::architect_action;
use crate::apps::architect::catalog::register_len;
use crate::apps::architect::chrome::{tree_item, tree_item_with_action, tree_node, tree_section};
use crate::apps::architect::config::{active_register, ArchitectConfig};
use crate::artifacts::program::engine::status_summary::status_summary;
use crate::artifacts::program::Program;
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL};
use serde_json::json;

//#region 🔖️Constants
pub const ARCHITECT_BODY_DOCUMENT: &str = "architect.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🏛️ Stitched into the app manifest by `crate::apps::architect::create_architect_app`.
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_DOCUMENT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(ARCHITECT_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(program: &Program, cfg: &ArchitectConfig) -> UiNode {
    let summary = status_summary(program);
    let element_items: Vec<UiTreeItemNode> = program
        .elements
        .iter()
        .map(|element| {
            tree_item_with_action(
                format!("architect-document.element.{}", element.header.id),
                format!("{} ({:?})", element.header.name, element.kind),
                Some(element.header.id.to_string()),
                architect_action("setSelection", Some(json!({ "ids": [element.header.id] }))),
            )
        })
        .collect();
    let register_items: Vec<UiTreeItemNode> = summary
        .by_register
        .iter()
        .map(|row| tree_item_with_action(format!("architect-document.register.{}", row.register), format!("{} ({})", row.register, row.count), None, architect_action("selectRegister", Some(json!({ "registerId": row.register })))))
        .collect();
    tree_node(
        vec![
            tree_section(
                "architect-document.meta",
                Some("Program".into()),
                vec![
                    tree_item("architect-document.meta.title", format!("Title: {}", program.meta.title)),
                    tree_item("architect-document.meta.project", format!("Project: {} ({})", program.project.client_name, program.project.code)),
                    tree_item("architect-document.meta.entities", format!("Entities tracked: {} (active register: {} / {})", summary.total_entities, active_register(cfg), register_len(program, active_register(cfg)))),
                ],
            ),
            tree_section("architect-document.registers", Some("Registers".into()), register_items),
            tree_section("architect-document.elements", Some("Elements".into()), if element_items.is_empty() { vec![tree_item("architect-document.elements.empty", "(none)")] } else { element_items }),
        ],
        Some(cfg.selected_ids.iter().map(|id| format!("architect-document.element.{id}")).collect()),
    )
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
        let json = serde_json::to_string(&render(&sample_plugin(), &ArchitectConfig::default())).expect("json");
        assert!(json.contains("Sample Clinic"));
        assert!(json.contains("architect-document.element."));
    }

    #[test]
    fn an_empty_program_renders_the_none_placeholder_row() {
        let json = serde_json::to_string(&render(&empty_plugin(), &ArchitectConfig::default())).expect("json");
        assert!(json.contains("architect-document.elements.empty"));
    }
}
//#endregion 🧪️Tests
