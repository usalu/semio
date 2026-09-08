
use super::*;
use crate::editor::generation2d::testkit::{app, app_with_registry};
use semio_framework_artifact_flow_flow::Widget;
use semio_framework_plugin::PluginApp;
use semio_framework_plugin::testkit::assert_undo_redo_round_trip;

fn production_initial_snapshot(label: &str) -> Generation2dSnapshot {
    let mut snapshot = Generation2dSnapshot::default();
    snapshot.fixture.schema = label.into();
    for (id, text) in [("replace-target", "before replacement"), ("delete-target", "delete me"), ("move-target", "move me"), ("clear-target", "clear me")] {
        snapshot.fixture.widgets.push(Widget::InputNote { id: id.into(), text: text.into() });
    }
    snapshot.fixture.synapses.push(semio_framework_artifact_flow_flow::SynapseSpec { id: "replace-synapse".into(), from: "replace-target".into(), to: "move-target".into(), from_port: "old".into(), to_port: "old".into() });
    snapshot.fixture.synapses.push(semio_framework_artifact_flow_flow::SynapseSpec { id: "disconnect-synapse".into(), from: "move-target".into(), to: "clear-target".into(), from_port: String::new(), to_port: String::new() });
    snapshot.fixture.layout.insert("move-target".into(), semio_framework_artifact_flow_flow::WidgetLayout { x: 1.0, y: 2.0 });
    snapshot.fixture.layout.insert("clear-target".into(), semio_framework_artifact_flow_flow::WidgetLayout { x: 3.0, y: 4.0 });
    for (id, name) in [("delete-generation", "Delete"), ("rename-generation", "Before Rename"), ("change-generation", "Change Value")] {
        snapshot.generation.cold_builder_mut().expect("unique cold generation owner").generations.push(semio_framework_artifact_playbook_playbook::FormGeneration { id: id.into(), name: name.into(), values: Default::default() });
    }
    snapshot.generation.cold_builder_mut().expect("unique cold generation owner").selected_generation_id = Some("rename-generation".into());
    snapshot
}

fn production_mutations() -> Vec<Generation2dMutation> {
    use crate::standards::v1::subsets::any::schema::mutations::*;
    let params = semio_framework_artifact_flow_flow::neural::Dictionary::new().insert("integer", semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::Integer(7))).insert(
        "nested",
        semio_framework_artifact_flow_flow::neural::Value::Dictionary(
            semio_framework_artifact_flow_flow::neural::Dictionary::new().insert("text", semio_framework_artifact_flow_flow::neural::Value::Atom(semio_framework_artifact_flow_flow::neural::Atom::String("production".into()))),
        ),
    );
    vec![
        create_widget(0, Widget::Neuron { id: "created-widget".into(), neuron_kind: "law".into(), params, input_ports: vec!["in".into()], output_ports: vec!["out".into()], preview: true }),
        replace_widget(Widget::Cluster { id: "replace-target".into(), name: "After Replacement".into(), tree: Default::default(), flow: Default::default() }),
        delete_widget("delete-target".into()),
        connect_synapse(0, semio_framework_artifact_flow_flow::SynapseSpec { id: "created-synapse".into(), from: "created-widget".into(), to: "replace-target".into(), from_port: "out".into(), to_port: "in".into() }),
        replace_synapse(semio_framework_artifact_flow_flow::SynapseSpec { id: "replace-synapse".into(), from: "replace-target".into(), to: "move-target".into(), from_port: "new-out".into(), to_port: "new-in".into() }),
        disconnect_synapse("disconnect-synapse".into()),
        move_widget("move-target".into(), semio_framework_artifact_flow_flow::WidgetLayout { x: 31.0, y: -17.0 }),
        clear_widget_layout("clear-target".into()),
        update_camera(semio_framework_artifact_flow_flow::CameraJson { x: 9.0, y: 8.0, zoom: 1.75 }),
        change_schema("flow.fixture.production-retained".into()),
        create_generation(semio_framework_artifact_playbook_playbook::FormGeneration { id: "created-generation".into(), name: "Created".into(), values: Default::default() }),
        delete_generation("delete-generation".into()),
        rename_generation("rename-generation".into(), "After Rename".into()),
        change_generation_value("change-generation".into(), "deep-answer".into(), serde_json::json!({"object": {"array": [1.0, false, "retained"]}}).into()),
    ]
}

