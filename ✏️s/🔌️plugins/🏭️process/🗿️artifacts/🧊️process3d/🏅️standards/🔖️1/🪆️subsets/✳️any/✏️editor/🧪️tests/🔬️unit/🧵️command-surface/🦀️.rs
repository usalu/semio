//! 🧵️ Process3d editor unit tests: the retained command routes, the resumable work, the command round trips and the context menu.

use super::*;

/// 🎟️ Unbounded retained grant: the pre-migration step budget carried no retained limit.
const UNBOUNDED_STEP_GRANT: semio_framework_job::RetainedCloneGrant = semio_framework_job::RetainedCloneGrant { maximum_items: usize::MAX, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX };

//#region 🔖️CommandSurface
fn retained_snapshot(machine_count: usize) -> Process3dSnapshot {
    let mut snapshot = semio_s_artifact_process_process3d_model::snapshot::empty_process3d_snapshot();
    snapshot.workshop.machines.clear();
    snapshot.step_payloads.clear();
    snapshot.tool_solids.clear();
    snapshot.workshop.machines.extend((0..machine_count).map(|index| WorkshopMachine { id: format!("retained-{index}"), label: String::new(), icon_id: String::new(), catalog_id: None, capabilities: Vec::new() }));
    snapshot
}

fn retained_operation() -> AppOperationContext {
    AppOperationContext { app_instance_id: 1, parent_document_id: "process3d-retained-test".into(), operation_id: 2, generation: 3, canonical_base_revision: [4; 32], authoring_seed: "authoring-seed-test".into() }
}

fn drive_resumable_work(
    work: &mut Process3dResumableCommandWork,
    command: &Process3dCommand,
    snapshot: &Process3dSnapshot,
    config: &Process3dConfig,
    interaction: &protocol::InteractionState,
    history: &HistoryView,
) -> (usize, Emit<Process3dMutation, Process3dConfigMutation, NoDraftMutation>) {
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let operation = retained_operation();
    let mut progress = 0;
    loop {
        match work.step(&semio_framework_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: None, command, snapshot, config, history, interaction, hover: &hover, context: None, operation: &operation }, &mut semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(256, u64::MAX, UNBOUNDED_STEP_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut 0, &mut semio_framework_job::RetainedCloneProgress::default())).expect("retained work step") {
            ArtifactCommandWorkStep::Replay { .. } | ArtifactCommandWorkStep::Progress { .. } => progress += 1,
            ArtifactCommandWorkStep::Complete(emit) => return (progress, emit),
            ArtifactCommandWorkStep::CompleteWithEphemeral { emit, .. } => return (progress, emit),
            ArtifactCommandWorkStep::CompleteDownload { .. } => panic!("a process3d document command never completes as a download"),
        }
    }
}

#[test]
fn retained_route_dispositions_are_exact_and_exhaustive() {
    use semio_framework::{ToolCancellationPolicy, ToolExecutionShape};

    let mut ids = PROCESS3D_BOUNDED_TOOL_IDS.iter().chain(PROCESS3D_RESUMABLE_TOOL_IDS).copied().collect::<Vec<_>>();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 31);
    assert_eq!(PROCESS3D_BOUNDED_TOOL_IDS.len(), 25);
    assert_eq!(PROCESS3D_RESUMABLE_TOOL_IDS.len(), 6);
    assert!(ids.iter().all(|tool_id| process3d_command_disposition(tool_id).is_some()));
    assert_eq!(process3d_command_disposition("nonsense"), None);
    assert_eq!(<Process3dPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), 31);
    assert_eq!(process3d_bounded_contract().shape, ToolExecutionShape::BoundedFirstStep);
    assert_eq!(process3d_resumable_contract().shape, ToolExecutionShape::Resumable);
    assert_eq!(process3d_resumable_contract().cancellation, ToolCancellationPolicy::PerOperation);
    assert_eq!(process3d_resumable_contract().checkpoint_every_steps, 1);
    assert_eq!(process3d_resumable_contract().progress_every_steps, 1);
}

