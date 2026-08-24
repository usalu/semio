//! 🛠️ Process 3d play app panel — the workshop configurator: installed machines (select, remove) plus
//! one section per installed catalog (add if not yet installed).

use crate::artifacts::process3d::{MachineCatalog, Process3dSnapshot};
use crate::editor::process3d::iconed_tree_item_with_action;
use crate::editor::process3d::installed_catalogs;
use crate::editor::process3d::process3d_action;
use crate::editor::process3d::terminology::Process3dLabels;
use crate::editor::process3d::PROCESS3D_INTERACTION_DOMAIN;
use semio_framework_plugin::{tree_item_desc, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeActionPlacement, UiTreeItemAction, UiTreeItemNode};
use serde_json::json;

//#region 🔖️Constants
pub const PROCESS_3D_PLAY_BODY_WORKSHOP: &str = "process.play.workshop";
const PROCESS_3D_PLAY_PANEL_WORKSHOP: &str = "workshop";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(PROCESS_3D_PLAY_PANEL_WORKSHOP.into()),
        label: LocalizedLabel::native("Workshop", "Werkstatt"),
        group: PanelGroup::Workbench,
        body_key: Some(PROCESS_3D_PLAY_BODY_WORKSHOP.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): installed-machine item ids are
/// `"machine:{id}"` — the SAME canonical `"geometry"` domain target the old `selected_id` used for a
/// machine pick — so `.interaction_domain` binding stamps/prunes this section correctly; the catalog
/// sections stay un-bound (their items are install actions, not domain targets)?.
pub async fn render(fixture: &Process3dSnapshot, contributions_json: &str, labels: &Process3dLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut builder = PanelTreeBuilder::new("process3d-play-workshop")?;
    let machine_items: Vec<UiTreeItemNode> = fixture
        .workshop
        .machines
        .iter()
        .map(|machine| UiTreeItemNode {
            icon_id: Some(machine.icon_id.as_str().into()),
            actions: Some(vec![UiTreeItemAction {
                icon_id: "trash".into(),
                label: Some(labels.remove_machine.into()),
                action: process3d_action("removeWorkshopMachine", Some(json!({ "id": machine.id }))),
                placement: Some(UiTreeActionPlacement::Menu),
            }]),
            menu: None,
            ..UiTreeItemNode::base(format!("machine:{}", machine.id), Label::data(machine.label.clone()))
        })
        .collect();
    builder = builder.section("process3d-play-workshop.machines", Some(labels.machines.into()), true, machine_items)?.interaction_domain(PROCESS3D_INTERACTION_DOMAIN)?;
    for catalog in installed_catalogs(contributions_json) {
        let catalog_id = catalog.catalog_id();
        let items: Vec<UiTreeItemNode> = catalog
            .machines()
            .into_iter()
            .map(|machine| {
                let id = format!("process3d-workshop.catalog.{catalog_id}.{}", machine.id);
                let already_installed = fixture.workshop.machines.iter().any(|existing| existing.id == machine.id);
                if already_installed {
                    UiTreeItemNode { icon_id: Some(machine.icon_id.as_str().into()), dimmed: Some(true), menu: None, ..tree_item_desc(id, Label::data(machine.label.clone()), Some(labels.installed.as_str().to_string()))? }
                } else {
                    iconed_tree_item_with_action(id, Label::data(machine.label.clone()), &machine.icon_id, process3d_action("addWorkshopMachine", Some(json!({ "catalogId": catalog_id, "machineId": machine.id }))))?
                }
            })
            .collect();
        builder = builder.section(format!("process3d-play-workshop.catalog.{catalog_id}"), Some(Label::data(catalog.label())), false, items)?;
    }
    builder.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::process3d::commands::workshop::{add_workshop_machine, remove_workshop_machine};
    use crate::editor::process3d::panels::catalogue;
    use crate::editor::process3d::testkit;
    use crate::editor::process3d::Process3dCommand;

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_a_workshop_panel_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key.as_deref(), Some(PROCESS_3D_PLAY_BODY_WORKSHOP));
    }

    /// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): installing a machine no longer
    /// auto-selects it (selection is framework-owned now, unreachable from `Emit` — see this file's
    /// `render` doc comment); this asserts the still-real document mutation and its `"machine:{id}"`
    /// tree item.
    #[semio_framework_async_macros::async_test]
    async fn add_workshop_machine_action_installs() {
        let mut app = testkit::app();
        let result = testkit::dispatch(&mut app, Process3dCommand::AddWorkshopMachine(add_workshop_machine::AddWorkshopMachine { catalog_id: "metal".into(), machine_id: "chopSaw".into() }));
        assert!(!result.mutations.is_empty(), "adding an uninstalled catalog machine must emit an operation");
        let document = app.snapshot().expect("snapshot");
        assert!(document.workshop.machines.iter().any(|machine| machine.id == "chopSaw"), "chopSaw should now be in the workshop");
        let rendered = testkit::render(&mut app, PROCESS_3D_PLAY_BODY_WORKSHOP);
        assert!(rendered.contains("machine:chopSaw"), "expected the newly installed machine's canonical target id in the tree: {rendered}");
    }

    #[semio_framework_async_macros::async_test]
    async fn add_workshop_machine_action_is_idempotent_when_already_installed() {
        let mut app = testkit::app();
        testkit::dispatch(&mut app, Process3dCommand::AddWorkshopMachine(add_workshop_machine::AddWorkshopMachine { catalog_id: "metal".into(), machine_id: "chopSaw".into() }));
        let count_after_first = app.snapshot().expect("snapshot").workshop.machines.len();
        let result = testkit::dispatch(&mut app, Process3dCommand::AddWorkshopMachine(add_workshop_machine::AddWorkshopMachine { catalog_id: "metal".into(), machine_id: "chopSaw".into() }));
        assert!(result.mutations.is_empty(), "adding an already-installed machine must be a no-op");
        assert_eq!(app.snapshot().expect("snapshot").workshop.machines.len(), count_after_first);
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_workshop_machine_action_removes_the_machine() {
        let mut app = testkit::app();
        testkit::dispatch(&mut app, Process3dCommand::AddWorkshopMachine(add_workshop_machine::AddWorkshopMachine { catalog_id: "metal".into(), machine_id: "chopSaw".into() }));
        let result = testkit::dispatch(&mut app, Process3dCommand::RemoveWorkshopMachine(remove_workshop_machine::RemoveWorkshopMachine { id: "chopSaw".into() }));
        assert!(!result.mutations.is_empty());
        let document = app.snapshot().expect("snapshot");
        assert!(!document.workshop.machines.iter().any(|machine| machine.id == "chopSaw"));
        let rendered = testkit::render(&mut app, PROCESS_3D_PLAY_BODY_WORKSHOP);
        assert!(!rendered.contains("machine:chopSaw"), "removing the machine must drop its tree item: {rendered}");
    }

    #[semio_framework_async_macros::async_test]
    async fn catalogue_reflects_workshop_after_machine_removal() {
        let mut app = testkit::app();
        let before = testkit::render(&mut app, catalogue::PROCESS_3D_PLAY_BODY_CATALOGUE);
        assert!(before.contains("Circular Saw"));
        testkit::dispatch(&mut app, Process3dCommand::RemoveWorkshopMachine(remove_workshop_machine::RemoveWorkshopMachine { id: "circularSaw".into() }));
        let after = testkit::render(&mut app, catalogue::PROCESS_3D_PLAY_BODY_CATALOGUE);
        assert!(!after.contains("Circular Saw"), "removed machine must disappear from the catalogue tree");
    }

    /// 🌉️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4: `AddStep` dispatches a
    /// `CreateStep` mutation, which is a documented no-op now (`steps` composes an
    /// `s.stdio.semio.flow` CHILD HANDLE — no resolver, see `ProcessWorkingScene`'s doc comment),
    /// so the end-to-end "edit a machine parameter, then add a step, then read the sized tool back
    /// off the document" path no longer has a real document to read from. The real, unaffected part
    /// of this test — `measure_for_capability` sizing a cut tool from a capability's own edited
    /// parameter — is asserted directly instead.
    #[semio_framework_async_macros::async_test]
    async fn workshop_machine_parameter_edit_sizes_the_capability_measure() {
        use crate::artifacts::process3d::schema::inferences::measure_for_capability;
        use crate::artifacts::process3d::{Capability, MeasureRecipe, ProcessMeasure, WorkingSolid};
        let capability = Capability {
            id: "crosscut".into(),
            label: "Crosscut".into(),
            icon_id: "scissors".into(),
            recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
            parameters: vec![
                crate::artifacts::process3d::CapabilityParameter { id: "bladeDiameter".into(), label: "Blade Diameter".into(), value: 0.4 },
                crate::artifacts::process3d::CapabilityParameter { id: "kerf".into(), label: "Kerf".into(), value: 0.002 },
            ],
            rules: Vec::new(),
        };
        let measure = measure_for_capability(&capability, None);
        let ProcessMeasure::Cut { tool: WorkingSolid::Cylinder { radius, .. }, .. } = measure else {
            panic!("expected a cylinder cut tool, got {measure:?}");
        };
        assert!((radius - 0.2).abs() < 1e-9, "edited blade diameter 0.4 should size the tool to radius 0.2, got {radius}");
    }
}
//#endregion 🧪️Tests