fn production_hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut value = String::new();
    value.try_reserve_exact(bytes.len() * 2).expect("P2 production hex preflight");
    for byte in bytes {
        value.push(char::from(DIGITS[usize::from(byte >> 4)]));
        value.push(char::from(DIGITS[usize::from(byte & 0x0f)]));
    }
    value
}

fn production_semantic_digest(snapshot: &Generation2dSnapshot) -> [u8; 32] {
    let mut digest = store::ArtifactStoreInitializationDigest::new(b"generation2d.production-law.semantic");
    digest.observe(&crate::standards::v1::subsets::any::schema::snapshot::binary::encode(snapshot));
    digest.finish()
}

fn production_envelope_wire(label: &str) -> (Vec<u8>, Generation2dSnapshot, [u8; 32]) {
    let snapshot = production_initial_snapshot(label);
    let mutations = production_mutations();
    assert_eq!(mutations.len(), 14, "production ingress carries every P2 mutation variant including clear-widget-layout");
    let mut mutation_hex = Vec::new();
    mutation_hex.try_reserve_exact(mutations.len()).expect("P2 production mutation owner preflight");
    for mutation in &mutations {
        mutation_hex.push(production_hex(&crate::standards::v1::subsets::any::schema::mutations::binary::encode_op(mutation).expect("P2 production mutation encoding")));
    }
    let mut expected = production_initial_snapshot(label);
    crate::standards::v1::subsets::any::schema::mutations::binary::generation2d_apply_retained_mutations_for_test(&mut expected, &mutations);
    let expected_digest = production_semantic_digest(&expected);
    let wire = serde_json::to_vec(&serde_json::json!({
        "schema": GENERATION_2D_SCHEMA,
        "id": "generation2d-production-mounted-law",
        "vcs": {
            "initialSnapshot": production_hex(&crate::standards::v1::subsets::any::schema::snapshot::binary::encode(&snapshot)),
            "edits": [{
                "id": "generation2d-production-all14-edit",
                "actor": "generation2d-production-law",
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
    .expect("schema-first P2 production fixture envelope");
    (wire, expected, expected_digest)
}

fn admit_production_envelope(app: &mut semio_framework_plugin::VcsArtifactApp<EditorApp<Generation2dPlayApp>>, wire: &[u8]) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle {
    let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
    let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("P2 production ingress credits");
    crate::standards::v1::subsets::any::schema::mutations::binary::generation2d_admit_publication_authority(
        handle.operation,
        handle.generation,
        handle.generation.0,
        handle.generation.0,
        handle.generation.0,
        8_192,
        crate::standards::v1::subsets::any::schema::mutations::binary::GENERATION2D_MOUNTED_OUTPUT_CHANNELS,
        crate::standards::v1::subsets::any::schema::mutations::binary::GENERATION2D_MOUNTED_CONTROL_CREDITS,
    )
    .expect("P2 production publication authority");
    for chunk in wire.chunks(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
        let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
        bytes[..chunk.len()].copy_from_slice(chunk);
        let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded P2 production envelope page");
        app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("P2 production envelope page admission failed: {fault:?}"));
    }
    assert!(app.seal_artifact_envelope_ingress(handle).expect("P2 production envelope seal"));
    handle
}

fn drive_production_envelope(app: &mut semio_framework_plugin::VcsArtifactApp<EditorApp<Generation2dPlayApp>>, handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll {
    for _ in 0..300_000 {
        crate::standards::v1::subsets::any::schema::mutations::binary::generation2d_refresh_publication_authority(handle.operation, handle.generation, app.artifact_generation_now().0).expect("P2 authority refresh immediately before production maintenance");
        PluginApp::maintenance_step(app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("one P2 production maintenance turn");
        let poll = app.advance_artifact_envelope_load(handle).expect("P2 production load advancement");
        if matches!(poll, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Cancelled | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault) {
            return poll;
        }
        std::thread::yield_now();
    }
    panic!("P2 production envelope load did not reach terminal");
}

/// 🔐️ LAW: non-empty P2D2 canonical ingress reaches the real VCS maintenance replacement,
/// and accepted, stale, ABA, and displaced stores remain owned until explicit terminal ACK/close.
#[semio_framework_async_macros::async_test]
async fn vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed() {
    let mut accepted = semio_framework_plugin::VcsArtifactApp::<EditorApp<Generation2dPlayApp>>::new(EditorApp::default()).await;
    let base_generation = accepted.artifact_generation_now();
    let (wire, expected, expected_digest) = production_envelope_wire("accepted-production-swap");
    let handle = admit_production_envelope(&mut accepted, &wire);
    assert_eq!(drive_production_envelope(&mut accepted, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
    assert_eq!(accepted.artifact_generation_now().0, base_generation.0 + 1);
    let snapshot = accepted.snapshot().expect("accepted P2 production snapshot");
    assert_eq!(&snapshot, &expected, "real maintenance must publish all P2 snapshot and all-14 replay fields");
    assert_eq!(production_semantic_digest(&snapshot), expected_digest);
    assert!(snapshot.fixture.layout.contains_key("move-target"));
    assert!(!snapshot.fixture.layout.contains_key("clear-target"), "2D-only clear-widget-layout must survive retained replay");
    assert!(accepted.acknowledge_artifact_store_replacement(handle).expect("accepted P2 terminal ACK"));
    assert!(crate::standards::v1::subsets::any::schema::mutations::binary::generation2d_release_publication_authority(handle.operation, handle.generation));

    use crate::standards::v1::subsets::any::schema::mutations::binary::Generation2dPublicationHostile::{Missing, WrongBase, WrongGeneration, WrongOperation, WrongParent};
    for (hostile, expected_code) in [
        (Missing, "generation2d-publication.authority-missing"),
        (WrongOperation, "generation2d-publication.wrong-operation"),
        (WrongGeneration, "generation2d-publication.wrong-generation"),
        (WrongBase, "generation2d-publication.wrong-base"),
        (WrongParent, "generation2d-publication.wrong-parent"),
    ] {
        let mut app = semio_framework_plugin::VcsArtifactApp::<EditorApp<Generation2dPlayApp>>::new(EditorApp::default()).await;
        let last_valid = app.snapshot().expect("last-valid P2 snapshot");
        let last_valid_digest = production_semantic_digest(&last_valid);
        let base_generation = app.artifact_generation_now();
        let (wire, _, _) = production_envelope_wire("rejected-production-candidate");
        let handle = admit_production_envelope(&mut app, &wire);
        crate::standards::v1::subsets::any::schema::mutations::binary::generation2d_arm_publication_hostile(handle.operation, hostile);
        assert_eq!(drive_production_envelope(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
        assert_eq!(crate::standards::v1::subsets::any::schema::mutations::binary::generation2d_take_publication_hostile_observed(handle.operation), Some(expected_code));
        assert_eq!(app.artifact_generation_now(), base_generation);
        let retained = app.snapshot().expect("last-valid P2 snapshot after rejected candidate");
        assert_eq!(production_semantic_digest(&retained), last_valid_digest);
        assert_eq!(retained, last_valid);
        assert!(app.acknowledge_artifact_store_replacement(handle).expect("rejected P2 terminal ACK after candidate retirement"));
        assert!(crate::standards::v1::subsets::any::schema::mutations::binary::generation2d_release_publication_authority(handle.operation, handle.generation));
    }
}

//#region 🔖️CommandSurface
#[test]
fn retained_route_dispositions_are_exact_and_exhaustive() {
    use semio_framework::{ToolCancellationPolicy, ToolExecutionShape};
    use semio_framework_plugin::ArtifactOwnedToolJobFactory;
    assert_eq!(GENERATION2D_BOUNDED_TOOL_IDS.len(), 7);
    assert_eq!(<Generation2dPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), 7);
    assert_eq!(Generation2dBoundedCommandJobFactory::PUBLICATION_CONTRACTS.len(), 7);
    assert_eq!(generation2d_bounded_contract().shape, ToolExecutionShape::BoundedFirstStep);
    assert_eq!(generation2d_bounded_contract().cancellation, ToolCancellationPolicy::PerOperation);
    assert!(GENERATION2D_BOUNDED_TOOL_IDS.iter().all(|tool_id| Generation2dBoundedCommandJobFactory::PUBLICATION_CONTRACTS.iter().any(|contract| contract.tool_id == *tool_id)));
    for blocked in [
        "nodeGraphEdit",
        "moveMediaNode",
        "addWidget",
        "removeWidget",
        "connectMediaPorts",
        "reorganize",
        "addGeneration",
        "removeGeneration",
        "renameGeneration",
        "updateGenerationValues",
        "setEvalOutputs",
        "selectGeneration",
        "flowEvalTick",
    ] {
        assert!(!GENERATION2D_BOUNDED_TOOL_IDS.contains(&blocked));
    }
}

#[test]
fn command_ids_are_unique_and_cover_every_row() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), 21, "every Generation2dCommand row must be covered by every_command()");
}

#[test]
fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — pinned
/// where the two vocabularies genuinely diverge. This is what a missing `#[dsl(keyword = ..)]` on a
/// payload struct silently breaks (the record prints with no keyword at all and fails to re-parse).
#[test]
fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
    let expected_keywords = [
        "node-graph-edit",
        "move-media-node",
        "add-widget",
        "remove-widget",
        "connect-media-ports",
        "reorganize",
        "add-generation",
        "remove-generation",
        "rename-generation",
        "update-generation-values",
        "node-graph-viewport",
        "set-show-mode",
        "generate",
        "set-eval-outputs",
        "canvas-pointer-down",
        "canvas-pointer-move",
        "canvas-pointer-up",
        "canvas-wheel",
        "select-generation",
        "flow-eval-tick",
        "locale",
    ];
    let commands = every_command();
    assert_eq!(commands.len(), expected_keywords.len(), "every_command() and expected_keywords must stay in the same declaration order");
    for (command, expected_keyword) in commands.iter().zip(expected_keywords) {
        let printed = protocol::OpText::print_op(command);
        assert_eq!(printed.split(' ').next().unwrap_or_default(), expected_keyword, "wire keyword drifted for command {}: {printed:?}", command.command_id());
    }
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub(super) fn every_command() -> Vec<Generation2dCommand> {
    vec![
        Generation2dCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: "[]".into() }),
        Generation2dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: "n1".into(), x: 1.0, y: 2.0 }),
        Generation2dCommand::AddWidget(add_widget::AddWidget { kind: "inputSlider".into(), neuron_kind: None, x: Some(10.0), y: None }),
        Generation2dCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: "n1".into() }),
        Generation2dCommand::ConnectMediaPorts(connect_media_ports::ConnectMediaPorts { source_node_id: "n1".into(), source_port_id: "out".into(), target_node_id: "n2".into(), target_port_id: "in".into() }),
        Generation2dCommand::Reorganize(reorganize::Reorganize {}),
        Generation2dCommand::AddGeneration(add_generation::AddGeneration {}),
        Generation2dCommand::RemoveGeneration(remove_generation::RemoveGeneration { id: "g1".into() }),
        Generation2dCommand::RenameGeneration(rename_generation::RenameGeneration { id: "g1".into(), name: "Copy".into() }),
        Generation2dCommand::UpdateGenerationValues(update_generation_values::UpdateGenerationValues { generation_id: Some("g1".into()), question_id: "q1".into(), value: dsl::DslValue::float(5.0) }),
        Generation2dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport_json: "{}".into() }),
        Generation2dCommand::SetShowMode(set_show_mode::SetShowMode { value: "wire".into() }),
        Generation2dCommand::Generate(enter_generate::Generate {}),
        Generation2dCommand::SetEvalOutputs(set_eval_outputs::SetEvalOutputs { outputs_json: "{}".into() }),
        Generation2dCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {}),
        Generation2dCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove {}),
        Generation2dCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {}),
        Generation2dCommand::CanvasWheel(canvas_wheel::CanvasWheel {}),
        Generation2dCommand::SelectGeneration(select_generation::SelectGeneration { id: Some("g1".into()) }),
        Generation2dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick {}),
    ]
}
//#endregion 🔖️CommandSurface

