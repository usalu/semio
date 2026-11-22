extern crate semio_os_kernel as store;
extern crate semio_os_kernel as protocol;
#[path = "../../../../🧪️tests/🧰️support/🦀️.rs"]
mod context;
use semio_s_artifact_process_process3d_panels::inspection::*;
use semio_s_artifact_process_process3d_kit::{editor::process3d::terminology::process3d_measure_label, editor::process3d::terminology::Process3dLabels};
use semio_s_artifact_process_process3d_model::{snapshot::Capability, snapshot::Pose, snapshot::Process3dSnapshot, snapshot::ProcessMeasure, snapshot::ProcessStep, snapshot::Stock, snapshot::WorkingSolid, snapshot::WorkshopMachine};
use semio_plugin::tree_item;
use semio_plugin::BuiltNode;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::PanelGroup;
use semio_plugin::PanelTabDefinition;
use semio_plugin::PanelTabKind;
use semio_plugin::PanelTreeBuilder;
use semio_plugin::TreeWindows;
use semio_plugin::UiAssemblyResult;
use semio_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_ID;
use semio_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_LABEL;
use semio_s_artifact_process_process3d_commands::step::add_step;
use semio_s_artifact_process_process3d_editor::Process3dCommand;
use semio_s_artifact_process_process3d_kit::PROCESS3D_INTERACTION_DOMAIN;
use semio_framework::DslValue;

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
    assert_eq!(definition.body_key.as_deref(), Some(PROCESS_3D_PLAY_BODY_INSPECTION));
}

//#region 🔖️AddStepDispatch
/// 🌉️ A measure-routed `AddStep` dispatches a real `CreateStep` mutation against `step_payloads`.
#[semio_framework_async_macros::async_test]
async fn add_step_dispatches_its_mutation() {
    let mut app = context::app();
    let (_, receipt) = context::settled_dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: Some("drill".into()), machine_id: None, capability_id: None, position: None }));
    assert!(context::published_a_document_mutation(&receipt), "AddStep must dispatch its CreateStep mutation");
}

/// 🪚️ The catalogue-routed (machine/capability-addressed) `AddStep` checks the capability rules against the
/// stock: the default circular saw cuts at most 65 mm deep, the default timber beam is 240 mm high, so
/// `add_step::handle` refuses with `process3d.capability.stock-limit` and the document keeps its steps.
#[semio_framework_async_macros::async_test]
async fn add_step_via_catalogue_refuses_a_stock_beyond_the_capability_rules() {
    use semio_plugin::artifact_app_laws::{dispatch_original_fixture_command, fixture_identity, meta, settle_registered_typed_operation};
    let mut app = context::app();
    let before = app.snapshot().expect("document").step_payloads.len();
    let command = Process3dCommand::AddStep(add_step::AddStep { measure: None, machine_id: Some("circularSaw".into()), capability_id: Some("crosscut".into()), position: None });
    let meta = meta("local");
    let outcome = match ::semio_framework_async::poll::resolve_ready(dispatch_original_fixture_command(&mut app.0, &command, &meta, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut fixture_identity(), app.1.cx())) {
        Err(fault) => format!("{fault:?}"),
        Ok(result) => format!("{result:?} {:?}", ::semio_framework_async::poll::resolve_ready(settle_registered_typed_operation(&mut app.0, meta.instance_id, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut fixture_identity(), app.1.cx())).err()),
    };
    assert!(outcome.contains("process3d.capability.stock-limit"), "expected stock-limit refusal: {outcome}");
    assert_eq!(app.snapshot().expect("document").step_payloads.len(), before);
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
    context::action(app, semio_plugin::INTERACTION_SELECT_ACTION_ID, Some(&args));
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
    let semio_s_artifact_process_process3d_model::snapshot::WorkingSolid::Box { width, height, .. } = &stock.solid else { panic!("the seeded example's stock is a box: {:?}", stock.solid) };
    select(&mut app, &stock_id);
    let rendered = context::render(&mut app, PROCESS_3D_PLAY_BODY_INSPECTION);
    assert!(rendered.contains("process3d-play-inspector.stock"), "expected the stock section: {rendered}");
    assert!(rendered.contains("process3d-play-inspector.stock.solid.width.input"), "expected the seeded stock's width {width}: {rendered}");
    assert!(rendered.contains("process3d-play-inspector.stock.solid.height.input"), "expected the seeded stock's height {height}: {rendered}");
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
    assert!(rendered.contains("process3d-play-inspector.step.radius.input"), "expected an editable drill radius: {rendered}");
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
    let _arena = semio_plugin::artifact_app_laws::FixtureUiArena::admit();
    use semio_s_artifact_process_process3d_model::{snapshot::Capability, snapshot::CapabilityParameter, snapshot::MeasureRecipe, snapshot::Workshop, snapshot::WorkshopMachine};
    use semio_plugin::{TreeWindowRequest, TreeWindows, ViewModel};
    let mut fixture = semio_s_artifact_process_process3d_model::snapshot::empty_process3d_snapshot();
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
    let labels = semio_s_artifact_process_process3d_kit::editor::process3d::terminology::process3d_labels(&ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native));
    let section = "process3d-play-inspector.capability.cut";
    let project = |windows: &TreeWindows<'_>| {
        let node = render(&fixture, &["machine:saw".to_string()], labels, windows).expect("inspector renders");
        semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::built_to_component_tree(node)).expect("inspector projection")
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
        ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)
    };
    let opened = project(&TreeWindows::for_body(&view, PROCESS_3D_PLAY_BODY_INSPECTION));
    assert!(opened.contains("process3d-play-inspector.capability.cut.kerf.input"), "opening the section materialises its parameters: {opened}");
    assert!(opened.contains("process3d-play-inspector.capability.cut.bladeDiameter.input"), "…all of them: {opened}");
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

