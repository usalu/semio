
use super::*;
use crate::editor::process3d::testkit::{action, app, app_with_registry, dispatch, dispatch_with_utility, main_window_measures, process3d_app_manifest_for_testkit, render as render_body};
use semio_framework_plugin::{ContextMenuRequest, ContextMenuSurfaceTarget, EditorApp, HistoryView, PluginApp, SET_ACTIVE_UTILITY_ACTION_ID, UiMenuRef, testkit};

fn production_initial_snapshot(label: &str) -> Process3dSnapshot {
    let mut snapshot = crate::empty_process3d_snapshot();
    snapshot.stock_label = label.into();
    snapshot.workshop.machines.push(WorkshopMachine { id: "machine".into(), label: "Original Machine".into(), icon_id: "original-tool".into(), catalog_id: Some("original-catalog".into()), capabilities: Vec::new() });
    snapshot
}

fn production_semantic_digest(snapshot: &Process3dSnapshot) -> [u8; 32] {
    let mut digest = store::ArtifactStoreInitializationDigest::new(b"process3d.production-law.semantic");
    digest.observe(&snapshot.encode_pack());
    digest.finish()
}

fn production_envelope_wire(label: &str) -> (Vec<u8>, Process3dSnapshot, [u8; 32]) {
    let snapshot = production_initial_snapshot(label);
    let snapshot_pack = snapshot.encode_pack();
    let snapshot_hex = snapshot_pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let mutations = crate::spr::process3d_all_retained_mutation_fixtures_for_test();
    assert_eq!(mutations.len(), 16, "production ingress carries every Process3d mutation variant");
    let mutation_hex: Vec<String> = mutations.iter().map(|mutation| crate::spr::encode_op(mutation).expect("deep Process3d mutation encoding").iter().map(|byte| format!("{byte:02x}")).collect()).collect();
    let mut expected = production_initial_snapshot(label);
    let expected_capability = match &mutations[11] {
        Process3dMutation::ReplaceMachineCapabilities(value) => value.new_capabilities[0].clone(),
        _ => unreachable!("fixed all-variant fixture order"),
    };
    let machine = expected.workshop.machines.first_mut().expect("production law initial machine");
    machine.label = "Renamed Machine".into();
    machine.icon_id = "drill".into();
    machine.capabilities = vec![expected_capability];
    if let Process3dMutation::MoveStock(value) = &mutations[12] {
        expected.stock_pose = value.new_pose.clone();
    }
    expected.stock_label = "Beam".into();
    if let Process3dMutation::ReplaceStockSolid(value) = &mutations[14] {
        expected.stock_solid = value.new_solid.clone();
    }
    expected.resolved_up_to = Some(7);
    let expected_digest = production_semantic_digest(&expected);
    let wire = serde_json::to_vec(&serde_json::json!({
        "schema": crate::PROCESS_3D_SCHEMA,
        "id": "process3d-production-mounted-law",
        "vcs": {
            "initialSnapshot": snapshot_hex,
            "edits": [{
                "id": "process3d-production-deep-edit",
                "actor": "process3d-production-law",
                "forwards": mutation_hex,
                "inverse": [],
                "sequenceNumber": 1,
                "startedAt": "1"
            }],
            "changes": [],
            "checkpoints": [],
            "alternatives": []
        },
        "editMessages": [],
        "conflicts": []
    }))
    .expect("schema-first Process3d production fixture envelope");
    let envelope = store::create_document_envelope(crate::PROCESS_3D_SCHEMA, "process3d-production-mounted-law", snapshot, None);
    let mut retirement = crate::spr::process3d_envelope_decode_owner_bundle().retire_envelope(envelope);
    for _ in 0..100_000 {
        match retirement.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("Process3d fixture envelope retirement") {
            store::SnapshotRetirementStep::Complete => {
                assert!(retirement.terminal_is_empty());
                drop(retirement);
                return (wire, expected, expected_digest);
            }
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES);
            }
            store::SnapshotRetirementStep::Blocked => panic!("unshared Process3d fixture envelope retirement blocked"),
        }
    }
    panic!("Process3d fixture envelope retirement did not reach terminal")
}