//#region 🔖️ManifestSanity
#[test]
fn the_manifest_stitches_every_taxonomy_node() {
    let json = serde_json::to_string(&create_generation2d_app()).expect("app definition json");
    for id in [
        flow_window::GENERATION2D_PLAY_WINDOW_MAIN,
        edit_preview::GENERATION2D_PLAY_WINDOW_PREVIEW,
        generations::GENERATION2D_PLAY_WINDOW_GENERATIONS,
        form::GENERATION2D_PLAY_WINDOW_GENERATE_FORM,
        generate_preview::GENERATION2D_PLAY_WINDOW_GENERATE_PREVIEW,
    ] {
        assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
    }
    for id in [edit::GENERATION2D_PLAY_MODE_EDIT, generate::GENERATION2D_PLAY_MODE_GENERATE] {
        assert!(json.contains(id), "mode {id} missing from the manifest");
    }
    for body in [document_panel::GENERATION2D_PLAY_BODY_DOCUMENT, catalogue_panel::GENERATION2D_PLAY_BODY_CATALOGUE, inspection_panel::GENERATION2D_PLAY_BODY_INSPECTION] {
        assert!(json.contains(body), "panel body {body} missing from the manifest");
    }
    assert!(json.contains("2d.generation"), "artifact kind missing from the manifest");
}
//#endregion 🔖️ManifestSanity