/// 🕹️ Every command this app declares is UI-reachable, on one of the three lanes it actually owns.
/// A browser dispatch passes three framework gates — `validate_ui_dispatch_classification` (the
/// manifest classification must be `Migrated`), `qualified_tool_proof` (the id must own a retained
/// disposition so `build_tool_job` builds a job rather than returning `Ok(None)`), and
/// `unsupported_publication_contracts` (each named lane must have a preparation factory) — so this
/// pins all three at once, over the closed command vocabulary rather than a hand-kept list.
#[semio_framework_async_macros::async_test]
async fn every_declared_command_is_ui_reachable_on_a_real_lane() {
    use semio_framework_plugin::ArtifactToolPublicationLane;

    let definition = create_process3d_app();
    let declared = definition
        .window_kinds
        .iter()
        .flat_map(|window| semio_framework::window_kind_actions(&definition, window).into_iter().map(|action| (action.id.as_str(), action.semantics.execution.interactive_job)))
        .chain(definition.commands.iter().map(|command| (command.id.as_str(), command.semantics.execution.interactive_job)))
        .collect::<HashMap<_, _>>();
    let lanes = <Process3dBoundedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS
        .iter()
        .chain(<Process3dResumableCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS)
        .map(|contract| (contract.tool_id, contract.lanes))
        .collect::<HashMap<_, _>>();
    for command in every_command() {
        let id = command.command_id();
        assert!(process3d_command_disposition(id).is_some(), "{id} owns no retained disposition, so build_tool_job would refuse to create a job");
        let contract = lanes.get(id).copied().unwrap_or_else(|| panic!("{id} names no publication-lane contract"));
        assert!(!contract.is_empty(), "{id} names an empty publication-lane contract");
        assert!(!contract.contains(&ArtifactToolPublicationLane::HostOnly) || contract.len() == 1, "{id} mixes HostOnly with a store lane: {contract:?}");
        assert!(
            contract.iter().all(|lane| matches!(lane, ArtifactToolPublicationLane::HostOnly | ArtifactToolPublicationLane::Artifact | ArtifactToolPublicationLane::Config)),
            "{id} publishes on a lane this app owns no one-item preparation factory for: {contract:?}"
        );
        assert_eq!(declared.get(id).copied(), Some(InteractiveJobClassification::Migrated), "{id} is not declared Migrated, so validate_ui_dispatch_classification would reject its UI dispatch");
    }
    assert_eq!(lanes.len(), 31);
    assert!(<Process3dPlayApp as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().is_some(), "the Artifact lane is rejected outright without a document one-item preparation factory");
    assert!(<Process3dPlayApp as ArtifactEditor>::build_config_store_one_item_preparation_factory().is_some());
}

/// 📬️ The document lane prepares through the mutation's OWN `diff`/`inverse`, keeps the fixed
/// per-turn grant separate from the document's validation maximum (a measured base compared against
/// the host's 4 KiB grant would stall a large document forever), and refuses an `Error`/`Fatal`
/// outcome instead of publishing its forced-empty diff as a no-op edit.
#[test]
fn document_preparation_uses_the_mutations_own_semantics_and_a_fixed_per_turn_grant() {
    use semio_s_artifact_process_process3d_mut_steps::create_step::CreateStep;
    use semio_s_artifact_process_process3d_mut_steps::delete_step::DeleteStep;

    assert_eq!(PROCESS3D_DOCUMENT_GRANT_BYTES, 4_096);
    assert!(PROCESS3D_DOCUMENT_MAXIMUM_BYTES > PROCESS3D_DOCUMENT_GRANT_BYTES, "the document maximum is a validation, never the per-turn grant");
    let base = semio_s_artifact_process_process3d_model::standards::v1::subsets::any::io::text::snapshot::default_document();
    let step = ProcessStep { id: "step-retained".into(), label: "Retained Cut".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: WorkingSolid::Box { width: 0.1, depth: 0.1, height: 0.1 }, pose: semio_s_artifact_process_process3d_model::snapshot::Pose::default() } };
    let forward = Process3dMutation::CreateStep(CreateStep { index: 0, step: step.clone() });
    let (post, inverse) = prepare_process3d_document(&base, &forward).expect("create step prepares");
    assert!(matches!(forward, Process3dMutation::CreateStep(_)));
    assert!(!inverse.is_empty(), "a retained edit must carry its own inverse");
    assert!(post.step_payloads.iter().any(|payload| payload.id == step.id), "the post-state must come from the mutation's own diff");
    assert_ne!(post.step_payloads.len(), base.step_payloads.len());
    let duplicate = prepare_process3d_document(&post, &Process3dMutation::CreateStep(CreateStep { index: 0, step })).expect_err("a duplicate id is refused, not published as a no-op");
    assert!(duplicate.contains("refused by its own vocabulary"), "{duplicate}");
    let missing = prepare_process3d_document(&base, &Process3dMutation::DeleteStep(DeleteStep { id: "ghost".into() })).expect_err("a missing target is refused");
    assert!(missing.contains("refused by its own vocabulary"), "{missing}");
    let delete = Process3dMutation::DeleteStep(DeleteStep { id: "step-1".into() });
    let footprint = store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(&delete, process3d_mutation_retained_bytes(&delete).expect("retained bytes"));
    assert_eq!(footprint.work_items, store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS);
    assert!(footprint.is_admissible());
    let oversized = process3d_mutation_retained_bytes(&Process3dMutation::DeleteStep(DeleteStep { id: "x".repeat(PROCESS3D_DOCUMENT_TEXT_BYTES + 1) }));
    assert!(oversized.is_err(), "an id past the text envelope is rejected, never truncated");
}

