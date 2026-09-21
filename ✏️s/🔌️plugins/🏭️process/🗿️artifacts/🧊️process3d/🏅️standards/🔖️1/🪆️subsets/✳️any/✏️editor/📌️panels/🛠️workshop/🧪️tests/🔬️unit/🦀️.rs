use super::*;
use crate::editor::process3d::commands::workshop::{add_workshop_machine, remove_workshop_machine};
use crate::editor::process3d::panels::catalogue;
use crate::editor::process3d::unit_tests::context;
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
    let mut app = context::app();
    let (_, receipt) = context::settled_dispatch(&mut app, Process3dCommand::AddWorkshopMachine(add_workshop_machine::AddWorkshopMachine { catalog_id: "metal".into(), machine_id: "chopSaw".into() }));
    assert!(context::published_a_document_mutation(&receipt), "adding an uninstalled catalog machine must emit an operation");
    let document = app.snapshot().expect("snapshot");
    assert!(document.workshop.machines.iter().any(|machine| machine.id == "chopSaw"), "chopSaw should now be in the workshop");
    let rendered = context::render(&mut app, PROCESS_3D_PLAY_BODY_WORKSHOP);
    assert!(rendered.contains("machine:chopSaw"), "expected the newly installed machine's canonical target id in the tree: {rendered}");
}

#[semio_framework_async_macros::async_test]
async fn add_workshop_machine_action_is_idempotent_when_already_installed() {
    let mut app = context::app();
    context::dispatch(&mut app, Process3dCommand::AddWorkshopMachine(add_workshop_machine::AddWorkshopMachine { catalog_id: "metal".into(), machine_id: "chopSaw".into() }));
    let count_after_first = app.snapshot().expect("snapshot").workshop.machines.len();
    let (_, receipt) = context::settled_dispatch(&mut app, Process3dCommand::AddWorkshopMachine(add_workshop_machine::AddWorkshopMachine { catalog_id: "metal".into(), machine_id: "chopSaw".into() }));
    assert!(!context::published_a_document_mutation(&receipt), "adding an already-installed machine must be a no-op");
    assert_eq!(app.snapshot().expect("snapshot").workshop.machines.len(), count_after_first);
}

#[semio_framework_async_macros::async_test]
async fn remove_workshop_machine_action_removes_the_machine() {
    let mut app = context::app();
    context::dispatch(&mut app, Process3dCommand::AddWorkshopMachine(add_workshop_machine::AddWorkshopMachine { catalog_id: "metal".into(), machine_id: "chopSaw".into() }));
    let (_, receipt) = context::settled_dispatch(&mut app, Process3dCommand::RemoveWorkshopMachine(remove_workshop_machine::RemoveWorkshopMachine { id: "chopSaw".into() }));
    assert!(context::published_a_document_mutation(&receipt), "removing an installed machine must reach the document lane");
    let document = app.snapshot().expect("snapshot");
    assert!(!document.workshop.machines.iter().any(|machine| machine.id == "chopSaw"));
    let rendered = context::render(&mut app, PROCESS_3D_PLAY_BODY_WORKSHOP);
    assert!(!rendered.contains("machine:chopSaw"), "removing the machine must drop its tree item: {rendered}");
}

#[semio_framework_async_macros::async_test]
async fn catalogue_reflects_workshop_after_machine_removal() {
    let mut app = context::app();
    let before = context::render(&mut app, catalogue::PROCESS_3D_PLAY_BODY_CATALOGUE);
    assert!(before.contains("Circular Saw"));
    context::dispatch(&mut app, Process3dCommand::RemoveWorkshopMachine(remove_workshop_machine::RemoveWorkshopMachine { id: "circularSaw".into() }));
    let after = context::render(&mut app, catalogue::PROCESS_3D_PLAY_BODY_CATALOGUE);
    assert!(!after.contains("Circular Saw"), "removed machine must disappear from the catalogue tree");
}

