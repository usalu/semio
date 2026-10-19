//! 🎮️ Process3d editor unit tests: labels, undo, world pointer and face drag gestures, replay cursor, sun, window bodies and the contribution envelope limits.

use super::*;

//#region 🔖️CrossCutting
#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_by_default_and_in_german() {
    let english = semio_framework_plugin::ViewModel { locale: semio_framework_ui_locale::Locale::En, ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    assert_eq!(process3d_labels(&english).stock.as_str(), "Stock");
    let german = semio_framework_plugin::ViewModel { locale: semio_framework_ui_locale::Locale::De, ..semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::De, semio_framework_ui_locale::Terminology::Native) };
    assert_eq!(process3d_labels(&german).stock.as_str(), "Rohteil");
}

/// ↩️ Ticket `26/09/01/PROCESS-END-TO-END`: `AddStep` dispatches a real `CreateStep` mutation
/// against the durable `step_payloads` timeline — the minted `steps` flow child gains a node,
/// so its content-addressed handle changes. Undo restores the pre-add handle; redo re-applies it.
#[semio_framework_async_macros::async_test]
async fn undo_after_add_step_restores_the_steps_handle() {
    let mut app = app();
    let before = app.snapshot().expect("snapshot").steps.clone();
    artifact_app_laws::assert_undo_redo_round_trip(&mut app.0, Process3dCommand::AddStep(add_step::AddStep { measure: Some("cut".into()), machine_id: None, capability_id: None, position: None }), |app| app.snapshot().expect("snapshot").steps == before, true, false, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx())
    .await;
}

#[semio_framework_async_macros::async_test]
async fn undo_after_add_workshop_machine_restores_previous_machine_count() {
    let mut app = app();
    artifact_app_laws::assert_undo_redo_round_trip(&mut app.0, Process3dCommand::AddWorkshopMachine(add_workshop_machine::AddWorkshopMachine { catalog_id: "metal".into(), machine_id: "chopSaw".into() }), |app| app.snapshot().expect("snapshot").workshop.machines.len(), 11, 12, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx())
    .await;
}

/// 🧬️ Swapping the stock kind resets the whole document (stock + cleared timeline), which has no
/// in-history mutation (a whole-snapshot variant is banned outright), so `setStock` now surfaces as a
/// `Effect::LoadDocument` rather than an `artifact_mutations` entry — `dispatch`'s in-process
/// harness never applies `effects` to its own store, so this asserts on the emitted effect.
#[semio_framework_async_macros::async_test]
async fn arg_form_set_stock_emits_ops_reading_kind_arg() {
    let mut app = app();
    let result = dispatch(&mut app, Process3dCommand::SetStock(set_stock::SetStock { kind: "cylinder".into() }));
    assert!(result.mutations.is_empty(), "setStock replaces the whole document via an effect, not in-history mutations");
    let Effect::LoadDocument { pack, .. } = result.requested_effects.first().expect("setStock must emit a LoadDocument effect") else {
        panic!("expected a LoadDocument effect");
    };
    let document = <Process3dSnapshot as ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
    let expected_solid = semio_s_artifact_process_process3d_model::snapshot::brep_child_handle("stock", &semio_s_artifact_process_process3d_model::snapshot::brep_snapshot_for_working_solid(&WorkingSolid::Cylinder { radius: 0.3, height: 1.0 }));
    assert_eq!(document.stock_solid, expected_solid, "setStock kind=cylinder must swap the stock solid to the real cylinder-content handle");
    let cleared_steps = semio_s_artifact_process_process3d_model::snapshot::flow_child_handle(&semio_s_artifact_process_process3d_model::snapshot::flow_snapshot_for_steps(&[], &Default::default()));
    assert_eq!(document.steps, cleared_steps, "swapping stock resets the step timeline");
}

/// 🌉️ `WorldPointerDown` dispatches `insert_step_mutations` → a real `CreateStep` mutation
/// against `step_payloads`, appended at the resolved-up-to cursor (or the timeline end). This
/// asserts the command dispatches a mutation for a real world-space click.
#[semio_framework_async_macros::async_test]
async fn world_pointer_down_dispatches_a_mutation_for_a_real_click() {
    let mut app = app();
    let (_, receipt) = settled_dispatch_with_utility(&mut app, Process3dCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { position: [1.0, 2.0, 3.0] }), "cut");
    assert!(published_a_document_mutation(&receipt), "worldPointerDown must still dispatch a mutation for a real click");
}