fn admit_production_envelope(app: &mut crate::editor::process3d::testkit::Process3dRawApp, wire: &[u8]) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle {
    let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
    let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("Process3d production ingress credits");
    crate::spr::process3d_admit_publication_authority(
        handle.operation,
        handle.generation,
        handle.generation.0,
        handle.generation.0,
        handle.generation.0,
        8_192,
        crate::spr::PROCESS3D_MOUNTED_OUTPUT_CHANNELS,
        crate::spr::PROCESS3D_MOUNTED_CONTROL_CREDITS,
    )
    .expect("Process3d production publication authority");
    for chunk in wire.chunks(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
        let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
        bytes[..chunk.len()].copy_from_slice(chunk);
        let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded Process3d production envelope page");
        app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("Process3d production envelope page admission failed: {fault:?}"));
    }
    assert!(app.seal_artifact_envelope_ingress(handle).expect("Process3d production envelope seal"));
    handle
}

fn drive_production_envelope(app: &mut crate::editor::process3d::testkit::Process3dRawApp, handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll {
    for _ in 0..200_000 {
        crate::spr::process3d_refresh_publication_authority(handle.operation, handle.generation, app.artifact_generation_now().0).expect("Process3d authority refresh immediately before production maintenance");
        PluginApp::maintenance_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("one Process3d production maintenance turn");
        let poll = app.advance_artifact_envelope_load(handle).expect("Process3d production load advancement");
        if matches!(poll, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Cancelled | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault) {
            return poll;
        }
        std::thread::yield_now();
    }
    panic!("Process3d production envelope load did not reach terminal")
}

/// 🔐️ LAW: the real `VcsArtifactApp` maintenance branch cannot swap around Process3d's
/// atomic authority hook; accepted and every hostile lease retire their displaced/candidate owner.
#[semio_framework_async_macros::async_test]
async fn vcs_artifact_app_production_maintenance_swap_is_authoritative_and_fail_closed() {
    let accepted_label = "accepted-production-swap";
    let mut accepted = crate::editor::process3d::testkit::unseeded_app_with_registry();
    let base_generation = accepted.artifact_generation_now();
    let (accepted_wire, expected_snapshot, expected_digest) = production_envelope_wire(accepted_label);
    let accepted_handle = admit_production_envelope(&mut accepted, &accepted_wire);
    assert_eq!(drive_production_envelope(&mut accepted, accepted_handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
    assert_eq!(accepted.artifact_generation_now().0, base_generation.0 + 1);
    let accepted_snapshot = accepted.snapshot().expect("accepted Process3d production snapshot");
    assert_eq!(&accepted_snapshot, &expected_snapshot, "real maintenance replay must publish the complete deep semantic state");
    assert_eq!(production_semantic_digest(&accepted_snapshot), expected_digest, "real maintenance replay must publish the deterministic semantic digest");
    let machine = accepted_snapshot.workshop.machines.first().expect("deep production machine");
    assert_eq!((machine.id.as_str(), machine.label.as_str(), machine.icon_id.as_str()), ("machine", "Renamed Machine", "drill"));
    let capability = machine.capabilities.first().expect("deep production capability");
    assert!(matches!(&capability.recipe, MeasureRecipe::BoxAttach { width, depth, height } if (width.as_str(), depth.as_str(), height.as_str()) == ("width", "depth", "height")));
    assert_eq!((capability.parameters.len(), capability.rules.len(), accepted_snapshot.stock_label.as_str(), accepted_snapshot.resolved_up_to), (3, 2, "Beam", Some(7)));
    assert!(accepted.acknowledge_artifact_store_replacement(accepted_handle).expect("accepted Process3d terminal ACK"));
    assert!(crate::spr::process3d_release_publication_authority(accepted_handle.operation, accepted_handle.generation));

    use crate::spr::Process3dPublicationHostile::{Missing, WrongBase, WrongGeneration, WrongOperation, WrongParent};
    for (hostile, expected_code) in [
        (Missing, "process3d-publication.authority-missing"),
        (WrongOperation, "process3d-publication.wrong-operation"),
        (WrongGeneration, "process3d-publication.wrong-generation"),
        (WrongBase, "process3d-publication.wrong-base"),
        (WrongParent, "process3d-publication.wrong-parent"),
    ] {
        let mut app = crate::editor::process3d::testkit::unseeded_app_with_registry();
        let last_valid = app.snapshot().expect("last-valid Process3d snapshot");
        let last_valid_digest = production_semantic_digest(&last_valid);
        let base_generation = app.artifact_generation_now();
        let (hostile_wire, hostile_snapshot, hostile_digest) = production_envelope_wire("rejected-production-candidate");
        assert_eq!(production_semantic_digest(&hostile_snapshot), hostile_digest);
        let handle = admit_production_envelope(&mut app, &hostile_wire);
        crate::spr::process3d_arm_publication_hostile(handle.operation, hostile);
        assert_eq!(drive_production_envelope(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
        assert_eq!(crate::spr::process3d_take_publication_hostile_observed(handle.operation), Some(expected_code), "removing or bypassing the real validator must fail this law");
        assert_eq!(app.artifact_generation_now(), base_generation);
        let retained = app.snapshot().expect("last-valid snapshot after rejected candidate");
        assert_eq!(retained, last_valid);
        assert_eq!(production_semantic_digest(&retained), last_valid_digest, "hostile candidate must not change the last-valid digest");
        assert!(app.acknowledge_artifact_store_replacement(handle).expect("rejected Process3d terminal ACK after candidate retirement"));
        assert!(crate::spr::process3d_release_publication_authority(handle.operation, handle.generation));
    }
}

//#region 🔖️CommandSurface
fn retained_snapshot(machine_count: usize) -> Process3dSnapshot {
    let mut snapshot = crate::empty_process3d_snapshot();
    snapshot.workshop.machines.clear();
    snapshot.step_payloads.clear();
    snapshot.tool_solids.clear();
    snapshot.workshop.machines.extend((0..machine_count).map(|index| WorkshopMachine { id: format!("retained-{index}"), label: String::new(), icon_id: String::new(), catalog_id: None, capabilities: Vec::new() }));
    snapshot
}

fn retained_operation() -> AppOperationContext {
    AppOperationContext { app_instance_id: 1, parent_document_id: "process3d-retained-test".into(), operation_id: 2, generation: 3, canonical_base_revision: [4; 32] }
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
        match work.step(&semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config, history, interaction, hover: &hover, context: None, operation: &operation }).expect("retained work step") {
            ArtifactCommandWorkStep::Replay { .. } | ArtifactCommandWorkStep::Progress { .. } => progress += 1,
            ArtifactCommandWorkStep::Complete(emit) => return (progress, emit),
            ArtifactCommandWorkStep::CompleteWithEphemeral { emit, .. } => return (progress, emit),
        }
    }
}

#[test]
fn retained_route_dispositions_are_exact_and_exhaustive() {
    use semio_framework::{ToolCancellationPolicy, ToolExecutionShape};

    let mut ids = PROCESS3D_BOUNDED_TOOL_IDS.iter().chain(PROCESS3D_RESUMABLE_TOOL_IDS).copied().collect::<Vec<_>>();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), 33);
    assert_eq!(PROCESS3D_BOUNDED_TOOL_IDS.len(), 25);
    assert_eq!(PROCESS3D_RESUMABLE_TOOL_IDS.len(), 8);
    assert!(ids.iter().all(|tool_id| process3d_command_disposition(tool_id).is_some()));
    assert_eq!(process3d_command_disposition("nonsense"), None);
    assert_eq!(<Process3dPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), 33);
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
        .flat_map(|window| window.actions.iter().map(|action| (action.id.as_str(), action.semantics.execution.interactive_job)))
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
    assert_eq!(lanes.len(), 33);
    assert!(<Process3dPlayApp as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().is_some(), "the Artifact lane is rejected outright without a document one-item preparation factory");
    assert!(<Process3dPlayApp as ArtifactEditor>::build_config_store_one_item_preparation_factory().is_some());
}

