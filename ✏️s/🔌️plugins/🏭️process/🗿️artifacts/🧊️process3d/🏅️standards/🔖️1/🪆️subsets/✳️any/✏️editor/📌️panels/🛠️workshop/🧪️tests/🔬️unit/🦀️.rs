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

/// 🔧 `measure_for_capability` sizing a cut tool from a capability's own edited parameter,
/// asserted directly (the full "edit a machine parameter, then AddStep, then read the sized
/// tool back off the document" path is covered end to end by the `🎮️commands/🪜️step` tests).
#[semio_framework_async_macros::async_test]
async fn workshop_machine_parameter_edit_sizes_the_capability_measure() {
    use crate::schema::inferences::measure_for_capability;
    use crate::{Capability, MeasureRecipe, ProcessMeasure, WorkingSolid};
    let capability = Capability {
        id: "crosscut".into(),
        label: "Crosscut".into(),
        icon_id: "scissors".into(),
        recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
        parameters: vec![crate::CapabilityParameter { id: "bladeDiameter".into(), label: "Blade Diameter".into(), value: 0.4 }, crate::CapabilityParameter { id: "kerf".into(), label: "Kerf".into(), value: 0.002 }],
        rules: Vec::new(),
    };
    let measure = measure_for_capability(&capability, None);
    let ProcessMeasure::Cut { tool: WorkingSolid::Cylinder { radius, .. }, .. } = measure else {
        panic!("expected a cylinder cut tool, got {measure:?}");
    };
    assert!((radius - 0.2).abs() < 1e-9, "edited blade diameter 0.4 should size the tool to radius 0.2, got {radius}");
}