#[test]
fn retained_resumable_extent_accepts_exact_byte_maximum_and_rejects_max_plus_one() {
    let config = Process3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let snapshot = retained_snapshot(0);
    let ceiling = semio_framework_plugin::CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES;
    let exact = Process3dCommand::SetContributions(set_contributions::SetContributions { json: "x".repeat(ceiling) });
    let rejected = Process3dCommand::SetContributions(set_contributions::SetContributions { json: "x".repeat(ceiling + 1) });
    assert_eq!(process3d_resumable_extent(&exact, &snapshot, &config, &interaction), Some(ceiling.div_ceil(PROCESS3D_SCAN_BYTES)));
    assert_eq!(process3d_resumable_extent(&Process3dCommand::EngagementInput(engagement_input::EngagementInput { value: "x".repeat(PROCESS3D_RETAINED_RAW_BYTES + 1) }), &snapshot, &config, &interaction), None, "a GESTURE past its own envelope is still refused");
    assert_eq!(process3d_resumable_extent(&rejected, &snapshot, &config, &interaction), None);
}

#[semio_framework_async_macros::async_test]
async fn retained_resumable_progress_checkpoint_identity_replay_and_close_are_exact() {
    let command = Process3dCommand::SetContributions(set_contributions::SetContributions { json: "x".repeat(4_096) });
    let snapshot = retained_snapshot(0);
    let config = Process3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let history = HistoryView::empty();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let operation = retained_operation();
    let extent = process3d_resumable_extent(&command, &snapshot, &config, &interaction).expect("extent");
    let mut uninterrupted = Process3dResumableCommandWork::new("setContributions", extent);
    for _ in 0..11 {
        assert!(matches!(
            uninterrupted
                .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: None, command: &command, snapshot: &snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation }, &mut semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(256, u64::MAX, UNBOUNDED_STEP_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut 0, &mut semio_framework_job::RetainedCloneProgress::default()))
                .expect("checkpoint prefix"),
            ArtifactCommandWorkStep::Progress { .. }
        ));
    }
    let mut checkpoint = [0u8; 40];
    assert_eq!(uninterrupted.checkpoint(&mut checkpoint, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).expect("checkpoint"), checkpoint.len());
    let mut wrong_tool = Process3dResumableCommandWork::new("engagementInput", extent);
    assert!(wrong_tool.restore(&checkpoint).is_err(), "a config command must not accept another tool's checkpoint");
    let mut replayed = Process3dResumableCommandWork::new("setContributions", extent);
    replayed.restore(&checkpoint).expect("restore");
    assert_eq!((replayed.cursor, replayed.digest), (uninterrupted.cursor, uninterrupted.digest));
    let (uninterrupted_progress, uninterrupted_emit) = drive_resumable_work(&mut uninterrupted, &command, &snapshot, &config, &interaction, &history);
    let (replayed_progress, replayed_emit) = drive_resumable_work(&mut replayed, &command, &snapshot, &config, &interaction, &history);
    assert_eq!(uninterrupted_progress, extent - 11);
    assert_eq!(replayed_progress, uninterrupted_progress);
    assert!(uninterrupted_emit.artifact_mutations.is_empty() && uninterrupted_emit.effects.is_empty());
    assert!(replayed_emit.artifact_mutations.is_empty() && replayed_emit.effects.is_empty());
    assert_eq!(uninterrupted_emit.config_mutations, vec![Process3dConfigMutation::SetContributions(Process3dConfigSetContributions{ json: "x".repeat(4_096) })]);
    assert_eq!(replayed_emit.config_mutations, uninterrupted_emit.config_mutations);
    let no_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 0, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 0 };
    assert_eq!(semio_framework_job::close_with_self_funded_context(no_grant, |cx| replayed.close_step(cx)).expect("original fixture close turn"), InteractiveJobCloseStep::Blocked);
    replayed.begin_close();
    assert_eq!(semio_framework_job::close_with_self_funded_context(no_grant, |cx| replayed.close_step(cx)).expect("original fixture close turn"), InteractiveJobCloseStep::Complete { progress: Default::default() });
    assert!(replayed.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn retained_bounded_and_resumable_maximum_steps_stay_below_eight_milliseconds() {
    let history = HistoryView::empty();
    let empty = retained_snapshot(0);
    let config = Process3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let operation = retained_operation();
    let bounded = [
        Process3dCommand::EngagementAbort(engagement_abort::EngagementAbort {}),
        Process3dCommand::SetCamera(set_camera::SetCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 }),
        Process3dCommand::LoadModelRequest(load_model_request::LoadModelRequest {}),
    ];
    for command in &bounded {
        let started = std::time::Instant::now();
        process3d_retained_reduce(command, &empty, &config, &history, &interaction, &hover, None, &operation).expect("bounded reducer");
        assert!(started.elapsed().as_micros() < 8_000, "bounded {} exceeded the interactive step ceiling", command.command_id());
    }

    let mut config_max = Process3dConfig::default();
    config_max.sun_color = "x".repeat(PROCESS3D_RETAINED_RAW_BYTES);
    let maximum = "x".repeat(PROCESS3D_RETAINED_RAW_BYTES);
    let fixtures = [
        (Process3dCommand::EngagementInput(engagement_input::EngagementInput { value: maximum.clone() }), Process3dConfig::default()),
        (Process3dCommand::ToggleSun(toggle_sun::ToggleSun {}), config_max.clone()),
        (Process3dCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: 1.0 }), config_max.clone()),
        (Process3dCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: 1.0 }), config_max.clone()),
        (Process3dCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: 1.0 }), config_max),
        (Process3dCommand::SetContributions(set_contributions::SetContributions { json: maximum }), Process3dConfig::default()),
    ];
    for (command, config) in fixtures {
        let interaction = protocol::InteractionState::default();
        let extent = process3d_resumable_extent(&command, &empty, &config, &interaction).expect("maximum extent");
        assert!(extent <= PROCESS3D_RETAINED_WORK_ITEMS, "{} fixture exceeds admission: {extent}", command.command_id());
        assert_eq!(extent, PROCESS3D_RETAINED_RAW_BYTES.div_ceil(PROCESS3D_SCAN_BYTES), "{} fixture no longer exercises exact maximum config admission", command.command_id());
        let mut work = Process3dResumableCommandWork::new(command.command_id(), extent);
        loop {
            let started = std::time::Instant::now();
            let step = work
                .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: None, command: &command, snapshot: &empty, config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation }, &mut semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(256, u64::MAX, UNBOUNDED_STEP_GRANT), semio_framework_job::root_cancel_token(), || Some(0), &mut 0, &mut semio_framework_job::RetainedCloneProgress::default()))
                .expect("maximum work step");
            assert!(started.elapsed().as_micros() < 8_000, "resumable {} exceeded the interactive step ceiling", command.command_id());
            if matches!(step, ArtifactCommandWorkStep::Complete(_) | ArtifactCommandWorkStep::CompleteWithEphemeral { .. }) {
                break;
            }
        }
    }
}

/// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every row's
/// wire keyword must be distinct.
#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), 31, "every Process3dCommand row must be covered by every_command()");
}

/// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — copied
/// verbatim from the pre-migration `Process3dCommand`/`command_id()` match (the two vocabularies
/// genuinely diverge for about a third of process3d's rows, unlike flow's single `setLocale`
/// exception, so this pins the full table rather than deriving it from a kebab-case guess).
#[semio_framework_async_macros::async_test]
async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
    let expected_wire_key = |id: &str| -> &'static str {
        match id {
            "setSnapshot" => "document",
            "setActiveExample" => "active-example",
            "addStep" => "add-step",
            "addWorkshopMachine" => "add-workshop-machine",
            "removeWorkshopMachine" => "remove-workshop-machine",
            "updateWorkshopMachine" => "update-workshop-machine",
            "removeStep" => "remove-step",
            "removeSelectedStep" => "remove-selected-step",
            "moveStep" => "move-step",
            "updateStep" => "update-step",
            "setStepEnabled" => "set-step-enabled",
            "setStock" => "stock",
            "patchInspector" => "patch-inspector",
            "setCursor" => "cursor",
            "stepCursor" => "step-cursor",
            "stepCursorBack" => "step-cursor-back",
            "stepCursorForward" => "step-cursor-forward",
            "engagementSubmit" => "engagement-submit",
            "worldPointerDown" => "world-pointer-down",
            "worldFaceDragEnd" => "world-face-drag-end",
            "importModelFile" => "import-model-file",
            "engagementInput" => "engagement-input",
            "engagementAbort" => "engagement-abort",
            "setCamera" => "camera",
            "toggleSun" => "toggle-sun",
            "setSunAzimuth" => "sun-azimuth",
            "setSunElevation" => "sun-elevation",
            "setSunIntensity" => "sun-intensity",
            "setContributions" => "contributions",
            "exportModel" => "export-model",
            "loadModelRequest" => "load-model-request",
            other if other == SET_ACTIVE_UTILITY_ACTION_ID => "active-utility",
            other => panic!("no expected wire key recorded for command id {other} — add it to this table"),
        }
    };
    for command in every_command() {
        let id = command.command_id();
        let printed = protocol::OpText::print_op(&command);
        assert_eq!(printed.split(' ').next().unwrap_or_default(), expected_wire_key(id), "wire keyword drifted for command {id}: {printed:?}");
    }
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub(super) fn every_command() -> Vec<Process3dCommand> {
    vec![
        Process3dCommand::SetDocument(set_snapshot::SetDocument { json: semio_framework_pack_json::to_json_string(&semio_s_artifact_process_process3d_model::snapshot::empty_process3d_snapshot()) }),
        Process3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: PROCESS3D_EXAMPLE_PLATE.into() }),
        Process3dCommand::AddStep(add_step::AddStep { measure: Some("cut".into()), machine_id: None, capability_id: None, position: Some([1.0, 2.0, 3.0]) }),
        Process3dCommand::AddWorkshopMachine(add_workshop_machine::AddWorkshopMachine { catalog_id: "wood".into(), machine_id: "circularSaw".into() }),
        Process3dCommand::RemoveWorkshopMachine(remove_workshop_machine::RemoveWorkshopMachine { id: "circularSaw".into() }),
        Process3dCommand::UpdateWorkshopMachine(update_workshop_machine::UpdateWorkshopMachine {
            machine: WorkshopMachine { id: "circularSaw".into(), label: "Circular Saw".into(), icon_id: "scissors".into(), catalog_id: Some("wood".into()), capabilities: vec![] },
        }),
        Process3dCommand::RemoveStep(remove_step::RemoveStep { id: "cut-1".into() }),
        Process3dCommand::RemoveSelectedStep(remove_selected_step::RemoveSelectedStep {}),
        Process3dCommand::MoveStep(move_step::MoveStep { id: "cut-1".into(), index: 2 }),
        Process3dCommand::UpdateStep(update_step::UpdateStep {
            step_json: semio_framework_pack_json::to_json_string(&ProcessStep {
                id: "cut-1".into(),
                label: "Cut".into(),
                enabled: true,
                origin: None,
                measure: ProcessMeasure::Cut { tool: WorkingSolid::Box { width: 0.1, depth: 0.1, height: 0.1 }, pose: semio_s_artifact_process_process3d_model::snapshot::Pose::default() },
            }),
        }),
        Process3dCommand::SetStepEnabled(set_step_enabled::SetStepEnabled { id: "cut-1".into(), enabled: false }),
        Process3dCommand::SetStock(set_stock::SetStock { kind: "cylinder".into() }),
        Process3dCommand::PatchInspector(patch_inspector::PatchInspector { target: "beam".into(), field: "width".into(), number: Some(1.5), text: None }),
        Process3dCommand::SetCursor(set_cursor::SetCursor { value: Some(3) }),
        Process3dCommand::StepCursor(step_cursor::StepCursor { delta: -1 }),
        Process3dCommand::StepCursorBack(step_cursor_back::StepCursorBack {}),
        Process3dCommand::StepCursorForward(step_cursor_forward::StepCursorForward {}),
        Process3dCommand::EngagementSubmit(engagement_submit::EngagementSubmit {}),
        Process3dCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { position: [1.0, 2.0, 3.0] }),
        Process3dCommand::WorldFaceDragEnd(world_face_drag_end::WorldFaceDragEnd { normal: [0.0, 0.0, 1.0], start_point: [0.5, 0.5, 1.0], distance: -0.5, face_extent: Some([1.0, 1.0]) }),
        Process3dCommand::ImportModelFile(import_model_file::ImportModelFile { name: "beam.step".into(), payload: "data:application/octet-stream;base64,AAAA".into() }),
        Process3dCommand::EngagementInput(engagement_input::EngagementInput { value: "cut".into() }),
        Process3dCommand::EngagementAbort(engagement_abort::EngagementAbort {}),
        Process3dCommand::SetCamera(set_camera::SetCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 }),
        Process3dCommand::ToggleSun(toggle_sun::ToggleSun {}),
        Process3dCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: 90.0 }),
        Process3dCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: 45.0 }),
        Process3dCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: 1.0 }),
        Process3dCommand::SetContributions(set_contributions::SetContributions { json: "[]".into() }),
        Process3dCommand::ExportModel(export_model::ExportModel { format: "step".into() }),
        Process3dCommand::LoadModelRequest(load_model_request::LoadModelRequest {}),
    ]
}