/// 📬️ The document lane prepares through the mutation's OWN `diff`/`inverse`, keeps the fixed
/// per-turn grant separate from the document's validation maximum (a measured base compared against
/// the host's 4 KiB grant would stall a large document forever), and refuses an `Error`/`Fatal`
/// outcome instead of publishing its forced-empty diff as a no-op edit.
#[test]
fn document_preparation_uses_the_mutations_own_semantics_and_a_fixed_per_turn_grant() {
    use crate::mutations::create_step::CreateStep;
    use crate::mutations::delete_step::DeleteStep;

    assert_eq!(PROCESS3D_DOCUMENT_GRANT_BYTES, 4_096);
    assert!(PROCESS3D_DOCUMENT_MAXIMUM_BYTES > PROCESS3D_DOCUMENT_GRANT_BYTES, "the document maximum is a validation, never the per-turn grant");
    let base = crate::schema::default_document();
    let step = ProcessStep { id: "step-retained".into(), label: "Retained Cut".into(), enabled: true, origin: None, measure: ProcessMeasure::Cut { tool: WorkingSolid::Box { width: 0.1, depth: 0.1, height: 0.1 }, pose: crate::Pose::default() } };
    let (post, inverse, forward) = prepare_process3d_document(&base, Process3dMutation::CreateStep(CreateStep { index: 0, step: step.clone() })).expect("create step prepares");
    assert!(matches!(forward, Process3dMutation::CreateStep(_)));
    assert!(!inverse.is_empty(), "a retained edit must carry its own inverse");
    assert!(post.step_payloads.iter().any(|payload| payload.id == step.id), "the post-state must come from the mutation's own diff");
    assert_ne!(post.step_payloads.len(), base.step_payloads.len());
    let duplicate = prepare_process3d_document(&post, Process3dMutation::CreateStep(CreateStep { index: 0, step })).expect_err("a duplicate id is refused, not published as a no-op");
    assert!(duplicate.contains("refused by its own vocabulary"), "{duplicate}");
    let missing = prepare_process3d_document(&base, Process3dMutation::DeleteStep(DeleteStep { id: "ghost".into() })).expect_err("a missing target is refused");
    assert!(missing.contains("refused by its own vocabulary"), "{missing}");
    let footprint = process3d_mutation_footprint(&Process3dMutation::DeleteStep(DeleteStep { id: "step-1".into() })).expect("footprint");
    assert_eq!(footprint.work_items, 1);
    assert!(footprint.is_admissible());
    let oversized = process3d_mutation_footprint(&Process3dMutation::DeleteStep(DeleteStep { id: "x".repeat(PROCESS3D_DOCUMENT_TEXT_BYTES + 1) }));
    assert!(oversized.is_err(), "an id past the text envelope is rejected, never truncated");
}

