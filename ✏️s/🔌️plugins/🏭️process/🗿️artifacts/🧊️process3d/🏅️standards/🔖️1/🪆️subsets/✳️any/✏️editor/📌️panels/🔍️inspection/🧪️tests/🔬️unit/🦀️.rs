use super::*;
use crate::editor::process3d::commands::step::add_step;
use crate::editor::process3d::unit_tests::context;
use crate::editor::process3d::{Process3dCommand, PROCESS3D_INTERACTION_DOMAIN};
use semio_framework::DslValue;

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
    assert_eq!(definition.body_key.as_deref(), Some(PROCESS_3D_PLAY_BODY_INSPECTION));
}

//#region 🔖️AddStepDispatch
/// 🌉️ `AddStep` dispatches a real `CreateStep` mutation against `step_payloads`.
/// `add_step::handle`'s own capability-dimension VALIDATION gate is a documented gap (no
/// resolvable stock extent — see its own doc comment), so every resolvable machine/capability
/// pair succeeds unconditionally.
#[semio_framework_async_macros::async_test]
async fn add_step_dispatches_its_mutation() {
    let mut app = context::app();
    let (_, receipt) = context::settled_dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: Some("drill".into()), machine_id: None, capability_id: None, position: None }));
    assert!(context::published_a_document_mutation(&receipt), "AddStep must dispatch its CreateStep mutation");
}

/// 🌉️ Same documented gap as above, from the catalogue-routed (machine/capability-addressed)
/// entry point: even a stock the pre-migration code would have rejected (circular saw needs
/// height ≤ 0.065m; the default timber beam is 0.24m) now succeeds, since the dimension gate
/// can no longer read real stock extents.
#[semio_framework_async_macros::async_test]
async fn add_step_via_catalogue_no_longer_gates_on_stock_dimensions() {
    let mut app = context::app();
    let (_, receipt) = context::settled_dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: None, machine_id: Some("circularSaw".into()), capability_id: Some("crosscut".into()), position: None }));
    assert!(context::published_a_document_mutation(&receipt), "documented gap: the dimension-validation gate can no longer reject an oversized stock");
}

#[semio_framework_async_macros::async_test]
async fn measure_arg_routes_to_generic_machine_and_dispatches() {
    let mut app = context::app();
    let (_, receipt) = context::settled_dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: Some("cut".into()), machine_id: None, capability_id: None, position: None }));
    assert!(context::published_a_document_mutation(&receipt), "a generic-machine measure arg must reach the document lane");
}
//#endregion 🔖️AddStepDispatch

//#region 🔖️SelectionInspector
fn select(app: &mut context::Process3dApp, id: &str) {
    let targets = serde_json::to_string(&vec![protocol::InteractionTarget { granularity: "object".into(), id: id.into() }]).expect("targets serialize");
    let args = DslValue::Object(vec![
        ("domainId".to_string(), DslValue::String(PROCESS3D_INTERACTION_DOMAIN.into())),
        ("targets".to_string(), DslValue::String(targets)),
        ("merge".to_string(), DslValue::String("replace".into())),
        ("method".to_string(), DslValue::String("pick".into())),
    ]);
    context::action(app, semio_framework_plugin::INTERACTION_SELECT_ACTION_ID, Some(&args));
}

#[semio_framework_async_macros::async_test]
async fn empty_selection_still_renders_the_empty_state() {
    let mut app = context::app_with_registry();
    let rendered = context::render(&mut app, PROCESS_3D_PLAY_BODY_INSPECTION);
    assert!(rendered.contains("process3d-play-inspector.empty"));
}

#[semio_framework_async_macros::async_test]
async fn selected_stock_id_renders_its_dimensions() {
    let mut app = context::app_with_registry();
    let snapshot = app.snapshot().expect("snapshot");
    let stock_id = snapshot.stock_id.clone();
    let stock = snapshot.stock_payload.clone();
    // 📏️ The dimensions come from the seeded document, never from a literal: the curated default
    // example is a timber beam (3 × 0.2 × 0.3), not the 1 × 1 × 1 box this law used to assume, and
    // `push_working_solid_fields` prints exactly this `WorkingSolid`'s own extents.
    let crate::WorkingSolid::Box { width, height, .. } = &stock.solid else { panic!("the seeded example's stock is a box: {:?}", stock.solid) };
    select(&mut app, &stock_id);
    let rendered = context::render(&mut app, PROCESS_3D_PLAY_BODY_INSPECTION);
    assert!(rendered.contains("process3d-play-inspector.stock"), "expected the stock section: {rendered}");
    assert!(rendered.contains(&format!("Width: {width}")), "expected the seeded stock's width {width}: {rendered}");
    assert!(rendered.contains(&format!("Height: {height}")), "expected the seeded stock's height {height}: {rendered}");
}

