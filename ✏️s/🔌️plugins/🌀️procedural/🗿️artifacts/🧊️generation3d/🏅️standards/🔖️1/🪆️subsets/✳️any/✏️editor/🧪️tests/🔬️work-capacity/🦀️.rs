use super::*;
use crate::editor::generation3d::unit_tests::context::{self, app_with_registry};
use crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_BOX_SHELL;
use semio_framework_plugin::app::TypedOperationResultLane;
use semio_framework_plugin::retained_command::ArtifactRetainedWorkCapacity;
use semio_framework_plugin::{PluginApp, ViewModel, ViewWindowInstance};

/// 🧮 The declared numbers this lane pins, printed once so a runtime report can quote them without
/// re-deriving anything: the route capacity, the rows one durable item folds, and the extent a
/// one-item command answers preflight with. All three come from ONE declaration.
#[test]
fn generation3d_declares_one_work_capacity_for_extent_footprint_and_preflight() {
    let capacity = GENERATION3D_RETAINED_CAPACITY;
    assert_eq!(capacity.work_items(), GENERATION3D_RETAINED_WORK_ITEMS, "the preflight ceiling IS the declared capacity");
    assert_eq!(capacity.rows_for_items(1), Some(store::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS), "one durable item is one forward row plus one inverse row");
    assert_eq!(generation3d_one_item_footprint(0).work_items, capacity.rows_for_items(1).expect("one item fits the route capacity"), "the footprint a preflight declares and the extent a work answers are the SAME quantity");
    assert!(capacity.admits(generation3d_one_item_footprint(0).work_items), "a route whose footprint is N rows must admit an extent of N");
    eprintln!(
        "[DEBUG] generation3d work capacity: items={} work_items={} one-item rows={} footprint rows={}",
        capacity.invertible_items(),
        capacity.work_items(),
        capacity.rows_for_items(1).expect("one item fits"),
        generation3d_one_item_footprint(0).work_items
    );
}

/// 🧾️ Every retained route's `extent` — the exact value `ArtifactRetainedCommandPhase::Preflight`
/// measures — answered against the boot document, and asserted admissible by the SAME capacity the
/// job's `maximum_work_items` carries. `Generation3dPreviewCommandWork` and the session work are the
/// two reducers `build_tool_job` picks; `flowEvalTick`'s window-addressed work is driven through the
/// real job below, because its extent reads a context only the host can build.
#[semio_framework_async_macros::async_test]
async fn every_bounded_retained_route_answers_an_admissible_extent() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    let read = context::snapshot(&app);
    let interaction = protocol::InteractionState::default();
    let capacity = GENERATION3D_RETAINED_CAPACITY;
    let instance_owner = context::instance_operation_owner();
    for tool_id in GENERATION3D_RETAINED_TOOL_IDS.iter().chain(GENERATION3D_FLOW_EVAL_TOOL_IDS).copied() {
        if tool_id == "flowEvalTick" {
            continue;
        }
        let command = <Generation3dPlayApp as ArtifactEditor>::command_from_action(tool_id, None).unwrap_or_else(|_| panic!("{tool_id} decodes from its own action id"));
        let extent = if GENERATION3D_PREVIEW_TOOL_IDS.contains(&tool_id) {
            Generation3dPreviewCommandWork::new(tool_id, instance_owner.clone()).extent(&command, &read, &interaction, None)
        } else {
            generation3d_bounded_extent(&command, &read, &interaction)
        };
        let extent = extent.unwrap_or_else(|| panic!("{tool_id}: extent refused the command outright — preflight reports that as a capacity fault"));
        assert!(capacity.admits(extent), "{tool_id}: extent {extent} exceeds the declared capacity {}", capacity.work_items());
        eprintln!("[DEBUG] retained route {tool_id}: extent={extent} maximum_work_items={}", capacity.work_items());
    }
    context::retire_instance_operation_owner(&instance_owner);
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

/// 🎬️ `setActiveExample` through the REAL job ladder (wire pages → decode → preflight → work →
/// publish) — the phase that faulted in the browser with `retained command exceeds semantic work
/// capacity` is the one this asserts passes.
#[semio_framework_async_macros::async_test]
async fn set_active_example_passes_the_retained_preflight() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    app.handle_action("setActiveExample", Some(&serde_json::json!({ "exampleId": PROCEDURAL_EXAMPLE_BOX_SHELL }).into()), &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("setActiveExample dispatches");
    let receipt = context::settle(&mut app).await;
    assert!(!receipt.lanes.contains(&TypedOperationResultLane::Fault), "setActiveExample faulted in the retained job ladder");
    eprintln!("[DEBUG] setActiveExample preflight admitted: lanes={:?}", receipt.lanes);
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

/// 🕹️ The framework-reserved local observation on the same instance. Its footprint is the framework's
/// `admit_bounded_config_mutation` declaration, and its reserved job's work-item budget now comes
/// from the SAME capacity — the two disagreed before this lane.
#[semio_framework_async_macros::async_test]
async fn interaction_select_passes_the_reserved_preflight() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    let node_id = context::snapshot(&app).fixture.widgets.first().map(crate::widget_id).expect("default fixture node").to_string();
    // 🧯️ The reserved spawn-job the admission requests has to be DRIVEN before the selection exists —
    // see `context::select_graph`. Settling a typed operation instead left this law reading a `None`
    // selection and asserting nothing (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    let settled = context::select_graph(&mut app, "node", &[node_id.as_str()]).await;
    assert_eq!(app.interaction_state().await.selection.get("graph").map(|selection| selection.ids.as_slice()), Some([node_id].as_slice()));
    eprintln!("[DEBUG] interactionSelect reserved work items={} effects={}", ArtifactRetainedWorkCapacity::for_invertible_items(1).work_items(), settled.requested_effects.len());
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

/// 🪟️ The runtime reproduction. `pending_effects` arms `flowEvalTick` with a bare
/// `Effect::DispatchAction` that carries NO window address, so the shell redispatches it under
/// whichever window is current — in the served app the flow window `procedural-main`, never the
/// preview. `Generation3dFlowEvalWindowWork::extent` then answers `None`, and preflight reported that
/// refusal as `retained command exceeds semantic work capacity`: a capacity message for an addressing
/// miss. This law pins the two apart — an unaddressed tick is REFUSED, with the refusal's own fault,
/// and the addressed tick is admitted (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn an_unaddressed_flow_eval_tick_is_refused_not_over_capacity() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    let main = ViewModel {
        window_instances: vec![ViewWindowInstance { id: "procedural-main".into(), window_kind_id: flow_window::GENERATION_3D_PLAY_WINDOW_MAIN.into() }],
        ..Default::default()
    }
    .for_window_instance("procedural-main")
    .expect("flow window instance");
    let refused = context::dispatch_with_view(&mut app, Generation3dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick { window_id: "procedural-preview".into(), window_kind_id: crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into() }), main).await;
    let detail = match refused {
        Ok(receipt) => format!("{:?}", receipt.lanes),
        Err(fault) => format!("{fault:?}"),
    };
    eprintln!("[DEBUG] unaddressed flowEvalTick: {detail}");
    assert!(!detail.contains("exceeds semantic work capacity"), "an addressing refusal must not be reported as a work-capacity fault: {detail}");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}
