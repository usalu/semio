use super::*;
use crate::editor::generation3d::testkit::{app, app_with_registry, drain_flow_eval_ticks, drain_flow_eval_ticks_with_view, preview_views};
use semio_framework_plugin::PluginApp;
use serde_json::json;

#[semio_framework_async_macros::async_test]
async fn preview_eval_exact_window_transient_isolates_and_resets_in_the_registered_app() {
    use crate::editor::generation3d::modes::edit::windows::preview::transient::Generation3dPreviewWindowTransientOwner;
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = Box::new(app_with_registry().await);
    let (left, right) = preview_views("generation3d-preview-left", "generation3d-preview-right");
    let config_before = app.config_pack().await.expect("Generation3d app config before preview evaluation");
    drain_flow_eval_ticks_with_view(&mut app, &left).await;
    let left_state = app.window_transient_snapshot(&left).expect("left preview transient snapshot").expect("left preview owner");
    let right_state = app.window_transient_snapshot(&right).expect("right preview transient snapshot").expect("right preview owner");
    assert!(left_state.get::<Generation3dPreviewWindowTransientOwner>().and_then(|state| state.preview_eval_text.as_deref()).is_some_and(|text| !text.is_empty()));
    assert!(right_state.get::<Generation3dPreviewWindowTransientOwner>().is_some_and(|state| state.preview_eval_text.is_none()));
    let config_after = app.config_pack().await.expect("Generation3d app config after preview evaluation");
    assert_eq!((config_after.pack, config_after.spr), (config_before.pack, config_before.spr));
    drain_flow_eval_ticks_with_view(&mut app, &right).await;
    assert!(app.window_transient_snapshot(&right).expect("right evaluated snapshot").and_then(|snapshot| snapshot.get::<Generation3dPreviewWindowTransientOwner>().cloned()).is_some_and(|state| state.preview_eval_text.is_some()));
    let document = app.document_pack().await.expect("Generation3d document before reload");
    app.load_document_pack(&document).await.expect("same document reload resets preview window transient");
    for view in [&left, &right] {
        assert!(app.window_transient_snapshot(view).expect("reset preview snapshot").and_then(|snapshot| snapshot.get::<Generation3dPreviewWindowTransientOwner>().cloned()).is_some_and(|state| state.preview_eval_text.is_none()));
    }
    semio_framework_plugin::testkit::close_registered_fixture_app(&mut app);
    eprintln!("[DEBUG] Generation3d registered runtime isolated two preview evaluations, preserved app config bytes, reset both ephemeral windows on document reload, and closed terminal-empty");
}
fn production_initial_snapshot(label: &str) -> Generation3dSnapshot {
    let mut snapshot = Generation3dSnapshot::default();
    snapshot.fixture.schema = label.into();
    for (id, text) in [("replace-target", "before replacement"), ("delete-target", "delete me"), ("move-target", "move me"), ("clear-target", "clear me")] {
        snapshot.fixture.widgets.push(semio_framework_artifact_flow_flow::Widget::InputNote { id: id.into(), text: text.into() });
    }
    snapshot.fixture.synapses.push(semio_framework_artifact_flow_flow::SynapseSpec { id: "update-synapse".into(), from: "replace-target".into(), to: "move-target".into(), from_port: "old".into(), to_port: "old".into() });
    snapshot.fixture.synapses.push(semio_framework_artifact_flow_flow::SynapseSpec { id: "disconnect-synapse".into(), from: "move-target".into(), to: "clear-target".into(), from_port: String::new(), to_port: String::new() });
    snapshot.fixture.layout.insert("move-target".into(), semio_framework_artifact_flow_flow::WidgetLayout { x: 1.0, y: 2.0 });
    snapshot.fixture.layout.insert("clear-target".into(), semio_framework_artifact_flow_flow::WidgetLayout { x: 3.0, y: 4.0 });
    for (id, name) in [("delete-generation", "Delete"), ("rename-generation", "Before Rename"), ("change-generation", "Change Value")] {
        snapshot.generation.cold_builder_mut().unwrap().generations.push(semio_framework_artifact_playbook_playbook::FormGeneration { id: id.into(), name: name.into(), values: Default::default() });
    }
    snapshot.generation.cold_builder_mut().unwrap().selected_generation_id = Some("rename-generation".into());
    snapshot
}

fn production_mutations() -> Vec<Generation3dMutation> {
    use crate::standards::v1::subsets::any::schema::mutations::*;
    let params = semio_framework_artifact_flow_flow::neural::Dictionary::new().insert("integer", semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Integer(7))).insert(
        "nested",
        semio_framework_artifact_flow_flow::neural::Value::Dictionary(
            semio_framework_artifact_flow_flow::neural::Dictionary::new().insert("text", semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::String("production".into()))),
        ),
    );
    vec![
        Generation3dMutation::CreateWidget(create_widget::CreateWidget {
            index: 0,
            widget: semio_framework_artifact_flow_flow::Widget::Neuron { id: "created-widget".into(), neuron_kind: "law".into(), params, input_ports: vec!["in".into()], output_ports: vec!["out".into()], preview: true },
        }),
        Generation3dMutation::UpdateWidget(update_widget::UpdateWidget {
            widget: semio_framework_artifact_flow_flow::Widget::Cluster { id: "replace-target".into(), name: "After Replacement".into(), tree: Default::default(), flow: Default::default() },
        }),
        Generation3dMutation::DeleteWidget(delete_widget::DeleteWidget { id: "delete-target".into() }),
        Generation3dMutation::ConnectSynapse(connect_synapse::ConnectSynapse {
            index: 0,
            synapse: semio_framework_artifact_flow_flow::SynapseSpec { id: "created-synapse".into(), from: "created-widget".into(), to: "replace-target".into(), from_port: "out".into(), to_port: "in".into() },
        }),
        Generation3dMutation::UpdateSynapse(update_synapse::UpdateSynapse {
            synapse: semio_framework_artifact_flow_flow::SynapseSpec { id: "update-synapse".into(), from: "replace-target".into(), to: "move-target".into(), from_port: "new-out".into(), to_port: "new-in".into() },
        }),
        Generation3dMutation::DisconnectSynapse(disconnect_synapse::DisconnectSynapse { id: "disconnect-synapse".into() }),
        Generation3dMutation::MoveWidget(move_widget::MoveWidget { id: "move-target".into(), layout: semio_framework_artifact_flow_flow::WidgetLayout { x: 31.0, y: -17.0 } }),
        Generation3dMutation::DeleteWidgetPosition(delete_widget_position::DeleteWidgetPosition { id: "clear-target".into() }),
        Generation3dMutation::UpdateCamera(update_camera::UpdateCamera { camera: semio_framework_artifact_flow_flow::CameraJson { x: 9.0, y: 8.0, zoom: 1.75 } }),
        Generation3dMutation::ChangeSchema(change_schema::ChangeSchema { new_schema: "flow.fixture.production-retained".into() }),
        Generation3dMutation::CreateGeneration(create_generation::CreateGeneration { generation: semio_framework_artifact_playbook_playbook::FormGeneration { id: "created-generation".into(), name: "Created".into(), values: Default::default() } }),
        Generation3dMutation::DeleteGeneration(delete_generation::DeleteGeneration { id: "delete-generation".into() }),
        Generation3dMutation::RenameGeneration(rename_generation::RenameGeneration { id: "rename-generation".into(), new_name: "After Rename".into() }),
        Generation3dMutation::ChangeGenerationValue(change_generation_value::ChangeGenerationValue {
            id: "change-generation".into(),
            question_id: "deep-answer".into(),
            new_value: serde_json::json!({"object": {"array": [1.0, false, "retained"]}}).into(),
        }),
    ]
}