#[semio_framework_async_macros::async_test]
async fn world_pointer_down_resets_active_utility_to_select() {
    let mut app = app();
    let result = dispatch_with_utility(&mut app, Process3dCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { position: [1.0, 2.0, 3.0] }), "cut");
    assert!(
        result.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveUtility { utility_id, .. } if utility_id == "select")),
        "placing a step must hand the host a SetActiveUtility(select) effect so the click-to-place utility disengages",
    );
}

/// 🌉️ Two distinct real clicks each dispatch their own `CreateStep`, appended in order.
#[semio_framework_async_macros::async_test]
async fn repeated_world_pointer_down_each_dispatch_a_mutation() {
    let mut app = app();
    let (_, first) = settled_dispatch_with_utility(&mut app, Process3dCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { position: [1.0, 0.0, 0.0] }), "cut");
    let (_, second) = settled_dispatch_with_utility(&mut app, Process3dCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { position: [2.0, 0.0, 0.0] }), "cut");
    assert!(published_a_document_mutation(&first) && published_a_document_mutation(&second), "each real click must dispatch its own mutation");
}

/// 🌉️ `WorldFaceDragEnd` dispatches `insert_step_mutations` → a real `CreateStep` mutation.
/// The kernel-replay math (cut/attach volume deltas) is covered directly against a literal
/// `ProcessWorkingScene` by `🧬️schema/💡️inferences`'s own
/// `drill_reduces_volume_below_stock`/`attach_increases_volume_above_stock` tests; these two
/// assert only that the command dispatches a mutation for a real face-drag gesture.
#[semio_framework_async_macros::async_test]
async fn world_face_drag_end_cut_dispatches_a_mutation() {
    let mut app = app();
    let (_, receipt) = settled_dispatch(&mut app, Process3dCommand::WorldFaceDragEnd(world_face_drag_end::WorldFaceDragEnd { normal: [0.0, 0.0, 1.0], start_point: [0.5, 0.5, 1.0], distance: -0.5, face_extent: Some([1.0, 1.0]) }));
    assert!(published_a_document_mutation(&receipt), "a cut face-drag must dispatch its CreateStep mutation");
}

#[semio_framework_async_macros::async_test]
async fn world_face_drag_end_attach_dispatches_a_mutation() {
    let mut app = app();
    let (_, receipt) = settled_dispatch(&mut app, Process3dCommand::WorldFaceDragEnd(world_face_drag_end::WorldFaceDragEnd { normal: [0.0, 0.0, 1.0], start_point: [0.5, 0.5, 1.0], distance: 0.5, face_extent: Some([0.2, 0.2]) }));
    assert!(published_a_document_mutation(&receipt), "an attach face-drag must dispatch its CreateStep mutation");
}

#[semio_framework_async_macros::async_test]
async fn world_face_drag_end_ignored_while_a_placement_utility_is_active() {
    let mut app = app();
    let result = dispatch_with_utility(&mut app, Process3dCommand::WorldFaceDragEnd(world_face_drag_end::WorldFaceDragEnd { normal: [0.0, 0.0, 1.0], start_point: [0.5, 0.5, 1.0], distance: -0.5, face_extent: None }), "cut");
    assert!(result.mutations.is_empty(), "worldFaceDragEnd should be a no-operation while a placement utility is active, not the select utility");
}

/// ⏱️ The replay cursor is view state: every cursor verb publishes the CONFIG lane only — never the document lane,
/// never a history row — and the document is unchanged after a whole replay session.
#[semio_framework_async_macros::async_test]
async fn stepping_the_replay_cursor_never_edits_the_document() {
    let mut app = app();
    let before = app.snapshot().expect("snapshot");
    for command in [
        Process3dCommand::StepCursorBack(step_cursor_back::StepCursorBack {}),
        Process3dCommand::StepCursorBack(step_cursor_back::StepCursorBack {}),
        Process3dCommand::StepCursorForward(step_cursor_forward::StepCursorForward {}),
        Process3dCommand::SetCursor(set_cursor::SetCursor { value: Some(1) }),
        Process3dCommand::StepCursor(step_cursor::StepCursor { delta: 2 }),
    ] {
        let (_, receipt) = settled_dispatch(&mut app, command);
        assert!(!published_a_document_mutation(&receipt), "a cursor move must never write the document: {:?}", receipt.lanes);
        assert!(receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Config), "a cursor move writes the viewer's config: {:?}", receipt.lanes);
    }
    assert_eq!(app.snapshot().expect("snapshot"), before, "a replay session leaves the document untouched");
}

