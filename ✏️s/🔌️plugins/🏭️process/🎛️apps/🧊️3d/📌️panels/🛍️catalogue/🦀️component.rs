//! 🛍️ Process 3d play app panel — the workshop capability catalogue plus quick-swap stock kinds.

use crate::apps::process3d::iconed_tree_item_with_action;
use crate::apps::process3d::process3d_action;
use crate::apps::process3d::terminology::Process3dLabels;
use crate::artifacts::process3d::engine::{installed_catalogs, validate_capability, validation_context_for_stock, validation_reason};
use crate::artifacts::process3d::{Process3dDocument, WorkshopMachine};
use semio_framework_plugin::{tree_item_desc, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};
use serde_json::json;

//#region 🔖️Constants
pub const PROCESS_3D_PLAY_BODY_CATALOGUE: &str = "process.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(PROCESS_3D_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🏷️ Display label for a catalog id, resolved against `installed_catalogs()` — falls back to the raw
/// id if the catalog that seeded a workshop machine was since uninstalled (never resolved back, per
/// `WorkshopMachine::catalog_id`'s informational-only contract).
fn catalog_label(catalog_id: &str) -> String {
    installed_catalogs().into_iter().find(|catalog| catalog.catalog_id() == catalog_id).map_or_else(|| catalog_id.to_string(), |catalog| catalog.label().to_string())
}

/// 🏭️ Builds one catalogue tree item per workshop machine capability, grouped by the machine's source
/// catalog (uncataloged/generic machines first, open by default), disabling (non-clickable, with a
/// reason) any capability the current stock doesn't satisfy.
pub fn render(fixture: &Process3dDocument, labels: &Process3dLabels) -> UiNode {
    let ctx = validation_context_for_stock(&fixture.stock);
    let mut builder = PanelTreeBuilder::new("process3d-play-catalogue");
    let mut sections: Vec<(Option<&str>, Vec<&WorkshopMachine>)> = Vec::new();
    for machine in &fixture.workshop.machines {
        let key = machine.catalog_id.as_deref();
        match sections.iter_mut().find(|(existing, _)| *existing == key) {
            Some(section) => section.1.push(machine),
            None => sections.push((key, vec![machine])),
        }
    }
    sections.sort_by_key(|(key, _)| key.is_some());
    for (catalog_id, machines) in sections {
        let items: Vec<UiTreeItemNode> = machines
            .iter()
            .flat_map(|machine| {
                machine.capabilities.iter().map(move |capability| {
                    let failures = validate_capability(capability, &ctx);
                    let id = format!("process3d-catalogue.{}.{}", machine.id, capability.id);
                    let label = Label::data(format!("{} — {}", machine.label, capability.label));
                    if failures.is_empty() {
                        iconed_tree_item_with_action(id, label, &capability.icon_id, process3d_action("addStep", Some(json!({ "machineId": machine.id, "capabilityId": capability.id }))))
                    } else {
                        UiTreeItemNode { icon_id: Some(capability.icon_id.as_str().into()), menu: None, ..tree_item_desc(id, label, Some(validation_reason(&failures))) }
                    }
                })
            })
            .collect();
        let section_id = format!("process3d-play-catalogue.{}", catalog_id.unwrap_or("workshop"));
        let section_label = catalog_id.map_or_else(|| labels.workshop.into(), |id| Label::data(catalog_label(id)));
        builder = builder.section(section_id, Some(section_label), catalog_id.is_none(), items);
    }
    let stock_items = vec![
        iconed_tree_item_with_action("process3d-catalogue.stock-box", labels.stock_kind_box, "box", process3d_action("setStock", Some(json!({ "kind": "box" })))),
        iconed_tree_item_with_action("process3d-catalogue.stock-cylinder", labels.stock_kind_cylinder, "cylinder", process3d_action("setStock", Some(json!({ "kind": "cylinder" })))),
        iconed_tree_item_with_action("process3d-catalogue.stock-sphere", labels.stock_kind_sphere, "circle", process3d_action("setStock", Some(json!({ "kind": "sphere" })))),
        iconed_tree_item_with_action("process3d-catalogue.stock-import", labels.import_model, "folder-open", process3d_action("loadModelRequest", None)),
    ];
    builder.section("process3d-play-catalogue.stock", Some(labels.stock.into()), false, stock_items).build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::process3d::testkit;

    /// 🪵️ The default timber beam (0.24m tall) exceeds both the circular saw's 0.065m max cut depth
    /// and the table saw's 0.102m — both wood machines list, both are disabled with a reason.
    #[test]
    fn catalogue_lists_workshop_wood_machines_with_mixed_validity_on_default_stock() {
        let mut app = testkit::app();
        let rendered = testkit::render(&mut app, PROCESS_3D_PLAY_BODY_CATALOGUE);
        assert!(rendered.contains("Circular Saw"), "expected wood's circular saw in the catalogue: {rendered}");
        assert!(rendered.contains("Table Saw"), "expected wood's table saw in the catalogue: {rendered}");
        assert!(rendered.contains("needs stock"), "expected at least one disabled-item validation reason: {rendered}");
    }

    #[test]
    fn definition_binds_the_framework_catalogue_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_CATALOGUE_ID);
        assert_eq!(definition.body_key.as_deref(), Some(PROCESS_3D_PLAY_BODY_CATALOGUE));
    }
}
//#endregion 🧪️Tests