/// 🌉️ Every Process action emitted by React or wgpu must enter the same closed typed command
/// vocabulary as native typed callers; undeclared strings fail at this single boundary.
#[semio_framework_async_macros::async_test]
async fn command_from_action_covers_every_declared_action_and_rejects_unknown_ones() {
    artifact_app_laws::assert_declared_actions_bridge_to_commands::<EditorApp<Process3dPlayApp>>(process3d_app_manifest_for_tests).await;
    assert!(Process3dPlayApp::command_from_action("nonsense", None, None).is_err());
}

/// 🧾️ What the lane RETAINS is the distilled roster, never the host's whole contribution pack: an
/// entry addressed to this app's `process.machines` topic survives, everything else is dropped
/// (`installable_contributions`, the same discipline `🪵️sourcing` applies to its own config lane).

/// 🖱️ The interaction payload shape that regressed retains its identifier through the transport
/// bridge instead of being dropped into ad-hoc host state.
/// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): `setHover`/`contextMenuAt` were
/// app-owned selection/hover commands, deleted along with `Process3dConfig::selected_id`/
/// `hovered_id` — hover/selection now decode through the framework's auto-injected
/// `interactionHover`/`interactionSelect` verbs instead of this app's own command vocabulary.
#[semio_framework_async_macros::async_test]
async fn interaction_actions_decode_into_typed_commands() {
    assert_eq!(
        Process3dPlayApp::command_from_action("setActiveExample", Some((DslValue::from(&serde_json::json!({ "exampleId": PROCESS3D_EXAMPLE_PLATE }))).clone()), None).expect("example bridge"),
        Process3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: PROCESS3D_EXAMPLE_PLATE.into() })
    );
}