fn production_hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut value = String::new();
    value.try_reserve_exact(bytes.len() * 2).expect("P3 production hex preflight");
    for byte in bytes {
        value.push(char::from(DIGITS[usize::from(byte >> 4)]));
        value.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
    }
    value
}

fn production_semantic_digest(snapshot: &Generation3dSnapshot) -> [u8; 32] {
    let mut digest = store::ArtifactStoreInitializationDigest::new(b"generation3d.production-law.semantic");
    digest.observe(&crate::standards::v1::subsets::any::schema::snapshot::binary::encode(snapshot));
    digest.finish()
}

fn production_envelope_wire(label: &str) -> (Vec<u8>, Generation3dSnapshot, [u8; 32]) {
    let snapshot = production_initial_snapshot(label);
    let mutations = production_mutations();
    assert_eq!(mutations.len(), 14, "production ingress carries every P3 mutation variant including delete-widget-position");
    let mut mutation_hex = Vec::new();
    mutation_hex.try_reserve_exact(mutations.len()).expect("P3 production mutation owner preflight");
    for mutation in &mutations {
        mutation_hex.push(production_hex(&crate::standards::v1::subsets::any::schema::mutations::binary::encode_op(mutation).expect("P3 production mutation encoding")));
    }
    let mut expected = production_initial_snapshot(label);
    crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_apply_retained_mutations_for_test(&mut expected, &mutations);
    let expected_digest = production_semantic_digest(&expected);
    let wire = serde_json::to_vec(&serde_json::json!({
        "schema": GENERATION_3D_SCHEMA,
        "id": "generation3d-production-mounted-law",
        "vcs": {
            "initialSnapshot": production_hex(&crate::standards::v1::subsets::any::schema::snapshot::binary::encode(&snapshot)),
            "edits": [{
                "id": "generation3d-production-all14-edit",
                "actor": "generation3d-production-law",
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
    .expect("schema-first P3 production fixture envelope");
    (wire, expected, expected_digest)
}

fn admit_production_envelope(app: &mut semio_framework_plugin::VcsArtifactApp<EditorApp<Generation3dPlayApp>>, wire: &[u8]) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle {
    let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
    let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("P3 production ingress credits");
    crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_admit_publication_authority(handle.operation, handle.generation, handle.generation.0, handle.generation.0, handle.generation.0, crate::standards::v1::subsets::any::schema::mutations::binary::Generation3dPublicationCredits { maximum_items: 8_192, maximum_output_pages: crate::standards::v1::subsets::any::schema::mutations::binary::GENERATION3D_MOUNTED_OUTPUT_CHANNELS, maximum_controls: crate::standards::v1::subsets::any::schema::mutations::binary::GENERATION3D_MOUNTED_CONTROL_CREDITS })
    .expect("P3 production publication authority");
    for chunk in wire.chunks(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
        let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
        bytes[..chunk.len()].copy_from_slice(chunk);
        let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded P3 production envelope page");
        app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("P3 production envelope page admission failed: {fault:?}"));
    }
    assert!(app.seal_artifact_envelope_ingress(handle).expect("P3 production envelope seal"));
    handle
}

fn drive_production_envelope(app: &mut semio_framework_plugin::VcsArtifactApp<EditorApp<Generation3dPlayApp>>, handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll {
    for _ in 0..300_000 {
        crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_refresh_publication_authority(handle.operation, handle.generation, app.artifact_generation_now().0)
            .expect("P3 authority refresh immediately before production maintenance");
        PluginApp::maintenance_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("one P3 production maintenance turn");
        let poll = app.advance_artifact_envelope_load(handle).expect("P3 production load advancement");
        if matches!(poll, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Cancelled | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault) {
            return poll;
        }
        std::thread::yield_now();
    }
    panic!("P3 production envelope load did not reach terminal");
}

/// 🔐️ LAW: non-empty P3D3 canonical ingress reaches the real VCS maintenance replacement,
/// and accepted, stale, ABA, and displaced stores remain owned until explicit terminal ACK/close.
#[semio_framework_async_macros::async_test]
async fn vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed() {
    let mut accepted = semio_framework_plugin::VcsArtifactApp::<EditorApp<Generation3dPlayApp>>::new(EditorApp::default()).await;
    let base_generation = accepted.artifact_generation_now();
    let (wire, expected, expected_digest) = production_envelope_wire("accepted-production-swap");
    let handle = admit_production_envelope(&mut accepted, &wire);
    assert_eq!(drive_production_envelope(&mut accepted, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
    assert_eq!(accepted.artifact_generation_now().0, base_generation.0 + 1);
    let snapshot = accepted.snapshot().expect("accepted P3 production snapshot");
    assert_eq!(&snapshot, &expected, "real maintenance must publish all P3 snapshot and all-14 replay fields");
    assert_eq!(production_semantic_digest(&snapshot), expected_digest);
    assert!(snapshot.fixture.layout.contains_key("move-target"));
    assert!(!snapshot.fixture.layout.contains_key("clear-target"), "3D-only delete-widget-position must survive retained replay");
    assert!(accepted.acknowledge_artifact_store_replacement(handle).expect("accepted P3 terminal ACK"));
    assert!(crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_release_publication_authority(handle.operation, handle.generation));

    use crate::standards::v1::subsets::any::schema::mutations::binary::Generation3dPublicationHostile::{Missing, WrongBase, WrongGeneration, WrongOperation, WrongParent};
    for (hostile, expected_code) in [
        (Missing, "generation3d-publication.authority-missing"),
        (WrongOperation, "generation3d-publication.wrong-operation"),
        (WrongGeneration, "generation3d-publication.wrong-generation"),
        (WrongBase, "generation3d-publication.wrong-base"),
        (WrongParent, "generation3d-publication.wrong-parent"),
    ] {
        let mut app = semio_framework_plugin::VcsArtifactApp::<EditorApp<Generation3dPlayApp>>::new(EditorApp::default()).await;
        let last_valid = app.snapshot().expect("last-valid P3 snapshot");
        let last_valid_digest = production_semantic_digest(&last_valid);
        let base_generation = app.artifact_generation_now();
        let (wire, _, _) = production_envelope_wire("rejected-production-candidate");
        let handle = admit_production_envelope(&mut app, &wire);
        crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_arm_publication_hostile(handle.operation, hostile);
        assert_eq!(drive_production_envelope(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
        assert_eq!(crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_take_publication_hostile_observed(handle.operation), Some(expected_code));
        assert_eq!(app.artifact_generation_now(), base_generation);
        let retained = app.snapshot().expect("last-valid P3 snapshot after rejected candidate");
        assert_eq!(production_semantic_digest(&retained), last_valid_digest);
        assert_eq!(retained, last_valid);
        assert!(app.acknowledge_artifact_store_replacement(handle).expect("rejected P3 terminal ACK after candidate retirement"));
        assert!(crate::standards::v1::subsets::any::schema::mutations::binary::generation3d_release_publication_authority(handle.operation, handle.generation));
    }
}

//#region 🔖️CommandSurface
#[test]
fn command_ids_are_unique_and_cover_every_row() {
    let _serial = test_support::lock();
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), 29, "every Generation3dCommand row must be covered by every_command()");
}

/// ⚖️ LAW: every one of the 29 declared `Generation3dCommand` rows is retained-owned by
/// `Generation3dBoundedCommandJobFactory`, with an exact, nonempty publication-lane contract —
/// the shape `ArtifactToolFactoryRegistry::register` itself enforces
/// (`🧰️framework/…/🔌️plugin/🦀️.rs:12736-12748`), asserted here so a future command addition that
/// forgets its retained-tool-id/publication-contract row fails this test instead of silently
/// reintroducing `interactive-job.missing-owned-reducer` at dispatch. Mirrors generation2d's own
/// `retained_route_dispositions_are_exact_and_exhaustive` (`…/generation2d/…/✏️editor/🦀️.rs:1033`).
#[test]
fn retained_route_dispositions_are_exact_and_exhaustive() {
    use semio_framework::{ToolCancellationPolicy, ToolExecutionShape};
    use semio_framework_plugin::ArtifactOwnedToolJobFactory;
    let _serial = test_support::lock();
    assert_eq!(GENERATION3D_RETAINED_TOOL_IDS.len(), 29);
    assert_eq!(<Generation3dPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), 29);
    assert_eq!(Generation3dBoundedCommandJobFactory::PUBLICATION_CONTRACTS.len(), 29);
    assert_eq!(generation3d_bounded_contract().shape, ToolExecutionShape::BoundedFirstStep);
    assert_eq!(generation3d_bounded_contract().cancellation, ToolCancellationPolicy::PerOperation);
    assert!(GENERATION3D_RETAINED_TOOL_IDS.iter().all(|tool_id| Generation3dBoundedCommandJobFactory::PUBLICATION_CONTRACTS.iter().any(|contract| contract.tool_id == *tool_id)));
    let mut sorted_ids = GENERATION3D_RETAINED_TOOL_IDS.to_vec();
    sorted_ids.sort_unstable();
    sorted_ids.dedup();
    assert_eq!(sorted_ids.len(), GENERATION3D_RETAINED_TOOL_IDS.len(), "duplicate retained tool ids in {GENERATION3D_RETAINED_TOOL_IDS:?}");
    for command in every_command() {
        assert!(GENERATION3D_RETAINED_TOOL_IDS.contains(&command.command_id()), "command {} is not owned by Generation3dBoundedCommandJobFactory", command.command_id());
    }
}

async fn drive_preview_operation(app: &mut semio_framework_plugin::VcsArtifactApp<EditorApp<Generation3dPlayApp>>) -> Result<(u64, u64, u64), String> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    let mut artifact = 0;
    let mut config = 0;
    let mut transient = 0;
    while app.has_pending_typed_operations() {
        if std::time::Instant::now() >= deadline {
            return Err("Generation3d preview operation did not finish".into());
        }
        app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).map_err(|error| format!("{error:?}"))?;
        app.advance_typed_operation_publication().await.map_err(|error| format!("{error:?}"))?;
        while let Some(page) = app.take_typed_operation_result_page(1) {
            use semio_framework_plugin::app::TypedOperationResultLane;
            if page.lane == TypedOperationResultLane::Fault {
                return Err(format!("preview operation fault: {:?}", page.bytes()));
            }
            artifact += u64::from(page.lane == TypedOperationResultLane::Artifact);
            config += u64::from(page.lane == TypedOperationResultLane::Config);
            transient += u64::from(page.lane == TypedOperationResultLane::Transient);
            app.acknowledge_typed_operation_result(page.token).map_err(|error| format!("{error:?}"))?;
        }
        app.take_typed_operation_effect();
        app.take_typed_operation_event();
        app.take_typed_operation_ui_scope();
        std::thread::yield_now();
    }
    Ok((artifact, config, transient))
}

#[semio_framework_async_macros::async_test]
async fn generation_preview_is_one_app_transient_shared_by_two_generation_windows() {
    let mut app = app_with_registry().await;
    let result: Result<(), String> = async {
        let before_document = app.snapshot().map_err(|error| format!("{error:?}"))?.clone();
        let before_generation = app.ephemeral_snapshot().await.transient_generation;
        app.dispatch_typed(Generation3dCommand::AddGeneration(add_generation::AddGeneration {}), &semio_framework_plugin::testkit::meta("preview-owner")).await.map_err(|error| format!("{error:?}"))?;
        if drive_preview_operation(&mut app).await? != (1, 1, 1) {
            return Err("preview command did not publish artifact, selection config, and app transient exactly once".into());
        }
        if app.ephemeral_snapshot().await.transient_generation != before_generation + 1 {
            return Err("preview app transient generation did not advance exactly once".into());
        }
        if app.snapshot().map_err(|error| format!("{error:?}"))?.generation.as_state().generations.len() != before_document.generation.as_state().generations.len() + 1 {
            return Err("addGeneration did not preserve its document behavior".into());
        }
        let view = semio_framework_plugin::ViewModel {
            window_instances: vec![
                semio_framework::ViewWindowInstance { id: "preview-a".into(), window_kind_id: generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW.into() },
                semio_framework::ViewWindowInstance { id: "preview-b".into(), window_kind_id: generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW.into() },
            ],
            ..Default::default()
        };
        let mut rendered = Vec::new();
        for window_id in ["preview-a", "preview-b"] {
            let context = view.for_window_instance(window_id).ok_or("missing generation preview window")?;
            let tree = app.render(generate_preview::GENERATION_3D_PLAY_BODY_GENERATE_PREVIEW, None, &context).await.map_err(|error| format!("{error:?}"))?;
            rendered.push(semio_framework_plugin::testkit::project_and_retire_fixture_tree(tree).map_err(str::to_string)?);
        }
        if rendered[0] != rendered[1] {
            return Err("generation windows did not consume the same app-transient preview".into());
        }
        if include_str!("../../🎚️config/🧬️schema/🔣️.json").contains("generationPreviewText") {
            return Err("config schema still owns computed preview output".into());
        }
        Ok(())
    }
    .await;
    semio_framework_plugin::testkit::close_registered_fixture_app(&mut app);
    result.expect("Generation3d preview ownership runtime");
}

#[test]
fn every_command_round_trips_through_text_and_binary() {
    let _serial = test_support::lock();
    for command in every_command() {
        semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — pinned
/// explicitly per row since generation3d's wire keys frequently diverge from a mechanical
/// kebab-case of the command id (for example, `nodeGraphViewport` → `viewport`).
#[test]
fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
    let _serial = test_support::lock();
    let expected_keywords = [
        "active-example",
        "graph-edit",
        "delete-selection",
        "remove-widget",
        "move-node",
        "add-widget",
        "patch-flow-widgets",
        "reorganize",
        "translate-selection",
        "rotate-selection",
        "scale-selection",
        "add-generation",
        "remove-generation",
        "rename-generation",
        "update-generation-values",
        "viewport",
        "world-pointer-down",
        "graph-pointer-down",
        "lod-mode",
        "show-mode",
        "toggle-sun",
        "sun-azimuth",
        "sun-elevation",
        "sun-intensity",
        "camera",
        "select-generation",
        "flow-eval-tick",
        "flow-eval-resolve",
        "flow-tessellate-resolve",
    ];
    let commands = every_command();
    assert_eq!(commands.len(), expected_keywords.len(), "every_command() and expected_keywords must stay in the same declaration order");
    for (command, expected_keyword) in commands.iter().zip(expected_keywords) {
        let printed = protocol::OpText::print_op(command);
        assert_eq!(printed.split(' ').next().unwrap_or_default(), expected_keyword, "wire keyword drifted for command {}: {printed:?}", command.command_id());
    }
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub(super) fn every_command() -> Vec<Generation3dCommand> {
    vec![
        Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "hexagonal-mushroom-column".into() }),
        Generation3dCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: "[]".into() }),
        Generation3dCommand::DeleteSelection(delete_selection::DeleteSelection {}),
        Generation3dCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: "extrude".into() }),
        Generation3dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: "extrude".into(), x: 1.0, y: 2.0 }),
        Generation3dCommand::AddWidget(add_widget::AddWidget { kind: "inputSlider".into(), x: Some(10.0), y: None }),
        Generation3dCommand::PatchFlowWidgets(patch_flow_widgets::PatchFlowWidgets { widget_ids: vec!["height".into()], field: "value".into(), value: Some(9.5) }),
        Generation3dCommand::Reorganize(reorganize::Reorganize {}),
        Generation3dCommand::TranslateSelection(translate_selection::TranslateSelection { node_ids: vec!["extrude".into()], dx: 1.0, dy: 2.0, dz: 3.0 }),
        Generation3dCommand::RotateSelection(rotate_selection::RotateSelection { node_ids: vec!["extrude".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 1.5 }),
        Generation3dCommand::ScaleSelection(scale_selection::ScaleSelection { node_ids: vec!["extrude".into()], sx: 2.0, sy: 2.0, sz: 2.0 }),
        Generation3dCommand::AddGeneration(add_generation::AddGeneration {}),
        Generation3dCommand::RemoveGeneration(remove_generation::RemoveGeneration { id: "generation-1".into() }),
        Generation3dCommand::RenameGeneration(rename_generation::RenameGeneration { id: "generation-1".into(), name: "Renamed".into() }),
        Generation3dCommand::UpdateGenerationValues(update_generation_values::UpdateGenerationValues { generation_id: Some("generation-1".into()), question_id: "q1".into(), value: dsl::DslValue::float(5.0) }),
        Generation3dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { camera: semio_framework_artifact_flow_flow::CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } }),
        Generation3dCommand::WorldPointerDown(world_pointer_down::WorldPointerDown {}),
        Generation3dCommand::GraphPointerDown(graph_pointer_down::GraphPointerDown {}),
        Generation3dCommand::SetLodMode(set_lod_mode::SetLodMode { value: "coarse".into() }),
        Generation3dCommand::SetShowMode(set_show_mode::SetShowMode { value: "wireframe".into() }),
        Generation3dCommand::ToggleSun(toggle_sun::ToggleSun {}),
        Generation3dCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: 90.0 }),
        Generation3dCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: 45.0 }),
        Generation3dCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: 1.0 }),
        Generation3dCommand::SetCamera(set_camera::SetCamera { camera: crate::editor::generation3d::config::Generation3dPreviewCamera::default() }),
        Generation3dCommand::SelectGeneration(select_generation::SelectGeneration { id: "generation-1".into() }),
        Generation3dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick {}),
        Generation3dCommand::FlowEvalResolve(flow_eval_resolve::FlowEvalResolve { node_hash: 7, output_json: "{}".into() }),
        Generation3dCommand::FlowTessellateResolve(flow_tessellate_resolve::FlowTessellateResolve { node_hash: 9, output_json: "{}".into() }),
    ]
}
//#endregion 🔖️CommandSurface