#[test]
fn retained_resumable_extent_accepts_exact_byte_maximum_and_rejects_max_plus_one() {
    let config = Process3dConfig::default();
    let interaction = protocol::InteractionState::default();
    let snapshot = retained_snapshot(0);
    let exact = Process3dCommand::SetContributions(set_contributions::SetContributions { json: "x".repeat(PROCESS3D_RETAINED_RAW_BYTES) });
    let rejected = Process3dCommand::SetContributions(set_contributions::SetContributions { json: "x".repeat(PROCESS3D_RETAINED_RAW_BYTES + 1) });
    assert_eq!(process3d_resumable_extent(&exact, &snapshot, &config, &interaction), Some(PROCESS3D_RETAINED_RAW_BYTES.div_ceil(PROCESS3D_SCAN_BYTES)));
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
                .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs { command: &command, snapshot: &snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation })
                .expect("checkpoint prefix"),
            ArtifactCommandWorkStep::Progress { .. }
        ));
    }
    let mut checkpoint = [0u8; 40];
    assert_eq!(uninterrupted.checkpoint(&mut checkpoint).expect("checkpoint"), checkpoint.len());
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
    assert_eq!(uninterrupted_emit.config_mutations, vec![Process3dConfigMutation::SetContributions { json: "x".repeat(4_096) }]);
    assert_eq!(replayed_emit.config_mutations, uninterrupted_emit.config_mutations);
    assert_eq!(replayed.close_step(0, 0), InteractiveJobCloseStep::Blocked);
    replayed.begin_close();
    assert_eq!(replayed.close_step(0, 0), InteractiveJobCloseStep::Complete);
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
                .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs { command: &command, snapshot: &empty, config: &config, history: &history, interaction: &interaction, hover: &hover, context: None, operation: &operation })
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
    assert_eq!(ids.len(), 33, "every Process3dCommand row must be covered by every_command()");
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
        Process3dCommand::SetDocument(set_snapshot::SetDocument { json: semio_framework_os_kernel::json::to_json_string(&crate::empty_process3d_snapshot()) }),
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
            step_json: semio_framework_os_kernel::json::to_json_string(&ProcessStep {
                id: "cut-1".into(),
                label: "Cut".into(),
                enabled: true,
                origin: None,
                measure: ProcessMeasure::Cut { tool: WorkingSolid::Box { width: 0.1, depth: 0.1, height: 0.1 }, pose: crate::Pose::default() },
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
    testkit::assert_declared_actions_bridge_to_commands::<EditorApp<Process3dPlayApp>>(process3d_app_manifest_for_testkit).await;
    assert!(Process3dPlayApp::command_from_action("nonsense", None).is_err());
}

#[test]
fn host_contributions_resolve_to_the_event_sourced_config_lane() {
    let mutation =
        <Process3dPlayApp as ArtifactEditor>::host_configuration_mutation("setContributions", Some(&DslValue::from(&serde_json::json!({ "json": "[{\"id\":\"process\"}]" })))).expect("host configuration").expect("process contribution mutation");
    assert_eq!(mutation, Process3dConfigMutation::SetContributions { json: "[{\"id\":\"process\"}]".into() });
    assert_eq!(<Process3dPlayApp as ArtifactEditor>::host_configuration_mutation("setCursor", None).expect("non-host action"), None);
}

/// 🖱️ The interaction payload shape that regressed retains its identifier through the transport
/// bridge instead of being dropped into ad-hoc host state.
/// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): `setHover`/`contextMenuAt` were
/// app-owned selection/hover commands, deleted along with `Process3dConfig::selected_id`/
/// `hovered_id` — hover/selection now decode through the framework's auto-injected
/// `interactionHover`/`interactionSelect` verbs instead of this app's own command vocabulary.
#[semio_framework_async_macros::async_test]
async fn interaction_actions_decode_into_typed_commands() {
    assert_eq!(
        Process3dPlayApp::command_from_action("setActiveExample", Some(&DslValue::from(&serde_json::json!({ "exampleId": PROCESS3D_EXAMPLE_PLATE })))).expect("example bridge"),
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
    assert_eq!(loaded, crate::schema::plate_document());
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
    let menu = app.context_menu(&request).await;
    let ids: Vec<&str> = menu.iter().map(|item| item.id.as_str()).collect();
    assert!(ids.contains(&"addStep"), "right-click menu must expose the primary Process command: {ids:?}");
    assert!(ids.contains(&"undo") && ids.contains(&"redo"), "right-click menu must expose history commands: {ids:?}");
}
//#endregion 🔖️CommandSurface

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let json = serde_json::to_string(&create_process3d_app()).expect("app definition json");
    assert!(json.contains(workpiece::PROCESS_3D_PLAY_WINDOW_MAIN), "window kind missing from the manifest");
    assert!(json.contains(edit::PROCESS3D_MODE_EDIT), "mode missing from the manifest");
    for body in [PROCESS_3D_PLAY_BODY_DOCUMENT, PROCESS_3D_PLAY_BODY_CATALOGUE, PROCESS_3D_PLAY_BODY_WORKSHOP, PROCESS_3D_PLAY_BODY_INSPECTION] {
        assert!(json.contains(body), "panel body {body} missing from the manifest");
    }
    assert!(json.contains("3d.process"), "artifact kind missing from the manifest");
}

#[semio_framework_async_macros::async_test]
async fn utility_registry_declares_four_flat_utilities_scoped_to_workpiece_window() {
    let definition = create_process3d_app();
    let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
    assert_eq!(utility_ids, ["select", "cut", "drill", "attach"], "utilities declared in registry order");
    assert!(definition.utilities.iter().all(|utility| utility.group.is_none()), "process's select/cut/drill/attach are the window's entire top-level utility set, so none carry a visual group",);
    let window = definition.window_kinds.iter().find(|window| window.id == workpiece::PROCESS_3D_PLAY_WINDOW_MAIN).expect("workpiece window");
    let scoped: Vec<&str> = window.utilities.iter().map(|utility| utility.as_str()).collect();
    assert_eq!(scoped, ["select", "cut", "drill", "attach"], "all four utilities scoped to the workpiece window kind");
}
//#endregion 🔖️ManifestSanity

//#region 🔖️IoTests
/// 🔤️ `AppIo.export_formats`/`import_formats` (unlike `ArtifactKindSpec`) have no `export_stdio_kinds`/
/// `import_stdio_kinds` string-id peer and are never read by `register_app_io`, so they stay empty
/// here in step with `artifact_kind()`'s own now-empty lists (see that fn's doc).
#[semio_framework_async_macros::async_test]
async fn process3d_io_mirrors_the_declared_artifact_kind() {
    let io = process3d_io();
    assert_eq!(io.document_schema, crate::PROCESS_3D_SCHEMA);
    assert_eq!(io.artifact.id, "3d.process");
    assert!(io.export_formats.is_empty());
    assert!(io.import_formats.is_empty());
}

/// 🔌️ WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE Wave 2 port recipe:
/// `geometry:in` and `brep:out` are declared with the right direction/kind/multiplicity.
#[semio_framework_async_macros::async_test]
async fn process3d_io_declares_geometry_in_and_brep_out_ports() {
    let io = process3d_io();
    let geometry_in = io.ports.iter().find(|port| port.id == "geometry:in").expect("geometry:in declared");
    assert_eq!(geometry_in.direction, semio_framework_plugin::MediaPortDirection::In);
    assert!(geometry_in.kind_id.is_none());
    assert!(!geometry_in.required);
    assert_eq!(geometry_in.multiplicity, semio_framework_plugin::PortMultiplicity::Many);

    let brep_out = io.ports.iter().find(|port| port.id == "brep:out").expect("brep:out declared");
    assert_eq!(brep_out.direction, semio_framework_plugin::MediaPortDirection::Out);
    assert_eq!(brep_out.kind_id.as_deref(), Some("3d.process"));
    assert!(!brep_out.required);
    assert_eq!(brep_out.multiplicity, semio_framework_plugin::PortMultiplicity::Many);
    assert_eq!(brep_out.media_type.class, MediaClass::ThreeD);
    assert_eq!(brep_out.media_type.form, MediaForm::Brep);
}
//#endregion 🔖️IoTests

//#region 🔖️CrossCutting
#[semio_framework_async_macros::async_test]
async fn labels_resolve_native_by_default_and_in_german() {
    let english = semio_framework_plugin::ViewModel { locale: semio_framework_plugin::Locale::En, ..Default::default() };
    assert_eq!(process3d_labels(&english).stock.as_str(), "Stock");
    let german = semio_framework_plugin::ViewModel { locale: semio_framework_plugin::Locale::De, ..Default::default() };
    assert_eq!(process3d_labels(&german).stock.as_str(), "Rohteil");
}

/// ↩️ Ticket `26/09/01/PROCESS-END-TO-END`: `AddStep` dispatches a real `CreateStep` mutation
/// against the durable `step_payloads` timeline — the minted `steps` flow child gains a node,
/// so its content-addressed handle changes. Undo restores the pre-add handle; redo re-applies it.
#[semio_framework_async_macros::async_test]
async fn undo_after_add_step_restores_the_steps_handle() {
    let mut app = app();
    let before = app.snapshot().expect("snapshot").steps.clone();
    testkit::assert_undo_redo_round_trip(
        &mut app,
        Process3dCommand::AddStep(add_step::AddStep { measure: Some("cut".into()), machine_id: None, capability_id: None, position: None }),
        |app| app.snapshot().expect("snapshot").steps == before,
        true,
        false,
    )
    .await;
}

#[semio_framework_async_macros::async_test]
async fn undo_after_add_workshop_machine_restores_previous_machine_count() {
    let mut app = app();
    testkit::assert_undo_redo_round_trip(
        &mut app,
        Process3dCommand::AddWorkshopMachine(add_workshop_machine::AddWorkshopMachine { catalog_id: "metal".into(), machine_id: "chopSaw".into() }),
        |app| app.snapshot().expect("snapshot").workshop.machines.len(),
        11,
        12,
    )
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
    let expected_solid = crate::brep_child_handle("stock", &crate::brep_snapshot_for_working_solid(&WorkingSolid::Cylinder { radius: 0.3, height: 1.0 }));
    assert_eq!(document.stock_solid, expected_solid, "setStock kind=cylinder must swap the stock solid to the real cylinder-content handle");
    let cleared_steps = crate::flow_child_handle(&crate::flow_snapshot_for_steps(&[], &Default::default()));
    assert_eq!(document.steps, cleared_steps, "swapping stock resets the step timeline");
}

/// 🌉️ `WorldPointerDown` dispatches `insert_step_mutations` → a real `CreateStep` mutation
/// against `step_payloads`, appended at the resolved-up-to cursor (or the timeline end). This
/// asserts the command dispatches a mutation for a real world-space click.
#[semio_framework_async_macros::async_test]
async fn world_pointer_down_dispatches_a_mutation_for_a_real_click() {
    let mut app = app();
    let result = dispatch_with_utility(&mut app, Process3dCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { position: [1.0, 2.0, 3.0] }), "cut");
    assert!(!result.mutations.is_empty(), "worldPointerDown must still dispatch a mutation for a real click");
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
    let first = dispatch_with_utility(&mut app, Process3dCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { position: [1.0, 0.0, 0.0] }), "cut");
    let second = dispatch_with_utility(&mut app, Process3dCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { position: [2.0, 0.0, 0.0] }), "cut");
    assert!(!first.mutations.is_empty() && !second.mutations.is_empty(), "each real click must dispatch its own mutation");
}