//#region 🔖️CrossCutting
#[semio_framework_async_macros::async_test]
async fn declared_actions_bridge_to_commands() {
    semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<EditorApp<Generation2dPlayApp>>(testkit::generation2d_manifest_for_testkit).await;
}

#[semio_framework_async_macros::async_test]
async fn add_widget_materializes_declared_kind_default_into_an_operation() {
    let mut app = app_with_registry().await;
    let before = app.snapshot().expect("snapshot").fixture.widgets.len();
    app.dispatch_typed(Generation2dCommand::AddWidget(add_widget::AddWidget { kind: "inputSlider".into(), neuron_kind: None, x: None, y: None }), &semio_framework_plugin::testkit::meta("local")).await.expect("add widget");
    assert_eq!(app.snapshot().expect("snapshot").fixture.widgets.len(), before + 1);
}

#[semio_framework_async_macros::async_test]
async fn add_widget_undo_redo_round_trip() {
    let mut app = app().await;
    let before = app.snapshot().expect("snapshot").fixture.widgets.len();
    assert_undo_redo_round_trip(&mut app, Generation2dCommand::AddWidget(add_widget::AddWidget { kind: "inputNote".into(), neuron_kind: None, x: None, y: None }), |app| app.snapshot().expect("snapshot").fixture.widgets.len(), before, before + 1)
        .await;
}