#[semio_framework_async_macros::async_test]
async fn declared_actions_bridge_to_commands() {
    let _serial = test_support::lock();
    semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<EditorApp<Generation3dPlayApp>>(testkit::generation3d_app_manifest_for_testkit).await;
}

#[semio_framework_async_macros::async_test]
async fn registry_backed_editor_installs_every_declared_bounded_command_proof() {
    let _serial = test_support::lock();
    let _app = semio_framework_plugin::testkit::new_app_with_registry::<EditorApp<Generation3dPlayApp>>(testkit::generation3d_app_manifest_for_testkit).await;
}

#[test]
fn the_manifest_stitches_every_taxonomy_node() {
    let _serial = test_support::lock();
    let json = serde_json::to_string(&create_generation3d_app()).expect("app definition json");
    for id in [
        flow_window::GENERATION_3D_PLAY_WINDOW_MAIN,
        edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW,
        generations::GENERATION_3D_PLAY_WINDOW_GENERATIONS,
        form::GENERATION_3D_PLAY_WINDOW_GENERATE_FORM,
        generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW,
    ] {
        assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
    }
    for id in [edit::GENERATION_3D_PLAY_MODE_EDIT, generate::GENERATION_3D_PLAY_MODE_GENERATE] {
        assert!(json.contains(id), "mode {id} missing from the manifest");
    }
    assert!(json.contains("3d.generation"), "artifact kind missing from the manifest");
}