#[semio_framework_async_macros::async_test]
async fn toggle_sun_round_trips_through_config_and_defaults_off() {
    let mut app = app();
    let measures = app.window_measures(&main_window_view()).await;
    let sun_group = |measures: &HashMap<String, Vec<WindowMeasure>>| {
        measures[workpiece::PROCESS_3D_PLAY_WINDOW_MAIN]
            .iter()
            .find_map(|measure| match measure {
                WindowMeasure::Group { id, children, .. } if id == "process3d-measure-sun" => Some(children.clone()),
                _ => None,
            })
            .expect("sun measure group")
    };
    let children = sun_group(&measures);
    assert!(children.iter().any(|measure| matches!(measure, WindowMeasure::Toggle { pressed, .. } if !*pressed)));
    dispatch(&mut app, Process3dCommand::ToggleSun(toggle_sun::ToggleSun {}));
    let measures = app.window_measures(&main_window_view()).await;
    let children = sun_group(&measures);
    assert!(children.iter().any(|measure| matches!(measure, WindowMeasure::Toggle { pressed, .. } if *pressed)));
}

#[semio_framework_async_macros::async_test]
async fn window_measures_surface_the_sun_group() {
    let mut app = app();
    let measures = main_window_measures(&mut app);
    assert_eq!(measures.len(), 1);
    assert!(matches!(&measures[0], WindowMeasure::Group { id, .. } if id == "process3d-measure-sun"));
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    let mut app = app();
    assert!(render_body(&mut app, "process3d.play.nope").contains("Unknown body"));
}

#[semio_framework_async_macros::async_test]
async fn window_body_accepts_the_framework_instance_suffix() {
    let mut app = app();
    let body_key = format!("{}:{}", PROCESS_3D_PLAY_BODY_MAIN, workpiece::PROCESS_3D_PLAY_WINDOW_MAIN);
    let rendered = render_body(&mut app, &body_key);
    assert!(rendered.contains("processed"), "window-instance body key must render the Process world: {rendered}");
}

/// 🧪️ The registry-enforced app must accept every declared manifest action id without a kind-
/// discipline error — proves the `app_commands!` rows and the manifest's `.operation`/`.shell_action`/
/// `.action_with` declarations stay in sync.
#[semio_framework_async_macros::async_test]
async fn registry_enforced_app_accepts_a_declared_operation_action() {
    let mut app = app_with_registry();
    let (_, receipt) = settled_dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: Some("cut".into()), machine_id: None, capability_id: None, position: None }));
    assert!(published_a_document_mutation(&receipt), "a declared operation action must reach the document lane");
}

/// 🙈️ LAW: `setStepEnabled` — the step row's eye — is idempotent by value: the value the row's target asks for publishes
/// one document edit, replaying it publishes none and leaves the step exactly as the first dispatch set it; a dispatch
/// without its value is refused at the command boundary, never defaulted into a flip.
#[semio_framework_async_macros::async_test]
async fn set_step_enabled_is_idempotent_by_value_and_refuses_a_missing_value() {
    let mut app = app_with_registry();
    settled_dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: Some("cut".into()), machine_id: None, capability_id: None, position: None }));
    let step = app.snapshot().expect("snapshot with the added step").step_payloads.last().cloned().expect("the added step");
    let bare = DslValue::object([("id".to_string(), DslValue::String(step.id.clone()))]);
    assert!(<Process3dPlayApp as ArtifactEditor>::command_from_action("setStepEnabled", Some((bare).clone()), None).is_err(), "setStepEnabled without its value must be refused");
    let command = || Process3dCommand::SetStepEnabled(set_step_enabled::SetStepEnabled { id: step.id.clone(), enabled: !step.enabled });
    let (_, first) = settled_dispatch(&mut app, command());
    assert!(published_a_document_mutation(&first), "setStepEnabled with the inverse value is one document edit");
    let set = app.snapshot().expect("snapshot after the first set");
    assert_eq!(set.step_payloads.iter().find(|entry| entry.id == step.id).map(|entry| entry.enabled), Some(!step.enabled), "the step reaches the value the row asked for");
    let (_, replay) = settled_dispatch(&mut app, command());
    assert!(!published_a_document_mutation(&replay), "replaying setStepEnabled with the same value publishes no second edit");
    assert_eq!(app.snapshot().expect("snapshot after the replay"), set, "replaying setStepEnabled with the same value leaves the document unchanged");
}
//#endregion 🔖️CrossCutting