/// 📄️ Example switching exercises the complete registry-backed action path and emits the
/// architecture's sanctioned whole-document load effect for the requested fixture.
#[semio_framework_async_macros::async_test]
async fn registry_backed_example_action_emits_the_requested_document() {
    let mut app = app_with_registry();
    let result = action(&mut app, "setActiveExample", Some(&DslValue::from(&serde_json::json!({ "exampleId": PROCESS3D_EXAMPLE_PLATE }))));
    let Effect::LoadDocument { pack, .. } = result.requested_effects.first().expect("example action must load a document") else {
        panic!("expected a LoadDocument effect");
    };
    let loaded = <Process3dSnapshot as ArtifactPack>::decode_pack(pack).expect("decode example document");
    assert_eq!(loaded, semio_s_artifact_process_process3d_model::standards::v1::subsets::any::io::text::snapshot::plate_document());
}

/// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): `worldPick` (a pure selection-setting
/// command) is deleted along with `Process3dConfig::selected_face_id` — object/face picking now
/// decodes through the framework's auto-injected `interactionSelect` verb, and the rendered
/// world3d selection JSON carries no live selection ids anymore (see
/// `🎭️modes/✏️edit/🪟️windows/🪚️workpiece`'s `process3d_selection_json` doc comment).
#[semio_framework_async_macros::async_test]
async fn world3d_render_carries_no_stale_selection_json_fields() {
    let mut app = app();
    let rendered = render_body(&mut app, PROCESS_3D_PLAY_BODY_MAIN);
    assert!(!rendered.contains("componentIds"), "componentIds is no longer emitted by the render boundary: {rendered}");
}