#[semio_framework_async_macros::async_test]
async fn each_example_loads_distinct_fixture_and_preview_geometry() {
    use crate::standards::v1::subsets::any::schema::*;
    use crate::widget_id;
    let _serial = test_support::lock();
    let examples = [
        PROCEDURAL_EXAMPLE_HEX_COLUMN,
        PROCEDURAL_EXAMPLE_RECT_EXTRUDE,
        PROCEDURAL_EXAMPLE_SPHERE_TORUS,
        PROCEDURAL_EXAMPLE_BOX_FILLET,
        PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE,
        PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE,
        PROCEDURAL_EXAMPLE_RECTANGLE_WIRE,
        PROCEDURAL_EXAMPLE_BOX_SHELL,
    ];
    let mut signatures = std::collections::BTreeSet::new();
    for example_id in examples {
        let mut app = app().await;
        app.dispatch_typed(Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: example_id.into() }), &semio_framework_plugin::testkit::meta("local")).await.expect("set example");
        let signature = format!("{:?}", app.snapshot().expect("snapshot").fixture.widgets.iter().map(|widget| widget_id(widget).to_string()).collect::<std::collections::BTreeSet<_>>());
        assert!(signatures.insert(signature.clone()), "duplicate fixture signature for {example_id}: {signature}");
    }
}

#[semio_framework_async_macros::async_test]
async fn refresh_pending_effects_arms_flow_eval_tick_chain() {
    let _serial = test_support::lock();
    let mut app = app().await;
    app.dispatch_typed(Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_SPHERE_TORUS.into() }), &semio_framework_plugin::testkit::meta("local"))
        .await
        .expect("set example");
    let effects = app.pending_effects().await;
    assert!(effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == "flowEvalTick")));
    drain_flow_eval_ticks(&mut app).await;
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trips_flow_graph_edits() {
    let _serial = test_support::lock();
    let mut app = app().await;
    let before = app.snapshot().expect("snapshot").fixture.widgets.len();
    semio_framework_plugin::testkit::assert_undo_redo_round_trip(
        &mut app,
        Generation3dCommand::AddWidget(add_widget::AddWidget { kind: "inputNote".into(), x: None, y: None }),
        |app| app.snapshot().expect("snapshot").fixture.widgets.len(),
        before,
        before + 1,
    )
    .await;
}