#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_widget_moves() {
    let widgets: Vec<String> = app().await.snapshot().expect("snapshot").fixture.widgets.iter().map(|widget| crate::widget_id(widget).to_string()).collect();
    assert!(widgets.len() >= 2, "default fixture needs two widgets for the test");
    let (w0, w1) = (widgets[0].clone(), widgets[1].clone());
    semio_framework_plugin::testkit::assert_two_instances_converge::<EditorApp<Generation2dPlayApp>, (Option<f64>, Option<f64>)>(
        "mem://generation2d-convergence",
        Generation2dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: w0.clone(), x: 111.0, y: 5.0 }),
        Generation2dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: w1.clone(), x: 222.0, y: 6.0 }),
        move |app| {
            let layout = &app.snapshot().expect("snapshot").fixture.layout;
            (layout.get(&w0).map(|entry| entry.x), layout.get(&w1).map(|entry| entry.x))
        },
    )
    .await;
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    use crate::editor::generation2d::testkit::render;
    let mut app = app().await;
    assert!(render(&mut app, "generation2d.play.nope").await.contains("Unknown body"));
}
//#endregion 🔖️CrossCutting

//#region 🔖️ContextMenuTests
/// 🕹️ `context_menu` no longer has anything to dispatch a selection command WITH (`setSelection`
/// is deleted — selection is the framework's `graph` interaction domain now) and `context_menu`
/// itself carries no `InteractionView` to read it back even if it did (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM, same discovered gap as `render`), so the
/// destructive `delete-selection` row — conditioned on a real selection — never appears; this test
/// now only pins the disclosure budget.
#[semio_framework_async_macros::async_test]
async fn context_menu_stays_within_disclosure_budget() {
    let mut app = app_with_registry().await;
    let request = semio_framework_plugin::ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "nodeGraph".into(), args: None }, surface: None, window_instance_id: None, point: None };
    let items = app.context_menu(&request).await;
    assert!(items.len() <= 9, "top-level menu rows (leaves + groups + separator) must stay within disclosure budget, got {}", items.len());
    assert!(items.iter().all(|item| item.id != "delete-selection"), "no interaction data at context_menu time means delete-selection cannot appear");
}
//#endregion 🔖️ContextMenuTests

