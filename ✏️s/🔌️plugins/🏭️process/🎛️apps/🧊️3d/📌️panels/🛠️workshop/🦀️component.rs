//! 🛠️ Process 3d play app panel — the workshop configurator: installed machines (select, remove) plus
//! one section per installed catalog (add if not yet installed).

use crate::apps::process3d::config::Process3dConfig;
use crate::apps::process3d::iconed_tree_item_with_action;
use crate::apps::process3d::process3d_action;
use crate::apps::process3d::terminology::Process3dLabels;
use crate::artifacts::process3d::engine::installed_catalogs;
use crate::artifacts::process3d::Process3dDocument;
use semio_framework_plugin::{tree_item_desc, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiPresence, UiTreeActionPlacement, UiTreeItemAction, UiTreeItemNode};
use serde_json::json;

//#region 🔖️Constants
pub const PROCESS_3D_PLAY_BODY_WORKSHOP: &str = "process.play.workshop";
const PROCESS_3D_PLAY_PANEL_WORKSHOP: &str = "workshop";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(PROCESS_3D_PLAY_PANEL_WORKSHOP.into()), label: LocalizedLabel::native("Workshop", "Werkstatt"), group: PanelGroup::Workbench, body_key: Some(PROCESS_3D_PLAY_BODY_WORKSHOP.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(fixture: &Process3dDocument, cfg: &Process3dConfig, labels: &Process3dLabels) -> UiNode {
    let mut builder = PanelTreeBuilder::new("process3d-play-workshop");
    let machine_items: Vec<UiTreeItemNode> = fixture
        .workshop
        .machines
        .iter()
        .map(|machine| {
            let target = format!("machine:{}", machine.id);
            UiTreeItemNode {
                icon_id: Some(machine.icon_id.as_str().into()),
                presence: UiPresence::selected(cfg.selected_id.as_deref() == Some(target.as_str())),
                action: Some(process3d_action("setSelection", Some(json!({ "id": target })))),
                actions: Some(vec![UiTreeItemAction {
                    icon_id: "trash".into(),
                    label: Some(labels.remove_machine.into()),
                    action: process3d_action("removeWorkshopMachine", Some(json!({ "id": machine.id }))),
                    placement: Some(UiTreeActionPlacement::Menu),
                }]),
                menu: None,
                ..UiTreeItemNode::base(format!("process3d-workshop.machine.{}", machine.id), Label::data(machine.label.clone()))
            }
        })
        .collect();
    builder = builder.section("process3d-play-workshop.machines", Some(labels.machines.into()), true, machine_items);
    for catalog in installed_catalogs() {
        let catalog_id = catalog.catalog_id();
        let items: Vec<UiTreeItemNode> = catalog
            .machines()
            .into_iter()
            .map(|machine| {
                let id = format!("process3d-workshop.catalog.{catalog_id}.{}", machine.id);
                let already_installed = fixture.workshop.machines.iter().any(|existing| existing.id == machine.id);
                if already_installed {
                    UiTreeItemNode { icon_id: Some(machine.icon_id.as_str().into()), dimmed: Some(true), menu: None, ..tree_item_desc(id, Label::data(machine.label.clone()), Some(labels.installed.as_str().to_string())) }
                } else {
                    iconed_tree_item_with_action(id, Label::data(machine.label.clone()), &machine.icon_id, process3d_action("addWorkshopMachine", Some(json!({ "catalogId": catalog_id, "machineId": machine.id }))))
                }
            })
            .collect();
        builder = builder.section(format!("process3d-play-workshop.catalog.{catalog_id}"), Some(Label::data(catalog.label())), false, items);
    }
    builder.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::process3d::commands::workshop::{add_workshop_machine, remove_workshop_machine};
    use crate::apps::process3d::panels::{catalogue, inspection};
    use crate::apps::process3d::testkit;
    use crate::apps::process3d::Process3dCommand;

    #[test]
    fn definition_binds_a_workshop_panel_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key.as_deref(), Some(PROCESS_3D_PLAY_BODY_WORKSHOP));
    }

    #[test]
    fn add_workshop_machine_action_installs_and_selects() {
        let mut app = testkit::app();
        let result = testkit::dispatch(&mut app, Process3dCommand::AddWorkshopMachine(add_workshop_machine::AddWorkshopMachine { catalog_id: "metal".into(), machine_id: "chopSaw".into() }));
        assert!(!result.mutations.is_empty(), "adding an uninstalled catalog machine must emit an operation");
        let document = app.projection().expect("projection");
        assert!(document.workshop.machines.iter().any(|machine| machine.id == "chopSaw"), "chopSaw should now be in the workshop");
        let rendered = testkit::render(&mut app, inspection::PROCESS_3D_PLAY_BODY_INSPECTION);
        assert!(!rendered.contains("No selection"), "expected the newly added machine to be selected: {rendered}");
    }

    #[test]
    fn add_workshop_machine_action_is_idempotent_when_already_installed() {
        let mut app = testkit::app();
        testkit::dispatch(&mut app, Process3dCommand::AddWorkshopMachine(add_workshop_machine::AddWorkshopMachine { catalog_id: "metal".into(), machine_id: "chopSaw".into() }));
        let count_after_first = app.projection().expect("projection").workshop.machines.len();
        let result = testkit::dispatch(&mut app, Process3dCommand::AddWorkshopMachine(add_workshop_machine::AddWorkshopMachine { catalog_id: "metal".into(), machine_id: "chopSaw".into() }));
        assert!(result.mutations.is_empty(), "adding an already-installed machine must be a no-op");
        assert_eq!(app.projection().expect("projection").workshop.machines.len(), count_after_first);
    }

    #[test]
    fn remove_workshop_machine_action_removes_and_clears_selection() {
        let mut app = testkit::app();
        testkit::dispatch(&mut app, Process3dCommand::AddWorkshopMachine(add_workshop_machine::AddWorkshopMachine { catalog_id: "metal".into(), machine_id: "chopSaw".into() }));
        let result = testkit::dispatch(&mut app, Process3dCommand::RemoveWorkshopMachine(remove_workshop_machine::RemoveWorkshopMachine { id: "chopSaw".into() }));
        assert!(!result.mutations.is_empty());
        let document = app.projection().expect("projection");
        assert!(!document.workshop.machines.iter().any(|machine| machine.id == "chopSaw"));
        let rendered = testkit::render(&mut app, inspection::PROCESS_3D_PLAY_BODY_INSPECTION);
        assert!(rendered.contains("No selection"), "removing the selected machine must clear the selection: {rendered}");
    }

    #[test]
    fn catalogue_reflects_workshop_after_machine_removal() {
        let mut app = testkit::app();
        let before = testkit::render(&mut app, catalogue::PROCESS_3D_PLAY_BODY_CATALOGUE);
        assert!(before.contains("Circular Saw"));
        testkit::dispatch(&mut app, Process3dCommand::RemoveWorkshopMachine(remove_workshop_machine::RemoveWorkshopMachine { id: "circularSaw".into() }));
        let after = testkit::render(&mut app, catalogue::PROCESS_3D_PLAY_BODY_CATALOGUE);
        assert!(!after.contains("Circular Saw"), "removed machine must disappear from the catalogue tree");
    }

    #[test]
    fn workshop_machine_parameter_edit_resizes_future_tool() {
        use crate::apps::process3d::commands::inspector::patch_inspector;
        use crate::apps::process3d::commands::step::add_step;
        use crate::artifacts::process3d::{ProcessMeasure, SolidSpec};
        let mut app = testkit::app();
        testkit::dispatch(&mut app, Process3dCommand::PatchInspector(patch_inspector::PatchInspector { target: "beam".into(), field: "height".into(), number: Some(0.05), text: None }));
        testkit::dispatch(&mut app, Process3dCommand::PatchInspector(patch_inspector::PatchInspector { target: "machine:circularSaw".into(), field: "crosscut.bladeDiameter".into(), number: Some(0.4), text: None }));
        let result = testkit::dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: None, machine_id: Some("circularSaw".into()), capability_id: Some("crosscut".into()), position: None }));
        assert!(!result.mutations.is_empty());
        let document = app.projection().expect("projection");
        let last = document.steps.last().expect("inserted step");
        let ProcessMeasure::Cut { tool: SolidSpec::Cylinder { radius, .. }, .. } = &last.measure else {
            panic!("expected a cylinder cut tool, got {:?}", last.measure);
        };
        assert!((radius - 0.2).abs() < 1e-9, "edited blade diameter 0.4 should size the next tool to radius 0.2, got {radius}");
    }
}
//#endregion 🧪️Tests