/// ⚖️ The real four-extension pack pushed through `setContributions` renders each catalog id ONCE —
/// the shipped `process-extension-*` bundles re-contribute the ids this build compiles in, and before
/// `installed_catalogs` deduped them the Werkstatt showed nine sections instead of five.
#[semio_framework_async_macros::async_test]
async fn the_real_extension_pack_renders_each_catalog_section_once() {
    use crate::editor::process3d::commands::contribution::set_contributions;
    let mut app = context::app();
    let pack = crate::editor::process3d::unit_tests::demonstrator_contributions_pack();
    let distilled = crate::editor::process3d::installable_contributions(&pack, crate::editor::process3d::PROCESS3D_CONFIG_CONTRIBUTIONS_BYTES);
    context::dispatch(&mut app, Process3dCommand::SetContributions(set_contributions::SetContributions { json: distilled }));
    let rendered = context::render(&mut app, PROCESS_3D_PLAY_BODY_WORKSHOP);
    let marker = "\"key\":\"process3d-play-workshop.catalog.";
    let mut rendered_ids: Vec<String> = Vec::new();
    for (index, _) in rendered.match_indices(marker) {
        let rest = &rendered[index + marker.len()..];
        rendered_ids.push(rest[..rest.find('"').expect("section key terminator")].to_string());
    }
    let mut unique = rendered_ids.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(rendered_ids.len(), unique.len(), "no catalog id may open two sections: {rendered_ids:?}");
    assert!(rendered_ids.iter().any(|id| id == "metal"), "the metal catalog still has machines left to add: {rendered_ids:?}");
    let builtin: Vec<String> = installed_catalogs("[]").iter().map(|catalog| catalog.catalog_id().to_string()).collect();
    assert!(rendered_ids.iter().all(|id| builtin.contains(id)), "the shipped packs re-contribute built-in ids, so nothing new may appear: {rendered_ids:?}");
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

//#region 🪟️WindowLaws
use semio_framework_plugin::{TreeWindowRequest, TreeWindows, ViewModel};

/// 🔎️ `(total, offset, row keys)` of one container, read off a projected body.
fn container_of(json: &str, key: &str) -> (u64, u64, Vec<String>) {
    fn walk(node: &serde_json::Value, key: &str) -> Option<serde_json::Value> {
        if node["key"].as_str() == Some(key) {
            return Some(node.clone());
        }
        node["children"].as_array().and_then(|children| children.iter().find_map(|child| walk(child, key)))
    }
    let projection: serde_json::Value = serde_json::from_str(json).expect("workshop projection json");
    let node = walk(&projection, key).unwrap_or_else(|| panic!("{key} is not in the rendered workshop: {json}"));
    let window = node["component"]["window"].as_object().unwrap_or_else(|| panic!("{key} stamps no window: {json}"));
    (
        window.get("total").and_then(serde_json::Value::as_u64).unwrap_or_default(),
        window.get("offset").and_then(serde_json::Value::as_u64).unwrap_or_default(),
        node["children"].as_array().cloned().unwrap_or_default().iter().map(|row| row["key"].as_str().unwrap_or_default().to_string()).collect(),
    )
}

/// 🌾️ A workshop an order of magnitude past any viewport: 60 installed machines.
fn oversized_workshop() -> crate::Process3dSnapshot {
    use crate::{Workshop, WorkshopMachine};
    let mut fixture = crate::empty_process3d_snapshot();
    fixture.workshop = Workshop {
        machines: (0..60)
            .map(|index| WorkshopMachine { id: format!("machine-{index:03}"), label: format!("Machine {index}"), icon_id: "scissors".into(), catalog_id: None, capabilities: Vec::new() })
            .collect(),
    };
    fixture
}

/// 🚚️ Reads through the retiring PROJECTION, never `serde_json::to_string` on a `BuiltNode`.
fn project(fixture: &crate::Process3dSnapshot, windows: &TreeWindows<'_>) -> String {
    let labels = crate::editor::process3d::terminology::process3d_labels(&ViewModel::default());
    let node = render(fixture, "[]", labels, windows).expect("workshop renders");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("workshop projection")
}

fn window_view(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> ViewModel {
    ViewModel { tree_windows: vec![TreeWindowRequest { body_key: PROCESS_3D_PLAY_BODY_WORKSHOP.into(), node_key: node_key.into(), open, offset, rows }], ..Default::default() }
}

/// ⚖️ LAW (a): the installed-machines section stamps its whole extent, materialises at most its
/// slice, and never summarises the remainder as a `+n` / `…more` continuation row.
#[test]
fn an_oversized_workshop_stamps_its_full_machine_count() {
    let json = project(&oversized_workshop(), &TreeWindows::unhosted());
    let (total, offset, rows) = container_of(&json, PROCESS_3D_PLAY_WORKSHOP_MACHINES);
    assert_eq!(total, 60, "the machines section stamps every installed machine: {json}");
    assert_eq!(offset, 0, "a first paint starts at zero");
    assert!(rows.len() <= 60, "the section materialises at most its slice: {json}");
    assert!(!json.contains(".more\""), "a windowed workshop has no continuation row: {json}");
    assert!(!json.contains(r#""label":"+"#), "a windowed workshop publishes no `+n` label: {json}");
}

/// ⚖️ LAW (b): a closed section states its extent and materialises nothing.
#[test]
fn a_closed_workshop_section_stamps_its_total_with_no_rows() {
    let view = window_view(PROCESS_3D_PLAY_WORKSHOP_MACHINES, Some(false), 0, 0);
    let json = project(&oversized_workshop(), &TreeWindows::for_body(&view, PROCESS_3D_PLAY_BODY_WORKSHOP));
    let (total, offset, rows) = container_of(&json, PROCESS_3D_PLAY_WORKSHOP_MACHINES);
    assert_eq!(total, 60, "a closed section still states its extent: {json}");
    assert_eq!(offset, 0);
    assert!(rows.is_empty(), "a closed section materialises nothing: {json}");
}

/// ⚖️ LAW (c): a host window request materialises exactly `[offset, offset + rows)`, keyed by the raw
/// canonical `"machine:{id}"` domain target id.
#[test]
fn a_workshop_window_request_materialises_exactly_its_slice() {
    let view = window_view(PROCESS_3D_PLAY_WORKSHOP_MACHINES, Some(true), 30, 5);
    let json = project(&oversized_workshop(), &TreeWindows::for_body(&view, PROCESS_3D_PLAY_BODY_WORKSHOP));
    let (total, offset, rows) = container_of(&json, PROCESS_3D_PLAY_WORKSHOP_MACHINES);
    assert_eq!(total, 60);
    assert_eq!(offset, 30, "the stamped offset is the requested one: {json}");
    let expected: Vec<String> = (30..35).map(|index| format!("machine:machine-{index:03}")).collect();
    assert_eq!(rows, expected, "exactly entries [30, 35) are materialised: {json}");
}

/// ⚖️ LAW (d): machine rows are domain pick targets — each declares its `granularity` and carries no
/// `Activate` binding of its own (its remove verb stays a row ACTION), while the tree carries exactly
/// one `interactionSelect`.
#[test]
fn workshop_machine_rows_declare_their_granularity_and_carry_no_activate_binding() {
    let view = window_view(PROCESS_3D_PLAY_WORKSHOP_MACHINES, Some(true), 0, 5);
    let json = project(&oversized_workshop(), &TreeWindows::for_body(&view, PROCESS_3D_PLAY_BODY_WORKSHOP));
    let projection: serde_json::Value = serde_json::from_str(&json).expect("workshop projection json");
    assert_eq!(projection["component"]["interactionDomain"].as_str(), Some(PROCESS3D_INTERACTION_DOMAIN), "{json}");
    assert_eq!(
        projection["bindings"].as_array().cloned().unwrap_or_default().iter().filter(|binding| binding["trigger"] == "activate").count(),
        1,
        "exactly one tree-level interactionSelect: {json}"
    );
    for section in projection["children"].as_array().cloned().unwrap_or_default() {
        if section["key"].as_str() != Some(PROCESS_3D_PLAY_WORKSHOP_MACHINES) {
            continue;
        }
        let rows = section["children"].as_array().cloned().unwrap_or_default();
        assert_eq!(rows.len(), 5, "the requested five rows: {json}");
        for row in rows {
            assert!(row["component"]["granularity"].as_str().is_some(), "a pick row declares its granularity: {row}");
            assert!(
                row["bindings"].as_array().cloned().unwrap_or_default().iter().all(|binding| binding["trigger"] != "activate"),
                "a pick row carries no activate binding of its own: {row}"
            );
        }
    }
}
//#endregion 🪟️WindowLaws