fn pane_node<'a>(root: &'a serde_json::Value, key: &str) -> &'a serde_json::Value {
    fn find<'a>(node: &'a serde_json::Value, key: &str) -> Option<&'a serde_json::Value> {
        if node["key"].as_str() == Some(key) {
            return Some(node);
        }
        node["children"].as_array()?.iter().find_map(|child| find(child, key))
    }
    find(root, key).unwrap_or_else(|| panic!("missing {key}"))
}

fn commit_pane_field(app: &mut context::Process3dApp, input: &serde_json::Value, id: &str, field: &str, value: serde_json::Value) {
    let binding = &input["bindings"][0];
    assert_eq!(binding["trigger"], "commit");
    assert_eq!(binding["action"]["scope"], "s.process.process3d@1/*#editor");
    assert_eq!(binding["args"]["target"], id);
    assert_eq!(binding["args"]["field"], field);
    let mut args = binding["args"].clone();
    args["value"] = value;
    let args: DslValue = semio_framework_pack_json::from_json_str(&args.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let receipt = context::settled_action(app, binding["action"]["name"].as_str().unwrap(), Some(&args)).1;
    assert!(context::published_a_document_mutation(&receipt));
}

#[semio_framework_async_macros::async_test]
async fn selected_step_inspector_dispatches_its_number_binding() {
    let mut app = context::app_with_registry();
    context::dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: Some("drill".into()), machine_id: None, capability_id: None, position: None }));
    let snapshot = app.snapshot().expect("current process");
    let index = snapshot.step_payloads.len() - 1;
    let id = snapshot.step_payloads[index].id.clone();
    select(&mut app, &id);
    let inspector = |app: &mut context::Process3dApp| serde_json::from_str::<serde_json::Value>(&context::render(app, PROCESS_3D_PLAY_BODY_INSPECTION)).unwrap();
    let projection = inspector(&mut app);
    let key = "process3d-play-inspector.step.radius.input";
    let input = pane_node(&projection, key);
    assert_eq!(input["component"]["kind"], "number");
    assert_eq!(input["accessibility"]["label"], "Radius");
    commit_pane_field(&mut app, input, &id, "radius", serde_json::json!(0.04));
    let updated = inspector(&mut app);
    assert_eq!(pane_node(&updated, key)["component"]["value"], "0.04");
    let move_earlier = &pane_node(&updated, "process3d-play-inspector.step.move-earlier")["component"]["target"];
    assert_eq!(move_earlier["scope"], "s.process.process3d@1/*#editor");
    assert_eq!(move_earlier["activation"], "moveStep");
    assert_eq!(move_earlier["args"]["index"].as_f64(), Some((index - 1) as f64));
    let args: DslValue = semio_framework_pack_json::from_json_str(&move_earlier["args"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let receipt = context::settled_action(&mut app, move_earlier["activation"].as_str().unwrap(), Some(&args)).1;
    assert!(context::published_a_document_mutation(&receipt));
    assert_eq!(app.snapshot().expect("reordered process").step_payloads[index - 1].id, id);
}

#[semio_framework_async_macros::async_test]
async fn stock_and_workshop_inspector_dispatch_their_own_fields() {
    let mut app = context::app_with_registry();
    let snapshot = app.snapshot().expect("current process");
    let id = snapshot.stock_id.clone();
    select(&mut app, &id);
    let stock = |app: &mut context::Process3dApp| serde_json::from_str::<serde_json::Value>(&context::render(app, PROCESS_3D_PLAY_BODY_INSPECTION)).unwrap();
    let projection = stock(&mut app);
    let width = "process3d-play-inspector.stock.solid.width.input";
    let position = "process3d-play-inspector.stock.pose.x.input";
    let label = "process3d-play-inspector.stock.label.input";
    commit_pane_field(&mut app, pane_node(&projection, width), &id, "width", serde_json::json!("1.25"));
    commit_pane_field(&mut app, pane_node(&projection, position), &id, "posX", serde_json::json!(0.5));
    commit_pane_field(&mut app, pane_node(&projection, label), &id, "label", serde_json::json!("123"));
    let updated = stock(&mut app);
    assert_eq!(pane_node(&updated, width)["component"]["value"], "1.25");
    assert_eq!(pane_node(&updated, position)["component"]["value"], "0.5");
    assert_eq!(pane_node(&updated, label)["component"]["value"], "123");

    let machine_id = "machine:saw";
    let section = "process3d-play-inspector.capability.cut";
    select(&mut app, machine_id);
    let view = semio_plugin::ViewModel {
        tree_windows: vec![semio_plugin::TreeWindowRequest { body_key: PROCESS_3D_PLAY_BODY_INSPECTION.into(), node_key: section.into(), open: Some(true), offset: 0, rows: 32 }],
        ..semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)
    };
    let workshop = |app: &mut context::Process3dApp| serde_json::from_str::<serde_json::Value>(&context::render_with_view(app, PROCESS_3D_PLAY_BODY_INSPECTION, &view)).unwrap();
    let projection = workshop(&mut app);
    let label = "process3d-play-inspector.machine.label.input";
    let parameter = "process3d-play-inspector.capability.cut.kerf.input";
    commit_pane_field(&mut app, pane_node(&projection, label), machine_id, "label", serde_json::json!("Workshop saw"));
    commit_pane_field(&mut app, pane_node(&projection, parameter), machine_id, "cut.kerf", serde_json::json!(0.04));
    let updated = workshop(&mut app);
    assert_eq!(pane_node(&updated, label)["component"]["value"], "Workshop saw");
    assert_eq!(pane_node(&updated, parameter)["component"]["value"], "0.04");
}