#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_widget_moves() {
    let _serial = test_support::lock();
    let widgets: Vec<String> = app().await.snapshot().expect("snapshot").fixture.widgets.iter().map(|widget| crate::widget_id(widget).to_string()).collect();
    assert!(widgets.len() >= 2, "default fixture needs two widgets for the test");
    let (w0, w1) = (widgets[0].clone(), widgets[1].clone());
    semio_framework_plugin::testkit::assert_two_instances_converge::<EditorApp<Generation3dPlayApp>, (Option<f64>, Option<f64>)>(
        "mem://generation3d-convergence",
        Generation3dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: w0.clone(), x: 111.0, y: 5.0 }),
        Generation3dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: w1.clone(), x: 222.0, y: 6.0 }),
        move |app| {
            let layout = &app.snapshot().expect("snapshot").fixture.layout;
            (layout.get(&w0).map(|entry| entry.x), layout.get(&w1).map(|entry| entry.x))
        },
    )
    .await;
}

#[semio_framework_async_macros::async_test]
async fn generation3d_labels_translate_catalogue_and_inspector_in_german() {
    let _serial = test_support::lock();
    let mut app = app().await;
    let view_state = semio_framework_plugin::ViewModel { locale: semio_framework_plugin::Locale::De, ..Default::default() };
    let catalogue = testkit::render_with_view(&mut app, catalogue_panel::GENERATION_3D_PLAY_BODY_CATALOGUE, &view_state).await;
    assert!(catalogue.contains("\"Elemente\""));
    let inspector = testkit::render_with_view(&mut app, inspection_panel::GENERATION_3D_PLAY_BODY_INSPECTION, &view_state).await;
    assert!(inspector.contains("Elemente:"));
}

/// 🕹️ The runtime graph-selection route persists through an exactly owned interaction store.
#[semio_framework_async_macros::async_test]
async fn generation3d_interaction_selection_owns_its_persisted_history() {
    let _serial = test_support::lock();
    let mut app = app_with_registry().await;
    let node_id = app.snapshot().expect("snapshot").fixture.widgets.first().map(crate::widget_id).expect("default fixture node").to_string();
    let targets = serde_json::to_string(&vec![semio_framework_plugin::InteractionTarget { granularity: "node".into(), id: node_id.clone() }]).expect("selection targets");
    let args: dsl::DslValue = serde_json::json!({ "domainId": "graph", "targets": targets, "merge": "replace", "method": "pick" }).into();
    app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&args), &semio_framework_plugin::testkit::meta("local")).await.expect("interaction selection persists");
    assert_eq!(app.interaction_state().await.selection.get("graph").map(|selection| selection.ids.as_slice()), Some([node_id].as_slice()));
}

/// 🕹️ `context_menu` carries no `InteractionView` (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM,
/// same discovered gap as `render`), so `has_selection` is always false now and the destructive
/// `delete-selection` row (conditioned on a real selection) never appears; this test now only pins
/// the disclosure budget.
#[semio_framework_async_macros::async_test]
async fn context_menu_grouped_disclosure_stays_within_budget() {
    let _serial = test_support::lock();
    let mut app = app_with_registry().await;
    let widgets: Vec<String> = app.snapshot().expect("snapshot").fixture.widgets.iter().map(|widget| crate::widget_id(widget).to_string()).collect();
    assert!(!widgets.is_empty(), "default fixture needs at least one widget for the test");
    let request = semio_framework_plugin::ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None }, surface: None, window_instance_id: None, point: None };
    let menu = app.context_menu(&request, &semio_framework_plugin::ViewModel::default()).await;
    assert!(menu.len() <= 9, "top-level menu (leaves+groups+separator) should stay within the row budget: {menu:?}");
    assert!(!menu.is_empty(), "grouped disclosure menu should not be empty");
}

#[semio_framework_async_macros::async_test]
async fn sun_measures_are_exposed_on_preview_windows() {
    let _serial = test_support::lock();
    let mut app = app().await;
    let measures = app.window_measures(&semio_framework_plugin::ViewModel::default()).await;
    assert!(measures.contains_key(edit_preview::GENERATION_3D_PLAY_WINDOW_PREVIEW));
    assert!(measures.contains_key(generate_preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW));
}

//#region 🔖️EngineComputeTests
/// 🧬️ Rehomed verbatim from the deleted `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) — these tests exercise
/// `PreviewPipeline`/`MeshBridge` functions above, all of which are app
/// behavior (they construct or take a [`Generation3dConfig`]), so the tests travel with them.
use semio_framework_ui::wgpu::kernel_3d_scene::{aabb_intersects_frustum, frustum_planes, transform_aabb, Camera3d, Instance3d, Vec3};
use std::sync::MutexGuard;

fn test_serial() -> MutexGuard<'static, ()> {
    test_support::lock()
}

/// 🌉️ Decodes one preview mesh record's `data` field (a `pack::json`/`serde_json::Value` fragment
/// off the wire) into `MeshData` through its first-party `FromValue` codec — `MeshData` no longer
/// derives `serde::Deserialize` in a non-`#[cfg(test)]` build of its own crate (`🏗️mesh-engine/🦀️.rs`'s
/// `#[cfg_attr(test, derive(Serialize, Deserialize))]` only activates inside that crate's OWN test
/// build, never for a downstream consumer like this one), so `serde_json::from_value` cannot reach
/// it here; round-trip through the wire text and `dsl::json::from_json_str` instead, exactly like
/// production's `mesh_data_for_preview_handle` (above) does.
fn mesh_data_from_json(value: &Value) -> semio_framework_plugin::MeshData {
    dsl::json::from_json_str(&serde_json::to_string(value).expect("mesh data json text")).expect("mesh data")
}

/// 🌉️ `Mesh3d` (a plain positions/normals/indices struct) no longer exists —
/// `semio_framework_ui::wgpu::kernel_3d_scene`'s mesh API is now a generation/revision-keyed write-token/lease
/// pair (`mesh3d_begin`/`mesh3d_write_vec3`/`mesh3d_seal`) meant for the shared render-owned mesh
/// arena, not for a one-off AABB check. This test only ever needed the bounding box of the raw
/// position buffer, so compute it directly instead of standing up a lease.
fn aabb_of_positions(positions: &[f32]) -> ([f32; 3], [f32; 3]) {
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for vertex in positions.chunks_exact(3) {
        for axis in 0..3 {
            min[axis] = min[axis].min(vertex[axis]);
            max[axis] = max[axis].max(vertex[axis]);
        }
    }
    (min, max)
}

fn preview_payload_from_evaluated_fixture(fixture: &semio_framework_artifact_flow_flow::FlowFixture, cfg: &Generation3dConfig) -> (String, String) {
    let mut host = FlowHost::from_fixture(fixture.clone());
    host.set_neuron_kind_infos_json(&semio_framework_os_flow::flow_neuron_kind_infos_json());
    let eval_json = host.evaluate().unwrap_or_default();
    preview_payload_from_eval(&eval_json, fixture, cfg)
}

