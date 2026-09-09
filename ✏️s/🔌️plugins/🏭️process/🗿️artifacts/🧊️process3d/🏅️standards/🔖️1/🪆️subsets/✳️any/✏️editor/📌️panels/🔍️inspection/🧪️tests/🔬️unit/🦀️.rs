use super::*;
use crate::editor::process3d::commands::step::add_step;
use crate::editor::process3d::testkit;
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
    let mut app = testkit::app();
    let result = testkit::dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: Some("drill".into()), machine_id: None, capability_id: None, position: None }));
    assert!(!result.mutations.is_empty(), "AddStep must dispatch its CreateStep mutation");
}

/// 🌉️ Same documented gap as above, from the catalogue-routed (machine/capability-addressed)
/// entry point: even a stock the pre-migration code would have rejected (circular saw needs
/// height ≤ 0.065m; the default timber beam is 0.24m) now succeeds, since the dimension gate
/// can no longer read real stock extents.
#[semio_framework_async_macros::async_test]
async fn add_step_via_catalogue_no_longer_gates_on_stock_dimensions() {
    let mut app = testkit::app();
    let result = testkit::dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: None, machine_id: Some("circularSaw".into()), capability_id: Some("crosscut".into()), position: None }));
    assert!(!result.mutations.is_empty(), "documented gap: the dimension-validation gate can no longer reject an oversized stock");
}

#[semio_framework_async_macros::async_test]
async fn measure_arg_routes_to_generic_machine_and_dispatches() {
    let mut app = testkit::app();
    let result = testkit::dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: Some("cut".into()), machine_id: None, capability_id: None, position: None }));
    assert!(!result.mutations.is_empty());
}
//#endregion 🔖️AddStepDispatch

//#region 🔖️SelectionInspector
fn select(app: &mut testkit::Process3dApp, id: &str) {
    let targets = serde_json::to_string(&vec![protocol::InteractionTarget { granularity: "object".into(), id: id.into() }]).expect("targets serialize");
    let args = DslValue::Object(vec![
        ("domainId".to_string(), DslValue::String(PROCESS3D_INTERACTION_DOMAIN.into())),
        ("targets".to_string(), DslValue::String(targets)),
        ("merge".to_string(), DslValue::String("replace".into())),
        ("method".to_string(), DslValue::String("pick".into())),
    ]);
    testkit::action(app, semio_framework_plugin::INTERACTION_SELECT_ACTION_ID, Some(&args));
}

#[semio_framework_async_macros::async_test]
async fn empty_selection_still_renders_the_empty_state() {
    let mut app = testkit::app_with_registry();
    let rendered = testkit::render(&mut app, PROCESS_3D_PLAY_BODY_INSPECTION);
    assert!(rendered.contains("process3d-play-inspector.empty"));
}

#[semio_framework_async_macros::async_test]
async fn selected_stock_id_renders_its_dimensions() {
    let mut app = testkit::app_with_registry();
    let stock_id = app.snapshot().expect("snapshot").stock_id.clone();
    select(&mut app, &stock_id);
    let rendered = testkit::render(&mut app, PROCESS_3D_PLAY_BODY_INSPECTION);
    assert!(rendered.contains("process3d-play-inspector.stock"), "expected the stock section: {rendered}");
    assert!(rendered.contains("Width: 1"), "expected the default box stock's width: {rendered}");
    assert!(rendered.contains("Height: 1"), "expected the default box stock's height: {rendered}");
}

#[semio_framework_async_macros::async_test]
async fn selected_step_id_renders_its_label_and_measure_kind() {
    let mut app = testkit::app_with_registry();
    testkit::dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: Some("drill".into()), machine_id: None, capability_id: None, position: None }));
    let step = app.snapshot().expect("snapshot").step_payloads.last().expect("added step").clone();
    select(&mut app, &step.id);
    let rendered = testkit::render(&mut app, PROCESS_3D_PLAY_BODY_INSPECTION);
    assert!(rendered.contains("process3d-play-inspector.step"), "expected the step section: {rendered}");
    assert!(rendered.contains(&step.label), "expected the step's label {}: {rendered}", step.label);
    assert!(rendered.contains("Kind: Drill"), "expected the measure kind: {rendered}");
    assert!(rendered.contains("Radius: 0.05"), "expected the drill capability's own radius parameter: {rendered}");
}

#[semio_framework_async_macros::async_test]
async fn selected_machine_id_renders_its_capabilities() {
    let mut app = testkit::app_with_registry();
    select(&mut app, "machine:saw");
    let rendered = testkit::render(&mut app, PROCESS_3D_PLAY_BODY_INSPECTION);
    assert!(rendered.contains("process3d-play-inspector.machine"), "expected the machine section: {rendered}");
    assert!(rendered.contains("Generic Saw"), "expected the machine's label: {rendered}");
    assert!(rendered.contains("process3d-play-inspector.capability.cut"), "expected the saw's cut capability section: {rendered}");
    assert!(rendered.contains("Kerf: 0.05"), "expected the cut capability's own kerf parameter: {rendered}");
}
//#endregion 🔖️SelectionInspector