/// 🌉️ `WorldFaceDragEnd` dispatches `insert_step_mutations` → a real `CreateStep` mutation.
/// The kernel-replay math (cut/attach volume deltas) is covered directly against a literal
/// `ProcessWorkingScene` by `🧬️schema/💡️inferences`'s own
/// `drill_reduces_volume_below_stock`/`attach_increases_volume_above_stock` tests; these two
/// assert only that the command dispatches a mutation for a real face-drag gesture.
#[semio_framework_async_macros::async_test]
async fn world_face_drag_end_cut_dispatches_a_mutation() {
    let mut app = app();
    let result = dispatch(&mut app, Process3dCommand::WorldFaceDragEnd(world_face_drag_end::WorldFaceDragEnd { normal: [0.0, 0.0, 1.0], start_point: [0.5, 0.5, 1.0], distance: -0.5, face_extent: Some([1.0, 1.0]) }));
    assert!(!result.mutations.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn world_face_drag_end_attach_dispatches_a_mutation() {
    let mut app = app();
    let result = dispatch(&mut app, Process3dCommand::WorldFaceDragEnd(world_face_drag_end::WorldFaceDragEnd { normal: [0.0, 0.0, 1.0], start_point: [0.5, 0.5, 1.0], distance: 0.5, face_extent: Some([0.2, 0.2]) }));
    assert!(!result.mutations.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn world_face_drag_end_ignored_while_a_placement_utility_is_active() {
    let mut app = app();
    let result = dispatch_with_utility(&mut app, Process3dCommand::WorldFaceDragEnd(world_face_drag_end::WorldFaceDragEnd { normal: [0.0, 0.0, 1.0], start_point: [0.5, 0.5, 1.0], distance: -0.5, face_extent: None }), "cut");
    assert!(result.mutations.is_empty(), "worldFaceDragEnd should be a no-operation while a placement utility is active, not the select utility");
}

#[semio_framework_async_macros::async_test]
async fn toggle_sun_round_trips_through_config_and_defaults_off() {
    let mut app = app();
    let measures = app.window_measures().await;
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
    let measures = app.window_measures().await;
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
    let result = dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: Some("cut".into()), machine_id: None, capability_id: None, position: None }));
    assert!(!result.mutations.is_empty());
}

//#region 🔖️MediaTests
#[semio_framework_async_macros::async_test]
async fn export_brep_out_returns_step_text_structured_payload() {
    semio_framework::register_format_descriptors(semio_s_artifact_stdio_step::formats().expect("STEP format descriptors")).await.expect("register stdio format descriptors");
    let document = crate::schema::default_document();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let media = Process3dPlayApp::export_media("brep:out", &doc).expect("export brep:out");
    assert_eq!(media.media_type.class, MediaClass::ThreeD);
    assert_eq!(media.media_type.form, MediaForm::Brep);
    match media.payload {
        MediaPayload::Structured { schema, json } => {
            assert_eq!(schema, "3d.process");
            assert!(!json.is_empty());
        }
        MediaPayload::Binary { .. } => panic!("expected a Structured payload"),
    }
}

#[semio_framework_async_macros::async_test]
async fn export_unknown_port_is_not_implemented() {
    let document = crate::schema::default_document();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    assert!(matches!(Process3dPlayApp::export_media("nonsense:out", &doc), Err(MediaError::NotImplemented)));
}

#[semio_framework_async_macros::async_test]
async fn import_geometry_in_rejects_unrecognized_schema() {
    let document = crate::schema::default_document();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let media = semio_framework_plugin::Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep }, payload: MediaPayload::Structured { schema: "unknown.schema".into(), json: "irrelevant".into() } };
    assert!(matches!(Process3dPlayApp::import_media("geometry:in", &media, &doc), Err(MediaError::Payload(port, _)) if port == "geometry:in"));
}
//#endregion 🔖️MediaTests

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
                ("machinesJson".to_string(), DslValue::String(semio_framework_os_kernel::json::to_json_string(&vec![machine]))),
            ]),
        )),
    };
    let json = dsl::json::to_json_string(&vec![entry]);
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