#[test]
fn preview_payload_has_meshes_and_instances() {
    let _serial = test_serial();
    let projection = crate::standards::v1::subsets::any::schema::default_snapshot();
    let config = Generation3dConfig::default();
    let (meshes_json, instances_json) = preview_payload_from_evaluated_fixture(&projection.fixture, &config);
    assert_ne!(meshes_json, "[]", "meshes_json was empty");
    assert_ne!(instances_json, "[]", "instances_json was empty");
    let meshes: Vec<Value> = serde_json::from_str(&meshes_json).expect("meshes json");
    let instances: Vec<Value> = serde_json::from_str(&instances_json).expect("instances json");
    assert!(!meshes.is_empty());
    assert!(!instances.is_empty());
    for mesh in &meshes {
        let id = mesh.get("id").and_then(|value| value.as_str()).unwrap_or("");
        assert!(id.starts_with("eval-"), "mesh id must be tessellated eval handle, got {id}");
        let data = mesh_data_from_json(&mesh.get("data").cloned().unwrap_or_default());
        assert!(data.positions.len() >= 9, "mesh has too few positions");
        assert!(data.indices.len() >= 3, "mesh has too few indices");
        assert!(!data.edge_positions.is_empty(), "brep preview should include edge geometry");
    }
    let camera = Camera3d {
        position: Vec3::from_array([config.preview_camera.position[0] as f32, config.preview_camera.position[1] as f32, config.preview_camera.position[2] as f32]),
        target: Vec3::from_array([config.preview_camera.target[0] as f32, config.preview_camera.target[1] as f32, config.preview_camera.target[2] as f32]),
        up: Vec3::new(0.0, 0.0, 1.0),
        fov_y: config.preview_camera.fov as f32 * std::f32::consts::PI / 180.0,
        near: 0.1,
        far: 1000.0,
    };
    let view_proj = camera.view_proj(0.6);
    let planes = frustum_planes(view_proj);
    let mut visible = 0usize;
    for instance in instances {
        let mesh_id = instance.get("meshId").or_else(|| instance.get("mesh_id")).and_then(|value| value.as_str()).unwrap_or("eval-missing");
        let mesh = meshes.iter().find(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(mesh_id)).expect("mesh record");
        let data = mesh_data_from_json(&mesh.get("data").cloned().unwrap_or_default());
        let (mesh_aabb_min, mesh_aabb_max) = aabb_of_positions(&data.positions);
        let position = instance.get("position").and_then(|value| value.as_array()).map_or([0.0, 0.0, 0.0], |items| [items[0].as_f64().unwrap_or(0.0) as f32, items[1].as_f64().unwrap_or(0.0) as f32, items[2].as_f64().unwrap_or(0.0) as f32]);
        assert_eq!(position, [0.0, 0.0, 0.0], "preview instances stay in world space");
        let model = Instance3d::model_from_trs(position, [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0]);
        let (min, max) = transform_aabb(model, mesh_aabb_min, mesh_aabb_max);
        if aabb_intersects_frustum(&planes, min, max) {
            visible += 1;
        }
    }
    assert!(visible > 0, "no preview instances intersect camera frustum");
}

#[test]
fn document_from_mesh_returns_valid_default_snapshot() {
    let _serial = test_serial();
    let mesh = semio_framework_plugin::MeshData::default();
    let document = generation3d_document_from_mesh(&mesh).expect("dwg mesh import document");
    let projection: Generation3dSnapshot = <Generation3dSnapshot as protocol::FromValue>::from_value(protocol::json::to_dsl_value(&document)).expect("parseable projection");
    assert_eq!(projection.fixture.schema, "flow.fixture");
}

#[test]
fn generation3d_mesh_bridges_round_trip_through_obj_glb_stl_codecs() {
    let _serial = test_serial();
    use semio_framework_plugin::{GlbExporter, GlbImporter, MeshExporter, MeshImporter, ObjExporter, ObjImporter, StlExporter, StlImporter};
    let document_json: Value = serde_json::from_str(&dsl::json::to_json_string(&crate::standards::v1::subsets::any::schema::default_snapshot())).expect("projection json");
    let mesh = generation3d_mesh_from_document(&dsl::DslValue::from(&document_json)).expect("mesh from document");
    assert!(!mesh.positions.is_empty());

    let obj_bytes = ObjExporter.export(&mesh).expect("obj export");
    let obj_mesh = ObjImporter.import(&obj_bytes).expect("obj import");
    let obj_document = generation3d_document_from_mesh(&obj_mesh).expect("obj document from mesh");
    let _: Generation3dSnapshot = <Generation3dSnapshot as protocol::FromValue>::from_value(protocol::json::to_dsl_value(&obj_document)).expect("parseable obj projection");

    let glb_bytes = GlbExporter.export(&mesh).expect("glb export");
    let glb_mesh = GlbImporter.import(&glb_bytes).expect("glb import");
    let glb_document = generation3d_document_from_mesh(&glb_mesh).expect("glb document from mesh");
    let _: Generation3dSnapshot = <Generation3dSnapshot as protocol::FromValue>::from_value(protocol::json::to_dsl_value(&glb_document)).expect("parseable glb projection");

    let stl_bytes = StlExporter.export(&mesh).expect("stl export");
    let stl_mesh = StlImporter.import(&stl_bytes).expect("stl import");
    let stl_document = generation3d_document_from_mesh(&stl_mesh).expect("stl document from mesh");
    let _: Generation3dSnapshot = <Generation3dSnapshot as protocol::FromValue>::from_value(protocol::json::to_dsl_value(&stl_document)).expect("parseable stl projection");
}

#[test]
fn rectangle_wire_preview_emits_edge_only_mesh() {
    let _serial = test_serial();
    let projection = <Generation3dSnapshot as store::ArtifactDsl>::parse_dsl(crate::standards::v1::subsets::any::schema::snapshot::text::GENERATION3D_EXAMPLE_RECTANGLE_WIRE_TEXT).expect("rectangle wire example");
    let config = Generation3dConfig::default();
    let (meshes_json, instances_json) = preview_payload_from_evaluated_fixture(&projection.fixture, &config);
    let meshes: Vec<Value> = serde_json::from_str(&meshes_json).expect("meshes");
    assert!(!meshes.is_empty(), "rectangle wire preview should tessellate curve edges");
    let data = mesh_data_from_json(&meshes[0].get("data").cloned().unwrap_or_default());
    assert!(data.indices.is_empty(), "wire preview has no shaded triangles");
    assert!(data.edge_positions.len() >= 6, "curve preview should include edge polylines");
    assert!(!instances_json.is_empty());
}

#[test]
fn all_bundled_examples_emit_preview_meshes() {
    let _serial = test_serial();
    let config = Generation3dConfig::default();
    let cases = [
        ("hexagonal-mushroom-column", crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_HEX_COLUMN),
        ("rectangle-extrude-volume", crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_RECT_EXTRUDE),
        ("sphere-cut-with-torus", crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_SPHERE_TORUS),
        ("box-fillet-preview", crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_BOX_FILLET),
        ("sphere-box-fuse", crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE),
        ("face-sweep-extrude", crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE),
        ("rectangle-wire-preview", crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_RECTANGLE_WIRE),
        ("box-shell-preview", crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_BOX_SHELL),
    ];
    for (label, example_id) in cases {
        let projection = crate::standards::v1::subsets::any::schema::example_snapshot(example_id).unwrap_or_else(|| panic!("{label}: missing projection"));
        let (meshes_json, instances_json) = preview_payload_from_evaluated_fixture(&projection.fixture, &config);
        assert_ne!(meshes_json, "[]", "{label}: meshes empty; eval may have failed");
        assert_ne!(instances_json, "[]", "{label}: instances empty");
        let meshes: Vec<Value> = serde_json::from_str(&meshes_json).unwrap_or_else(|err| panic!("{label}: meshes json: {err}"));
        assert!(!meshes.is_empty(), "{label}: no mesh entries");
    }
}

#[test]
fn preview_tolerance_follows_lod_mode() {
    assert!((preview_tolerance("coarse") - 0.15).abs() < 1e-9);
    assert!((preview_tolerance("fine") - 0.02).abs() < 1e-9);
    assert!((preview_tolerance("") - 0.05).abs() < 1e-9);
}