//#region 🔖️PortTests
#[semio_framework_async_macros::async_test]
async fn export_drawing_out_returns_vector_media() {
    let mut app = app().await;
    let media = semio_framework_plugin::resolve_ready(app.export_media("drawing:out")).expect("export drawing:out");
    assert_eq!(media.media_type, MediaType { class: MediaClass::TwoD, form: MediaForm::Vector });
}

#[semio_framework_async_macros::async_test]
async fn export_document_out_returns_flow_media() {
    let mut app = app().await;
    let media = semio_framework_plugin::resolve_ready(app.export_media("document:out")).expect("export document:out");
    assert_eq!(media.media_type, MediaType { class: MediaClass::TwoD, form: MediaForm::Flow });
    assert!(matches!(media.payload, semio_framework_plugin::MediaPayload::Structured { schema, .. } if schema == GENERATION_2D_SCHEMA));
}

#[semio_framework_async_macros::async_test]
async fn import_params_in_patches_matching_input_slider() {
    let mut app = app().await;
    app.dispatch_typed(Generation2dCommand::AddWidget(add_widget::AddWidget { kind: "inputSlider".into(), neuron_kind: None, x: None, y: None }), &semio_framework_plugin::testkit::meta("local")).await.expect("add slider");
    let slider_id = app
        .snapshot()
        .expect("snapshot")
        .fixture
        .widgets
        .iter()
        .find_map(|widget| match widget {
            Widget::InputSlider { id, .. } => Some(id.clone()),
            _ => None,
        })
        .expect("just-added input slider");
    let media = semio_framework_plugin::Media {
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        payload: semio_framework_plugin::MediaPayload::Structured { schema: "params".into(), json: serde_json::json!({ slider_id.clone(): 42.0 }).to_string() },
    };
    app.import_media("params:in", media, &semio_framework_plugin::testkit::meta("local")).await.expect("import params");
    let value = app.snapshot().expect("snapshot").fixture.widgets.iter().find_map(|widget| match widget {
        Widget::InputSlider { id, value, .. } if id == &slider_id => Some(*value),
        _ => None,
    });
    assert_eq!(value, Some(42.0));
}

#[semio_framework_async_macros::async_test]
async fn media_ports_declare_params_in_and_drawing_out() {
    let ports = <Generation2dPlayApp as ArtifactEditor>::media_ports().await;
    assert!(ports.iter().any(|port| port.id == "document:in"));
    assert!(ports.iter().any(|port| port.id == "document:out"));
    let params_in = ports.iter().find(|port| port.id == "params:in").expect("params:in declared");
    assert_eq!(params_in.media_type, MediaType { class: MediaClass::Data, form: MediaForm::Value });
    let drawing_out = ports.iter().find(|port| port.id == "drawing:out").expect("drawing:out declared");
    assert_eq!(drawing_out.media_type, MediaType { class: MediaClass::TwoD, form: MediaForm::Vector });
    assert_eq!(drawing_out.kind_id.as_deref(), Some("2d.drawing"));
}

#[test]
fn generation2d_io_declares_the_params_and_drawing_ports() {
    let io = semio_framework::io::resolve_ready(generation2d_io());
    assert_eq!(io.document_schema, "generation.2d");
    let params = io.ports.iter().find(|port| port.id == "params:in").expect("params:in declared");
    assert!(!params.required);
    let drawing = io.ports.iter().find(|port| port.id == "drawing:out").expect("drawing:out declared");
    assert_eq!(drawing.kind_id.as_deref(), Some("2d.drawing"));
}
//#endregion 🔖️PortTests