/// 🖱️ A world right-click has an app-owned menu to request; the host no longer falls through to
/// an empty default.
#[semio_framework_async_macros::async_test]
async fn world_context_menu_exposes_process_commands() {
    let mut app = app_with_registry();
    let request = ContextMenuRequest {
        menu: UiMenuRef { id: "window".into(), args: None },
        surface: Some(ContextMenuSurfaceTarget { surface_id: "process.play".into(), kind: "world3d".into(), hits: Vec::new(), selection: Vec::new(), text: None }),
        window_instance_id: None,
        point: None,
    };
    let menu = app.context_menu(&request, &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await;
    let ids: Vec<&str> = menu.iter().map(|item| item.id.as_str()).collect();
    assert!(ids.contains(&"addStep"), "right-click menu must expose the primary Process command: {ids:?}");
    assert!(ids.contains(&"undo") && ids.contains(&"redo"), "right-click menu must expose history commands: {ids:?}");
}

/// 🕹️ The interaction-view threading law for this app's menu: `removeSelectedStep` is a verb ON the
/// `"geometry"` selection, so it exists exactly when that framework-owned domain is non-empty. Both
/// arms go through the same `process3d_context_menu_items` funnel the two trait entry points share.
#[semio_framework_async_macros::async_test]
async fn the_context_menu_gates_the_destructive_row_on_the_geometry_selection() {
    let registry = AppActionRegistry::from_definition(&create_process3d_app());
    let ids = |selected: &[String]| -> Vec<String> { process3d_context_menu_items(&registry, &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native), selected).into_iter().map(|item| item.id).collect() };
    let empty = ids(&[]);
    assert!(!empty.iter().any(|id| id == "removeSelectedStep"), "an empty geometry selection must not offer the destructive row: {empty:?}");
    assert!(empty.iter().any(|id| id == "addStep"), "the selection-free rows must survive an empty selection: {empty:?}");
    let selected = ids(&["step-1".to_string()]);
    assert!(selected.iter().any(|id| id == "removeSelectedStep"), "a non-empty geometry selection must offer the destructive row: {selected:?}");
}
//#endregion 🔖️CommandSurface