#[test]
fn wireframe_show_mode_strips_shaded_triangles() {
    let _serial = test_serial();
    let projection = crate::standards::v1::subsets::any::schema::default_snapshot();
    let config = Generation3dConfig { show_mode: "wireframe".into(), ..Default::default() };
    let (meshes_json, _) = preview_payload_from_evaluated_fixture(&projection.fixture, &config);
    let meshes: Vec<Value> = serde_json::from_str(&meshes_json).expect("meshes");
    assert!(!meshes.is_empty());
    let data = mesh_data_from_json(&meshes[0].get("data").cloned().unwrap_or_default());
    assert!(data.indices.is_empty());
    assert!(!data.edge_positions.is_empty());
}

#[test]
fn generation3d_io_declares_the_params_and_geometry_ports() {
    let io = semio_framework::io::resolve_ready(generation3d_io());
    assert_eq!(io.document_schema, "generation.3d");
    assert_eq!(io.artifact.id, "3d.generation");
    let params = io.ports.iter().find(|port| port.id == "params:in").expect("params:in declared");
    assert_eq!(params.direction, semio_framework_plugin::MediaPortDirection::In);
    assert!(!params.required);
    let geometry = io.ports.iter().find(|port| port.id == "geometry:out").expect("geometry:out declared");
    assert_eq!(geometry.direction, semio_framework_plugin::MediaPortDirection::Out);
    assert_eq!(geometry.kind_id.as_deref(), Some("3d.mesh"));
    assert_eq!(geometry.multiplicity, semio_framework::PortMultiplicity::Many);
}

/// 🔌️ One `preview: true` neuron with two output channels (a point channel and a vector
/// channel — neither needs a brep kernel) must yield one instance PER CHANNEL, each id
/// qualified with its own channel, not one flattened instance for the whole widget.
fn preview_widget_fixture(id: &str, output_ports: Vec<String>) -> semio_framework_artifact_flow_flow::FlowFixture {
    let widget = semio_framework_artifact_flow_flow::Widget::Neuron { id: id.into(), neuron_kind: "test.multi".into(), params: semio_framework_artifact_flow_flow::neural::Dictionary::new(), input_ports: Vec::new(), output_ports, preview: true };
    semio_framework_artifact_flow_flow::FlowFixture { schema: "flow.fixture".into(), camera: semio_framework_artifact_flow_flow::CameraJson { x: 0.0, y: 0.0, zoom: 1.0 }, widgets: vec![widget], synapses: Vec::new(), layout: Default::default() }
}

#[test]
fn preview_payload_channel_qualifies_ids_across_two_output_channels() {
    let _serial = test_serial();
    let fixture = preview_widget_fixture("multi", vec!["a".into(), "b".into()]);
    let eval_json = json!({
        "multi": {
            "out": {
                "a": { "$schema": "point", "x": 1.0, "y": 2.0, "z": 3.0 },
                "b": { "$schema": "vector", "x": 4.0, "y": 5.0, "z": 6.0 }
            }
        }
    })
    .to_string();
    let config = Generation3dConfig::default();
    let (meshes_json, instances_json) = preview_payload_from_eval(&eval_json, &fixture, &config);
    let instances: Vec<Value> = serde_json::from_str(&instances_json).expect("instances json");
    assert_eq!(instances.len(), 2, "two output channels should yield two preview instances, got {instances:?}");
    let ids: std::collections::HashSet<&str> = instances.iter().filter_map(|entry| entry.get("id").and_then(Value::as_str)).collect();
    assert!(ids.contains("multi@a#0"), "point-channel instance id missing, got {ids:?}");
    assert!(ids.contains("multi@b#0"), "vector-channel instance id missing, got {ids:?}");
    let meshes: Vec<Value> = serde_json::from_str(&meshes_json).expect("meshes json");
    assert_eq!(meshes.len(), 2, "each inline channel mints its own mesh, got {meshes:?}");
}

/// 🔌️ A single channel whose value is a `$schema: "list"` dictionary (the wire form
/// `semio_framework_artifact_flow_flow::neural::Dictionary` lists actually take) of N geometry-bearing entries must flatten
/// to N instances, indexed `#0..#{N-1}` in list order — proven here with inline points so the
/// test needs no brep kernel/session.
#[test]
fn preview_payload_flattens_a_list_channel_into_indexed_instances() {
    let _serial = test_serial();
    let fixture = preview_widget_fixture("listy", vec!["points".into()]);
    let eval_json = json!({
        "listy": {
            "out": {
                "points": {
                    "$schema": "list",
                    "0": { "$schema": "point", "x": 1.0, "y": 0.0, "z": 0.0 },
                    "1": { "$schema": "point", "x": 2.0, "y": 0.0, "z": 0.0 },
                    "2": { "$schema": "point", "x": 3.0, "y": 0.0, "z": 0.0 }
                }
            }
        }
    })
    .to_string();
    let config = Generation3dConfig::default();
    let (_meshes_json, instances_json) = preview_payload_from_eval(&eval_json, &fixture, &config);
    let instances: Vec<Value> = serde_json::from_str(&instances_json).expect("instances json");
    assert_eq!(instances.len(), 3, "a 3-entry list channel should yield 3 instances, got {instances:?}");
    for index in 0..3 {
        let expected_id = format!("listy@points#{index}");
        assert!(instances.iter().any(|entry| entry.get("id").and_then(Value::as_str) == Some(expected_id.as_str())), "missing {expected_id} in {instances:?}");
    }
}

/// 🔌️ A channel carrying only pure data (a number, no handle, no `x`/`y`/`z`) is not
/// geometry-bearing and must not fabricate a placeholder preview instance.
#[test]
fn preview_payload_emits_no_instance_for_a_pure_data_channel() {
    let _serial = test_serial();
    let fixture = preview_widget_fixture("scalar", vec!["value".into()]);
    let eval_json = json!({
        "scalar": {
            "out": {
                "value": { "$schema": "number", "value": 42.0 }
            }
        }
    })
    .to_string();
    let config = Generation3dConfig::default();
    let (meshes_json, instances_json) = preview_payload_from_eval(&eval_json, &fixture, &config);
    assert_eq!(meshes_json, "[]", "pure-data channel must not fabricate mesh geometry");
    assert_eq!(instances_json, "[]", "pure-data channel must not fabricate a preview instance");
}
/// 🕹️ The three id forms one mark can take, and the transitive reach of each: a node-level
/// mark covers every channel and every instance below it, a channel-level mark covers only its
/// own channel, and an instance-level mark covers only itself.
#[test]
fn preview_marks_resolve_node_channel_and_instance_ids() {
    let node = PreviewInteractionMarks { hovered: ["multi".to_string()].into_iter().collect(), selected: Default::default() };
    assert!(node.hovers("multi", "a", 0) && node.hovers("multi", "b", 3));
    assert!(!node.hovers("other", "a", 0));

    let channel = PreviewInteractionMarks { hovered: ["multi@b".to_string()].into_iter().collect(), selected: Default::default() };
    assert!(channel.hovers("multi", "b", 0) && channel.hovers("multi", "b", 7));
    assert!(!channel.hovers("multi", "a", 0));

    let instance = PreviewInteractionMarks { hovered: ["multi@b#2".to_string()].into_iter().collect(), selected: Default::default() };
    assert!(instance.hovers("multi", "b", 2));
    assert!(!instance.hovers("multi", "b", 1));
}