#[semio_framework_async_macros::async_test]
async fn selected_step_id_renders_its_label_and_measure_kind() {
    let mut app = context::app_with_registry();
    context::dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: Some("drill".into()), machine_id: None, capability_id: None, position: None }));
    let step = app.snapshot().expect("snapshot").step_payloads.last().expect("added step").clone();
    select(&mut app, &step.id);
    let rendered = context::render(&mut app, PROCESS_3D_PLAY_BODY_INSPECTION);
    assert!(rendered.contains("process3d-play-inspector.step"), "expected the step section: {rendered}");
    assert!(rendered.contains(&step.label), "expected the step's label {}: {rendered}", step.label);
    assert!(rendered.contains("Kind: Drill"), "expected the measure kind: {rendered}");
    assert!(rendered.contains("Radius: 0.05"), "expected the drill capability's own radius parameter: {rendered}");
}

/// 🪟️ A capability section authors itself CLOSED, so it states its parameter count and materialises
/// nothing until the reader opens it — the machine summary above it is open and carries its fields.
#[semio_framework_async_macros::async_test]
async fn selected_machine_id_renders_its_capabilities() {
    let mut app = context::app_with_registry();
    select(&mut app, "machine:saw");
    let rendered = context::render(&mut app, PROCESS_3D_PLAY_BODY_INSPECTION);
    assert!(rendered.contains("process3d-play-inspector.machine"), "expected the machine section: {rendered}");
    assert!(rendered.contains("Generic Saw"), "expected the machine's label: {rendered}");
    assert!(rendered.contains("process3d-play-inspector.capability.cut"), "expected the saw's cut capability section: {rendered}");
    assert!(!rendered.contains("Kerf: 0.05"), "a closed capability section materialises no parameter rows: {rendered}");
}

/// ⚖️ LAW (b)/(c): a capability section states its parameter count while closed and materialises
/// exactly those rows once the host opens its window. Driven through `render` with an explicit
/// selection rather than the live `interactionSelect` path, so the law pins the PANEL's windowing and
/// not the framework's selection admission (which its four neighbours above already exercise).
#[semio_framework_async_macros::async_test]
async fn a_capability_section_stamps_its_total_closed_and_materialises_it_open() {
    use crate::{Capability, CapabilityParameter, MeasureRecipe, Workshop, WorkshopMachine};
    use semio_framework_plugin::{TreeWindowRequest, TreeWindows, ViewModel};
    let mut fixture = crate::empty_process3d_snapshot();
    fixture.workshop = Workshop {
        machines: vec![WorkshopMachine {
            id: "saw".into(),
            label: "Generic Saw".into(),
            icon_id: "scissors".into(),
            catalog_id: None,
            capabilities: vec![Capability {
                id: "cut".into(),
                label: "Cut".into(),
                icon_id: "scissors".into(),
                recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
                parameters: vec![
                    CapabilityParameter { id: "bladeDiameter".into(), label: "Blade Diameter".into(), value: 0.4 },
                    CapabilityParameter { id: "kerf".into(), label: "Kerf".into(), value: 0.05 },
                ],
                rules: Vec::new(),
            }],
        }],
    };
    let labels = crate::editor::process3d::terminology::process3d_labels(&ViewModel::default());
    let section = "process3d-play-inspector.capability.cut";
    let project = |windows: &TreeWindows<'_>| {
        let node = render(&fixture, &["machine:saw".to_string()], labels, windows).expect("inspector renders");
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("inspector projection")
    };

    let closed = project(&TreeWindows::unhosted());
    assert!(closed.contains("Generic Saw"), "the open machine summary carries its fields: {closed}");
    assert!(closed.contains(section), "the capability section is a row even while closed: {closed}");
    assert!(!closed.contains("Kerf: 0.05"), "a closed capability section materialises no parameter rows: {closed}");
    let projection: serde_json::Value = serde_json::from_str(&closed).expect("inspector projection json");
    let total = find_window_total(&projection, section).unwrap_or_else(|| panic!("{section} stamps no window: {closed}"));
    assert_eq!(total, 2, "a closed capability section still states its two parameters: {closed}");

    let view = ViewModel {
        tree_windows: vec![TreeWindowRequest { body_key: PROCESS_3D_PLAY_BODY_INSPECTION.into(), node_key: section.into(), open: Some(true), offset: 0, rows: 32 }],
        ..Default::default()
    };
    let opened = project(&TreeWindows::for_body(&view, PROCESS_3D_PLAY_BODY_INSPECTION));
    assert!(opened.contains("Kerf: 0.05"), "opening the section materialises its parameters: {opened}");
    assert!(opened.contains("Blade Diameter: 0.4"), "…all of them: {opened}");
    assert!(!opened.contains(".more\""), "a windowed inspector has no continuation row: {opened}");
}

/// 🔎️ The `window.total` one container stamped, found by key anywhere in a projected body.
fn find_window_total(node: &serde_json::Value, key: &str) -> Option<u64> {
    if node["key"].as_str() == Some(key) {
        return node["component"]["window"]["total"].as_u64();
    }
    node["children"].as_array().and_then(|children| children.iter().find_map(|child| find_window_total(child, key)))
}

//#endregion 🔖️SelectionInspector