//#region 🔖️BehaviorTests
#[semio_framework_async_macros::async_test]
async fn face_drag_orients_box_along_normal() {
    let (axis, angle) = axis_angle_from_up_to([0.0, 1.0, 0.0]);
    assert!((angle - std::f64::consts::FRAC_PI_2).abs() < 1e-9);
    assert!((axis[0] - (-1.0)).abs() < 1e-9 && axis[1].abs() < 1e-9 && axis[2].abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn face_drag_degenerate_antiparallel_normal_does_not_panic() {
    let (_, angle) = axis_angle_from_up_to([0.0, 0.0, -1.0]);
    assert!((angle - std::f64::consts::PI).abs() < 1e-9);
}

#[semio_framework_async_macros::async_test]
async fn process_machine_contributions_are_configuration_owned() {
    use semio_framework::{ProgramContributionEntry, TopicContribution};
    let machine = WorkshopMachine { id: "hot-saw".into(), label: "Hot Saw".into(), icon_id: "scissors".into(), catalog_id: None, capabilities: vec![] };
    let entry = ProgramContributionEntry {
        plugin_id: "process-module-test".into(),
        topic_contribution: Some(TopicContribution::new(
            "process.machines",
            DslValue::object([
                ("appId".to_string(), DslValue::String("process3d-play".to_string())),
                ("moduleId".to_string(), DslValue::String("hot-catalog".to_string())),
                ("label".to_string(), DslValue::String("Hot Catalog".to_string())),
                ("iconId".to_string(), DslValue::String("wrench".to_string())),
                ("machinesJson".to_string(), DslValue::String(semio_framework_pack_json::to_json_string(&vec![machine]))),
            ]),
        )),
    };
    let json = semio_framework_pack_json::to_json_string(&vec![entry]);
    assert!(installed_catalogs(&json).iter().any(|catalog| catalog.catalog_id() == "hot-catalog"));
    assert!(!installed_catalogs("[]").iter().any(|catalog| catalog.catalog_id() == "hot-catalog"));
}

#[semio_framework_async_macros::async_test]
async fn process_contribution_envelope_accepts_exact_limits_and_rejects_plus_one() {
    let raw_max = format!("{{}}{}", " ".repeat(PROCESS_CONTRIBUTION_MAX_BYTES - 2));
    assert!(process_json_envelope_is_bounded(&raw_max));
    assert!(!process_json_envelope_is_bounded(&(raw_max + " ")));

    let depth_max = format!("{}0{}", "[".repeat(PROCESS_CONTRIBUTION_MAX_DEPTH), "]".repeat(PROCESS_CONTRIBUTION_MAX_DEPTH));
    assert!(process_json_envelope_is_bounded(&depth_max));
    let depth_plus_one = format!("{}0{}", "[".repeat(PROCESS_CONTRIBUTION_MAX_DEPTH + 1), "]".repeat(PROCESS_CONTRIBUTION_MAX_DEPTH + 1));
    assert!(!process_json_envelope_is_bounded(&depth_plus_one));

    let string_max = format!("\"{}\"", "x".repeat(PROCESS_CONTRIBUTION_MAX_STRING_BYTES));
    assert!(process_json_envelope_is_bounded(&string_max));
    let string_plus_one = format!("\"{}\"", "x".repeat(PROCESS_CONTRIBUTION_MAX_STRING_BYTES + 1));
    assert!(!process_json_envelope_is_bounded(&string_plus_one));

    let items_max = format!("[{}]", vec!["0"; PROCESS_CONTRIBUTION_MAX_ITEMS - 1].join(","));
    assert!(process_json_envelope_is_bounded(&items_max));
    let items_plus_one = format!("[{}]", vec!["0"; PROCESS_CONTRIBUTION_MAX_ITEMS].join(","));
    assert!(!process_json_envelope_is_bounded(&items_plus_one));
}
//#endregion 🔖️BehaviorTests