/// 🕹️ Graph → world: hovering the NODE in the node graph must light up every one of its
/// channels' preview geometry, not just one.
#[test]
fn preview_payload_marks_every_channel_of_a_hovered_node() {
    let _serial = test_serial();
    let fixture = preview_widget_fixture("multi", vec!["a".into(), "b".into()]);
    let eval_json = json!({ "multi": { "out": {
            "a": { "$schema": "point", "x": 1.0, "y": 2.0, "z": 3.0 },
            "b": { "$schema": "vector", "x": 4.0, "y": 5.0, "z": 6.0 }
        } } })
    .to_string();
    let marks = PreviewInteractionMarks { hovered: ["multi".to_string()].into_iter().collect(), selected: ["multi@a".to_string()].into_iter().collect() };
    let payload = preview_payload(&eval_json, &fixture, &Generation3dConfig::default(), None, &marks);
    let instances: Vec<Value> = serde_json::from_str(&payload.instances_json).expect("instances json");
    assert_eq!(instances.len(), 2);
    assert!(instances.iter().all(|entry| entry.get("hovered").and_then(Value::as_bool) == Some(true)), "node hover must reach every channel: {instances:?}");
    assert_eq!(payload.selected_ids, vec!["multi@a#0".to_string()], "channel-level selection must not spill onto the sibling channel");
    assert!(payload.hovered_id.is_some(), "the scene needs a concrete hovered instance to paint");
}

/// 🕹️ Graph → world, narrowed: hovering one PORT lights up only that channel's geometry.
#[test]
fn preview_payload_marks_only_the_hovered_channel() {
    let _serial = test_serial();
    let fixture = preview_widget_fixture("multi", vec!["a".into(), "b".into()]);
    let eval_json = json!({ "multi": { "out": {
            "a": { "$schema": "point", "x": 1.0, "y": 2.0, "z": 3.0 },
            "b": { "$schema": "vector", "x": 4.0, "y": 5.0, "z": 6.0 }
        } } })
    .to_string();
    let marks = PreviewInteractionMarks { hovered: ["multi@b".to_string()].into_iter().collect(), selected: Default::default() };
    let payload = preview_payload(&eval_json, &fixture, &Generation3dConfig::default(), None, &marks);
    let instances: Vec<Value> = serde_json::from_str(&payload.instances_json).expect("instances json");
    let hovered: Vec<&str> = instances.iter().filter(|entry| entry.get("hovered").and_then(Value::as_bool) == Some(true)).filter_map(|entry| entry.get("id").and_then(Value::as_str)).collect();
    assert_eq!(hovered, vec!["multi@b#0"], "only the hovered channel may light up: {instances:?}");
    assert_eq!(payload.hovered_id.as_deref(), Some("multi@b#0"));
}

/// 🕹️ World → graph: hovering one preview INSTANCE in the 3D world resolves back to its node
/// and its port, which is what the node-graph window paints.
#[test]
fn graph_marks_project_instance_hover_back_onto_its_node_and_port() {
    let marks = PreviewInteractionMarks { hovered: ["multi@b#0".to_string()].into_iter().collect(), selected: ["multi@a#1".to_string()].into_iter().collect() };
    assert_eq!(marks.hovered_graph_target(), Some(("multi".to_string(), Some("b".to_string()))));
    assert!(marks.graph_highlight_ids().contains(&"multi".to_string()));
    assert_eq!(marks.graph_selection_ids(), vec!["multi".to_string()]);
    assert_eq!(PreviewInteractionMarks::widget_of("multi@b#0"), "multi");
    assert_eq!(PreviewInteractionMarks::port_of("multi@b#0"), Some("b"));
    assert_eq!(PreviewInteractionMarks::port_of("multi"), None);
}

/// 🕸️ Every port the node graph paints is also an interaction target parented to its widget —
/// the topology link `HoverSpec { transitive: true }` walks.
#[test]
fn interaction_topology_ports_match_the_node_graph_port_ids() {
    let _serial = test_serial();
    let projection = crate::standards::v1::subsets::any::schema::default_snapshot();
    let ports_by_node = generation3d_port_ids_by_node(&projection.fixture);
    assert!(!ports_by_node.is_empty(), "default fixture should project graph nodes");
    assert!(ports_by_node.values().any(|ports| !ports.is_empty()), "default fixture should project at least one port");
    for (node_id, ports) in &ports_by_node {
        for port in ports {
            assert!(port.starts_with(&format!("{node_id}@")), "port {port} must be qualified by its node {node_id}");
            assert_eq!(PreviewInteractionMarks::widget_of(port), node_id.as_str());
        }
    }
}
/// 👁️ Which widget kinds contribute preview geometry: a neuron only when its author-set toggle
/// is on, an output preview always, a cluster always (it has no toggle of its own, and its
/// inner `semio_framework_artifact_flow_flow::neural::Neuron`s have none either), and a pure input widget never.
#[test]
fn widget_preview_eligibility_covers_neurons_output_previews_and_clusters() {
    let on = semio_framework_artifact_flow_flow::Widget::Neuron { id: "n".into(), neuron_kind: "k".into(), params: semio_framework_artifact_flow_flow::neural::Dictionary::new(), input_ports: Vec::new(), output_ports: Vec::new(), preview: true };
    let off = semio_framework_artifact_flow_flow::Widget::Neuron { id: "n".into(), neuron_kind: "k".into(), params: semio_framework_artifact_flow_flow::neural::Dictionary::new(), input_ports: Vec::new(), output_ports: Vec::new(), preview: false };
    let output = semio_framework_artifact_flow_flow::Widget::OutputPreview { id: "p".into(), preview: Default::default(), expanded: Default::default() };
    let cluster = semio_framework_artifact_flow_flow::Widget::Cluster { id: "c".into(), name: "Cluster".into(), tree: Default::default(), flow: Default::default() };
    let slider = semio_framework_artifact_flow_flow::Widget::InputSlider { id: "s".into(), label: "S".into(), value: 0.0, min: 0.0, max: 1.0, step: 0.1 };
    assert!(widget_previews(&on));
    assert!(!widget_previews(&off));
    assert!(widget_previews(&output));
    assert!(widget_previews(&cluster));
    assert!(!widget_previews(&slider));
}
//#endregion 🔖️EngineComputeTests

//#region 🔖️ExamplesTests
/// 📚️ Ticket 26/09/03/PROCEDURAL-3D-END-TO-END — `examples()` (wired at the plugin root via
/// `.editor_with_examples::<Generation3dPlayApp>(create_generation3d_app(), …examples())`) must
/// carry the same eight ids, in the same order, as the `setActiveExample` select options this app
/// declares — otherwise the navbar dropdown and the action's own arg picker disagree.
#[test]
fn examples_match_set_active_example_select_options() {
    let definition = create_generation3d_app();
    let select_ids: Vec<String> = definition
        .window_kinds
        .iter()
        .flat_map(|window| window.actions.iter())
        .find(|action| action.id == "setActiveExample")
        .and_then(|action| action.args.first())
        .and_then(|arg| match arg.control() {
            semio_framework::ActionArgControl::Select { options } => Some(options.into_iter().map(|option| option.value).collect::<Vec<_>>()),
            _ => None,
        })
        .expect("setActiveExample must declare a Select arg");
    let example_ids: Vec<String> = examples().into_iter().map(|source| source.id().to_string()).collect();
    assert_eq!(example_ids.len(), 8);
    assert_eq!(example_ids, select_ids);
}
//#endregion 🔖️ExamplesTests
