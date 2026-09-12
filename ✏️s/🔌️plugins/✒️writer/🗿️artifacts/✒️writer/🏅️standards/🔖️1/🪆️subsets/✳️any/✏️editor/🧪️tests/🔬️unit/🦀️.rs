pub(crate) mod context {
    use super::super::*;
    use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry as framework_new_app_with_registry};
    use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};
    
    pub const WRITER_TEST_WINDOW_ID: &str = "writer-main-test";
    
    pub fn main_window_view() -> ViewModel {
        ViewModel { window_id: Some(WRITER_TEST_WINDOW_ID.into()), window_instances: vec![ViewWindowInstance { id: WRITER_TEST_WINDOW_ID.into(), window_kind_id: WRITER_PLAY_WINDOW_KIND.into() }], ..Default::default() }
    }
    
    /// WriterPlayApp implements the AUTHORING trait ArtifactEditor, not the runtime ArtifactApp --
    /// EditorApp<WriterPlayApp> (SDK adapter, contract 2.1) is the real ArtifactApp implementor
    /// VcsArtifactApp wraps, the same way PluginBuilder::editor::<WriterPlayApp> builds it.
    pub type WriterApp = VcsArtifactApp<EditorApp<WriterPlayApp>>;
    
    /// 🧪️ Constructs the Writer app with its declared command registry.
    pub async fn new_app() -> WriterApp {
        framework_new_app_with_registry::<EditorApp<WriterPlayApp>>(writer_app_manifest_for_tests).await
    }
    
    /// Adapts create_writer_app's AppDefinition (contract 2.4) into the App { definition, examples }
    /// shape artifact_app_laws::new_app_with_registry/assert_declared_actions_bridge_to_commands still expect --
    /// framework test context gap (framework crate outside this packet's lease), not modifiable here.
    fn writer_app_manifest_for_tests() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_writer_app(), examples: Vec::new() }
    }
    
    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub async fn new_app_with_registry() -> WriterApp {
        framework_new_app_with_registry::<EditorApp<WriterPlayApp>>(writer_app_manifest_for_tests).await
    }
    
    /// ✍️ Loads the canonical jack fixture into the store, returning the app ready to exercise.
    /// 🌱️ Whole-document replace is not an in-history mutation (`SetSnapshot` is banned outright —
    /// see `reset_document_effect`'s doc comment), so `setActiveExample` no longer lands via
    /// `dispatch_typed` alone; this loads the same document pack a real host would apply from that
    /// command's `Effect::LoadDocument`, via `PluginApp::load_document_pack` directly — the same
    /// technique `📐️cad`'s own `two_instances_converge_disjoint_edits_via_backbone` test uses.
    pub async fn app_with_jack() -> WriterApp {
        let mut app = new_app().await;
        let document = crate::document_dsl::jack_example_document();
        let (schema, id) = (document.schema.clone(), document.id.clone());
        let envelope = store::create_document_envelope::<WriterSnapshot, WriterMutation>(&schema, &id, document, None);
        let files = store::print_document_pack(&envelope).await.expect("print jack document pack");
        app.load_document_pack(&files).await.expect("load jack");
        app
    }
    
    pub async fn dispatch(app: &mut WriterApp, command: WriterCommand) -> InvocationResult {
        let mut meta = meta("local");
        meta.view_state = Some(main_window_view());
        let result = app.dispatch_typed(command, &meta).await.expect("dispatch");
        drain_typed_operations(app).await;
        result
    }
    
    /// 🚰️ Completes every admitted retained operation and acknowledges its bounded output pages.
    pub async fn drain_typed_operations(app: &mut WriterApp) {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
        while app.has_pending_typed_operations() {
            assert!(std::time::Instant::now() < deadline, "Writer retained operations did not finish");
            app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("Writer retained maintenance");
            app.advance_typed_operation_publication().await.expect("Writer retained publication");
            if let Some(page) = app.take_typed_operation_result_page(1) {
                let lane = page.lane;
                let bytes = page.bytes().to_vec();
                app.acknowledge_typed_operation_result(page.token).expect("Writer retained output acknowledgement");
                assert_ne!(lane, semio_framework_plugin::app::TypedOperationResultLane::Fault, "Writer retained publication fault: {bytes:?}");
            }
            app.take_typed_operation_effect();
            app.take_typed_operation_event();
            app.take_typed_operation_ui_scope();
            std::thread::yield_now();
        }
    }
    
    pub async fn render(app: &mut WriterApp, body_key: &str) -> String {
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(app.render(body_key, None, &main_window_view()).await.expect("render")).expect("render json")
    }
    
    pub async fn main_window_measures(app: &mut WriterApp) -> Vec<WindowMeasure> {
        app.window_measures(&main_window_view()).await.get(WRITER_TEST_WINDOW_ID).cloned().expect("main window measures")
    }
}

use super::*;
use crate::editor::writer::unit_tests::context::{new_app_with_registry, WriterApp};
use semio_framework_plugin::PluginApp;

async fn context_menu_items(app: &mut WriterApp, surface: Option<semio_framework_plugin::ContextMenuSurfaceTarget>) -> Value {
    let request = ContextMenuRequest { menu: semio_framework_plugin::UiMenuRef { id: "writer.play".into(), args: None }, surface, window_instance_id: None, point: None };
    serde_json::to_value(app.context_menu(&request, &semio_framework_plugin::ViewModel::default()).await).unwrap_or(Value::Null)
}

fn writer_envelope_wire() -> Vec<u8> {
    let envelope = store::create_document_envelope(WRITER_DOCUMENT_SCHEMA, "writer-live-load", crate::schema::empty_writer_snapshot(), None);
    let wire = dsl::os_pack::json::to_json_string(&envelope.capture_read().expect("Writer fixture envelope read")).into_bytes();
    let mut retirement = crate::spr::writer_envelope_decode_owner_bundle().retire_envelope(envelope);
    for _ in 0..10_000 {
        match retirement.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("Writer fixture envelope retirement") {
            store::SnapshotRetirementStep::Complete => {
                assert!(retirement.terminal_is_empty());
                drop(retirement);
                return wire;
            }
            store::SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES);
            }
            store::SnapshotRetirementStep::Blocked => panic!("unshared Writer fixture envelope retirement blocked"),
        }
    }
    panic!("Writer fixture envelope retirement did not reach terminal")
}

#[test]
fn interactive_job_fixture_matches_the_exact_factory_join() {
    let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎬️writer-migration/🔣️.json")).expect("language-neutral Writer migration fixture");
    assert_eq!(fixture["payloadSchema"], WRITER_COMMAND_PAYLOAD_SCHEMA);
    assert_eq!(fixture["maxRawWireBytes"], MAX_WRITER_COMMAND_RAW_BYTES);
    assert_eq!(fixture["maxWorkUnitsPerStep"], 1);
    let actions = fixture["migrated"].as_array().expect("migrated action rows").iter().map(|row| row["action"].as_str().expect("action id")).collect::<Vec<_>>();
    assert_eq!(actions, WRITER_COMMAND_TOOL_IDS);
    assert_eq!(fixture["batchOnly"].as_array().map(Vec::len), Some(0));
    assert_eq!(fixture["artifactPreparation"]["maxBaseBytes"], WRITER_ARTIFACT_STORE_MAXIMUM_BYTES);
    assert_eq!(fixture["artifactPreparation"]["maxEditTextBytes"], MAX_WRITER_COMMAND_TEXT_BYTES);
    assert_eq!(fixture["artifactPreparation"]["workItemsPerAdvance"], 1);
    assert_eq!(fixture["artifactPreparation"]["sealedByStore"], true);
    assert_eq!(fixture["requiredLifecycle"].as_array().map(Vec::len), Some(10));
}

#[test]
fn writer_artifact_store_preparation_is_exact_bounded_and_reversible() {
    let base = crate::writer_snapshot_with_text(WRITER_DOCUMENT_SCHEMA, "writer", "plaintext", "writer://document", "before");
    let mutation = WriterMutation::EditText(crate::op::EditText { text: "after".into() });
    let footprint = admit_writer_artifact_mutation(&mutation).expect("bounded Writer Artifact mutation");
    assert_eq!(footprint.work_items, 1);
    assert_eq!(footprint.retained_bytes, 5);
    let (post, inverse, forward) = prepare_writer_artifact(&base, mutation.clone()).expect("exact Writer Artifact preparation");
    assert_eq!(writer_text(&post), "after");
    assert_eq!(forward, mutation);
    assert_eq!(inverse, vec![WriterMutation::EditText(crate::op::EditText { text: "before".into() })]);
    assert!(admit_writer_artifact_mutation(&WriterMutation::EditText(crate::op::EditText { text: "x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES + 1) })).is_err());
    assert!(admit_writer_artifact_mutation(&WriterMutation::RenameWriter(crate::op::RenameWriter { new_id: "other".into() })).is_err());
}

#[test]
fn retained_wire_decoder_and_third_party_serde_have_command_parity() {
    let snapshot_json = dsl::os_pack::json::to_json_string(&crate::schema::empty_writer_snapshot());
    let commands = vec![
        WriterCommand::TextEdit(text_edit::TextEdit { text: "ä".into() }),
        WriterCommand::SetText(set_text::SetText { text: "bounded".into() }),
        WriterCommand::SetCamera(set_camera::SetCamera { camera: crate::WriterCamera { x: 1.0, y: 2.0, zoom: 3.0 } }),
        WriterCommand::RequestCompletions(request_completions::RequestCompletions {}),
        WriterCommand::LintDocument(lint_document::LintDocument {}),
        WriterCommand::SetEditorSelection(set_editor_selection::SetEditorSelection { start: 1, end: 2 }),
        WriterCommand::ToggleLineNumbers(toggle_line_numbers::ToggleLineNumbers {}),
        WriterCommand::SetFontPx(set_font_px::SetFontPx { value: 14 }),
        WriterCommand::SetLineHeight(set_line_height::SetLineHeight { value: 20 }),
        WriterCommand::SetTabSize(set_tab_size::SetTabSize { value: 4 }),
        WriterCommand::EngagementInput(engagement_input::EngagementInput { value: "Format".into() }),
        WriterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "jack".into() }),
        WriterCommand::SetSnapshot(set_snapshot::SetSnapshot { json: snapshot_json.clone() }),
        WriterCommand::OpenDocument(open_document::OpenDocument { uri: "writer://brief.md".into(), text: "# Brief".into() }),
        WriterCommand::SetSnapshotJson(set_snapshot_json::SetSnapshotJson { json: snapshot_json.clone() }),
        WriterCommand::SetFixtureJson(set_fixture_json::SetFixtureJson { json: snapshot_json }),
        WriterCommand::FormatDocument(format_document::FormatDocument {}),
        WriterCommand::CommitRename(commit_rename::CommitRename { text: "renamed".into() }),
        WriterCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some("format".into()) }),
    ];
    for command in commands {
        let wire = <WriterCommand as protocol::OpBinary>::encode_op(&command).expect("owned protocol wire");
        assert!(wire.len() <= MAX_WRITER_COMMAND_RAW_BYTES);
        assert_eq!(<WriterCommand as protocol::OpBinary>::decode_op(&wire).expect("owned retained decoder"), command);
        let owned_wire = dsl::os_pack::json::to_json_string(&command);
        let oracle: Value = serde_json::from_str(&owned_wire).expect("third-party JSON decoder");
        let serde_wire = serde_json::to_string(&oracle).expect("third-party JSON encoder");
        assert_eq!(dsl::os_pack::json::from_json_str::<WriterCommand>(&serde_wire).expect("owned command decoder"), command);
    }
}

fn writer_command_job(command: WriterCommand, text: Arc<str>) -> WriterCommandToolJob {
    WriterCommandToolJob {
        command: Some(command),
        snapshot: Some(Arc::new(crate::schema::empty_writer_snapshot())),
        text: Some(text),
        view_state: Some(context::main_window_view()),
        window_config: Some(WriterMainWindowConfig::default()),
        window_transient: Some(WriterMainWindowTransient::default()),
        completion: None,
        pending_completion_rejection: None,
        raw_input: None,
        raw_bytes: vec![1, 2, 3],
        raw_page_cursor: 2,
        raw_scan_cursor: 1,
        raw_validated: true,
        text_admitted: false,
        completed: false,
        closing: false,
    }
}

#[test]
fn writer_completion_rejection_retires_child_before_command_without_reemission() {
    let mut emit: Emit<WriterMutation, NoConfigMutation, NoDraftMutation> = Emit::default();
    emit.child_emits.push(semio_framework_plugin::app::ChildEmit::of::<WriterSnapshot, WriterMutation>("member", "writer-child", &[]));
    let rejected = ArtifactToolCompletionRejection::<EditorApp<WriterPlayApp>> {
        emit: Ok(emit),
        ephemeral: EphemeralEmit::default(),
        fault: Fault::new(semio_framework_plugin::FaultOrigin::Framework, semio_framework_plugin::FaultCode::new("test.completion-rejected"), "injected completion rejection"),
    };
    let mut job = writer_command_job(WriterCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some("lint".into()) }), Arc::from("writer text"));
    job.raw_bytes = Vec::new();
    job.pending_completion_rejection = Some(rejected);
    job.begin_close();
    assert_eq!(job.close_step(0, 1), InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 });
    assert!(job.pending_completion_rejection.is_some());
    assert!(job.command.is_some());
    for _ in 0..128 {
        if job.pending_completion_rejection.is_none() {
            break;
        }
        let step = job.close_step(1, 4);
        if let InteractiveJobCloseStep::Pending { released_items, released_bytes } = step {
            assert!(released_items <= 1 && released_bytes <= 4);
        }
    }
    assert!(job.pending_completion_rejection.is_none());
    assert!(job.command.is_some(), "typed command owner stays retained until the rejected output is terminal");
    for _ in 0..16 {
        if job.terminal_is_empty() {
            break;
        }
        let _ = job.close_step(1, MAX_WRITER_COMMAND_RAW_BYTES);
    }
    assert!(job.terminal_is_empty());
}

#[test]
fn bounded_text_admission_preserves_rejected_job_state_and_owners() {
    for command in [
        WriterCommand::TextEdit(text_edit::TextEdit { text: "changed".into() }),
        WriterCommand::SetText(set_text::SetText { text: "changed".into() }),
        WriterCommand::FormatDocument(format_document::FormatDocument {}),
        WriterCommand::CommitRename(commit_rename::CommitRename { text: "renamed".into() }),
        WriterCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some("format".into()) }),
    ] {
        let maximum: Arc<str> = Arc::from("x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES));
        let mut accepted = writer_command_job(command, maximum.clone());
        let accepted_cursor = (accepted.raw_page_cursor, accepted.raw_scan_cursor, accepted.raw_bytes.clone());
        assert!(accepted.admit_text());
        assert!(accepted.text_admitted);
        assert_eq!((accepted.raw_page_cursor, accepted.raw_scan_cursor, accepted.raw_bytes.clone()), accepted_cursor);
        assert_eq!(Arc::strong_count(&maximum), 2);

        let over: Arc<str> = Arc::from("x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES + 1));
        let mut rejected = writer_command_job(accepted.command.take().expect("accepted command owner"), over.clone());
        let rejected_cursor = (rejected.raw_page_cursor, rejected.raw_scan_cursor, rejected.raw_bytes.clone());
        let rejected_command = rejected.command.clone();
        let rejected_snapshot = rejected.snapshot.clone();
        let rejected_window_config = rejected.window_config.clone();
        assert!(!rejected.admit_text());
        assert!(!rejected.text_admitted);
        assert_eq!((rejected.raw_page_cursor, rejected.raw_scan_cursor, rejected.raw_bytes.clone()), rejected_cursor);
        assert_eq!(rejected.command, rejected_command);
        assert!(Arc::ptr_eq(rejected.snapshot.as_ref().expect("snapshot owner"), rejected_snapshot.as_ref().expect("saved snapshot owner")));
        assert_eq!(rejected.window_config, rejected_window_config);
        assert_eq!(Arc::strong_count(&over), 2);
    }

    let over: Arc<str> = Arc::from("x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES + 1));
    let mut lint = writer_command_job(WriterCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some("lint".into()) }), over);
    assert!(lint.admit_text());
}

#[test]
fn bounded_open_document_admission_preserves_maximum_plus_one_job_state_and_owners() {
    let current: Arc<str> = Arc::from("");
    let accepted_command = WriterCommand::OpenDocument(open_document::OpenDocument { uri: "u".repeat(MAX_WRITER_COMMAND_URI_BYTES), text: "x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES) });
    let mut accepted = writer_command_job(accepted_command, current.clone());
    assert!(accepted.admit_text());
    assert_eq!(accepted.emit().expect("bounded open document emission").0.effects.len(), 1);

    for rejected_command in [
        WriterCommand::OpenDocument(open_document::OpenDocument { uri: "u".repeat(MAX_WRITER_COMMAND_URI_BYTES), text: "x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES + 1) }),
        WriterCommand::OpenDocument(open_document::OpenDocument { uri: "u".repeat(MAX_WRITER_COMMAND_URI_BYTES + 1), text: "x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES) }),
    ] {
        let mut rejected = writer_command_job(rejected_command, current.clone());
        let rejected_cursor = (rejected.raw_page_cursor, rejected.raw_scan_cursor, rejected.raw_bytes.clone());
        let rejected_command = rejected.command.clone();
        let rejected_snapshot = rejected.snapshot.clone();
        let rejected_window_config = rejected.window_config.clone();
        assert!(!rejected.admit_text());
        assert_eq!((rejected.raw_page_cursor, rejected.raw_scan_cursor, rejected.raw_bytes.clone()), rejected_cursor);
        assert_eq!(rejected.command, rejected_command);
        assert!(Arc::ptr_eq(rejected.snapshot.as_ref().expect("snapshot owner"), rejected_snapshot.as_ref().expect("saved snapshot owner")));
        assert_eq!(rejected.window_config, rejected_window_config);
    }
}

#[test]
fn bounded_host_load_and_engagement_admission_reject_plus_one_without_consuming_owners() {
    let current: Arc<str> = Arc::from("");
    let accepted = [
        WriterCommand::TextEdit(text_edit::TextEdit { text: "x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES) }),
        WriterCommand::SetText(set_text::SetText { text: "x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES) }),
        WriterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "x".repeat(MAX_WRITER_EXAMPLE_ID_BYTES) }),
        WriterCommand::SetSnapshot(set_snapshot::SetSnapshot { json: "x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES) }),
        WriterCommand::SetSnapshotJson(set_snapshot_json::SetSnapshotJson { json: "x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES) }),
        WriterCommand::SetFixtureJson(set_fixture_json::SetFixtureJson { json: "x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES) }),
        WriterCommand::EngagementInput(engagement_input::EngagementInput { value: "x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES) }),
        WriterCommand::CommitRename(commit_rename::CommitRename { text: "x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES) }),
        WriterCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some("x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES)) }),
    ];
    for command in accepted {
        let mut job = writer_command_job(command, current.clone());
        assert!(job.admit_text());
    }
    let rejected = [
        WriterCommand::TextEdit(text_edit::TextEdit { text: "x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES + 1) }),
        WriterCommand::SetText(set_text::SetText { text: "x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES + 1) }),
        WriterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "x".repeat(MAX_WRITER_EXAMPLE_ID_BYTES + 1) }),
        WriterCommand::SetSnapshot(set_snapshot::SetSnapshot { json: "x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES + 1) }),
        WriterCommand::SetSnapshotJson(set_snapshot_json::SetSnapshotJson { json: "x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES + 1) }),
        WriterCommand::SetFixtureJson(set_fixture_json::SetFixtureJson { json: "x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES + 1) }),
        WriterCommand::EngagementInput(engagement_input::EngagementInput { value: "x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES + 1) }),
        WriterCommand::CommitRename(commit_rename::CommitRename { text: "x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES + 1) }),
        WriterCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some("x".repeat(MAX_WRITER_COMMAND_TEXT_BYTES + 1)) }),
    ];
    for command in rejected {
        let mut job = writer_command_job(command, current.clone());
        let command_owner = job.command.clone();
        let snapshot_owner = job.snapshot.clone();
        let window_config_owner = job.window_config.clone();
        assert!(!job.admit_text());
        assert_eq!(job.command, command_owner);
        assert!(Arc::ptr_eq(job.snapshot.as_ref().expect("snapshot owner"), snapshot_owner.as_ref().expect("saved snapshot owner")));
        assert_eq!(job.window_config, window_config_owner);
    }
}

fn admit_writer_envelope(app: &mut WriterApp, wire: &[u8]) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle {
    let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
    let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("Writer live envelope ingress credits");
    for chunk in wire.chunks(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
        let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
        bytes[..chunk.len()].copy_from_slice(chunk);
        let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded Writer live envelope page");
        app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("Writer live envelope page admission failed: {fault:?}"));
    }
    assert!(app.seal_artifact_envelope_ingress(handle).expect("Writer live envelope seal/submit"));
    handle
}

fn drive_writer_live_load(app: &mut WriterApp, handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll {
    for _ in 0..100_000 {
        app.maintenance_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("one Writer live maintenance turn");
        let poll = app.advance_artifact_envelope_load(handle).expect("Writer live load advancement");
        if matches!(poll, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Cancelled | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault) {
            return poll;
        }
        std::thread::yield_now();
    }
    panic!("Writer live envelope load did not reach terminal")
}

#[semio_framework_async_macros::async_test]
async fn writer_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed() {
    let mut app = artifact_app_laws::new_app().await;
    let base_generation = app.artifact_generation_now();
    let handle = admit_writer_envelope(&mut app, &writer_envelope_wire());
    assert_eq!(handle.generation, base_generation);
    assert_eq!(drive_writer_live_load(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
    assert_eq!(app.artifact_generation_now().0, base_generation.0 + 1);
    assert!(app.acknowledge_artifact_store_replacement(handle).expect("first exact Writer load acknowledgement"));
    assert!(!app.acknowledge_artifact_store_replacement(handle).expect("duplicate Writer load acknowledgement is a no-op"));
}

#[semio_framework_async_macros::async_test]
async fn writer_live_envelope_cancel_closes_retained_pages_without_publication() {
    let mut app = artifact_app_laws::new_app().await;
    let base_generation = app.artifact_generation_now();
    let wire = writer_envelope_wire();
    let pages = wire.len().div_ceil(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
    let handle = app.begin_artifact_envelope_ingress(pages, wire.len()).expect("cancelled Writer ingress credits");
    let first = &wire[..wire.len().min(store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)];
    let mut bytes = [0; store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
    bytes[..first.len()].copy_from_slice(first);
    let page = store::ArtifactEnvelopeDecodePage::try_from_array(bytes, first.len()).expect("cancelled Writer first page");
    app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("cancelled Writer page admission failed: {fault:?}"));
    app.cancel_artifact_envelope_load(handle).expect("cancel exact Writer ingress");
    assert_eq!(drive_writer_live_load(&mut app, handle), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
    assert_eq!(app.artifact_generation_now(), base_generation);
}

#[semio_framework_async_macros::async_test]
async fn jack_completions_use_example_fixture() {
    let json = crate::standards::v1::subsets::any::schema::jack_completions_json("RETURN a.", 9).unwrap_or_default();
    assert!(!json.is_empty());
}

//#region 🔖️CommandSurface
/// 🏷️ Every declared manifest action id must be reachable as exactly one command row (`setEditorSetting`
/// legitimately covers three rows — see the `app_commands!` doc comment above), and every row's wire
/// keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
#[semio_framework_async_macros::async_test]
async fn command_surface_has_the_expected_row_count_and_distinct_wire_keywords() {
    let commands = every_command();
    assert_eq!(commands.len(), 21, "every WriterCommand row must be covered by every_command()");
    let mut keywords: Vec<String> = commands.iter().map(|command| protocol::OpText::print_op(command).split(' ').next().unwrap_or_default().to_string()).collect();
    keywords.sort();
    keywords.dedup();
    assert_eq!(keywords.len(), commands.len(), "every row's wire keyword must be distinct");
}

/// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — what a
/// missing `#[dsl(keyword = ..)]` on a payload struct silently breaks (the record prints with no
/// keyword at all and no longer parses).
#[semio_framework_async_macros::async_test]
async fn every_printed_op_line_starts_with_the_rows_declared_wire_keyword() {
    let expectations: Vec<(&str, WriterCommand)> = vec![
        ("text-edit", WriterCommand::TextEdit(text_edit::TextEdit { text: "x".into() })),
        ("set-text", WriterCommand::SetText(set_text::SetText { text: "x".into() })),
        ("set-snapshot", WriterCommand::SetSnapshot(set_snapshot::SetSnapshot { json: "{}".into() })),
        ("open-document", WriterCommand::OpenDocument(open_document::OpenDocument { uri: "writer://jack".into(), text: "x".into() })),
        ("document-json", WriterCommand::SetSnapshotJson(set_snapshot_json::SetSnapshotJson { json: "{}".into() })),
        ("fixture-json", WriterCommand::SetFixtureJson(set_fixture_json::SetFixtureJson { json: "{}".into() })),
        ("active-example", WriterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "jack".into() })),
        ("format-document", WriterCommand::FormatDocument(format_document::FormatDocument {})),
        ("commit-rename", WriterCommand::CommitRename(commit_rename::CommitRename { text: "x".into() })),
        ("camera", WriterCommand::SetCamera(set_camera::SetCamera { camera: crate::WriterCamera::default() })),
        ("request-completions", WriterCommand::RequestCompletions(request_completions::RequestCompletions {})),
        ("lint-document", WriterCommand::LintDocument(lint_document::LintDocument {})),
        ("editor-selection", WriterCommand::SetEditorSelection(set_editor_selection::SetEditorSelection { start: 0, end: 1 })),
        ("toggle-line-numbers", WriterCommand::ToggleLineNumbers(toggle_line_numbers::ToggleLineNumbers {})),
        ("font-px", WriterCommand::SetFontPx(set_font_px::SetFontPx { value: 16 })),
        ("line-height", WriterCommand::SetLineHeight(set_line_height::SetLineHeight { value: 24 })),
        ("tab-size", WriterCommand::SetTabSize(set_tab_size::SetTabSize { value: 4 })),
        ("engagement-input", WriterCommand::EngagementInput(engagement_input::EngagementInput { value: "x".into() })),
        ("engagement-submit", WriterCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some("x".into()) })),
    ];
    for (expected_keyword, command) in expectations {
        let printed = protocol::OpText::print_op(&command);
        assert_eq!(printed.split(' ').next().unwrap_or_default(), expected_keyword, "wire keyword drifted for {command:?}: {printed:?}");
    }
}

/// ✍️ Hand-built representative document — used across the app's own command-surface tests.
fn jack_snapshot() -> WriterSnapshot {
    crate::writer_snapshot_with_text("writer.document", "jack", "jack", "writer://jack", "MATCH (a:Piece)-[r:Connection]->(b:Piece)\nWHERE a.name = \"core\"\nRETURN a.name, b.name")
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub(super) fn every_command() -> Vec<WriterCommand> {
    vec![
        WriterCommand::TextEdit(text_edit::TextEdit { text: "hello".into() }),
        WriterCommand::SetText(set_text::SetText { text: "MATCH (a) RETURN a".into() }),
        WriterCommand::SetSnapshot(set_snapshot::SetSnapshot { json: "{}".into() }),
        WriterCommand::OpenDocument(open_document::OpenDocument { uri: "writer://jack".into(), text: String::new() }),
        WriterCommand::SetSnapshotJson(set_snapshot_json::SetSnapshotJson { json: "{}".into() }),
        WriterCommand::SetFixtureJson(set_fixture_json::SetFixtureJson { json: "{}".into() }),
        WriterCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "jack".into() }),
        WriterCommand::FormatDocument(format_document::FormatDocument {}),
        WriterCommand::CommitRename(commit_rename::CommitRename { text: "piece".into() }),
        WriterCommand::SetCamera(set_camera::SetCamera { camera: crate::WriterCamera { x: 1.0, y: 2.0, zoom: 1.5 } }),
        WriterCommand::RequestCompletions(request_completions::RequestCompletions {}),
        WriterCommand::LintDocument(lint_document::LintDocument {}),
        WriterCommand::SetEditorSelection(set_editor_selection::SetEditorSelection { start: 3, end: 7 }),
        WriterCommand::ToggleLineNumbers(toggle_line_numbers::ToggleLineNumbers {}),
        WriterCommand::SetFontPx(set_font_px::SetFontPx { value: 16 }),
        WriterCommand::SetLineHeight(set_line_height::SetLineHeight { value: 24 }),
        WriterCommand::SetTabSize(set_tab_size::SetTabSize { value: 4 }),
        WriterCommand::EngagementInput(engagement_input::EngagementInput { value: "format".into() }),
        WriterCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: None }),
    ]
}

/// ⚖️ The rows whose `Option` fields make `None`/`Some` distinct wire cases, pinned to the exact
/// bytes captured from the pre-merge `writer_protocol` crate (this ticket's
/// `🧪️wire-baseline-before.txt`, row 22 — rows 15/16 (`ast-hover`/`text-hover`) dissolved into the
/// framework's own `ast` interaction domain and no longer exist as writer commands). A regression
/// here is a real format break, not a test-fixture mismatch.
#[semio_framework_async_macros::async_test]
async fn optional_field_rows_keep_their_pre_migration_bytes() {
    let cases: [(WriterCommand, &str, &str); 1] = [(WriterCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: None }), "engagement-submit engagement-submit", "01120000")];
    for (command, text, hex) in cases {
        assert_eq!(protocol::OpText::print_op(&command), text);
        assert_eq!(protocol::OpBinary::encode_op(&command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), hex);
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}
//#endregion 🔖️CommandSurface

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let json = serde_json::to_string(&create_writer_app()).expect("app definition json");
    assert!(json.contains(WRITER_PLAY_WINDOW_KIND), "window kind missing from the manifest: {json}");
    assert!(json.contains(edit::WRITER_PLAY_MODE_EDIT), "mode missing from the manifest");
    for body in [WRITER_PLAY_BODY_ARTIFACT, WRITER_PLAY_BODY_CATALOGUE, WRITER_PLAY_BODY_INSPECTION] {
        assert!(json.contains(body), "panel body {body} missing from the manifest");
    }
    assert!(json.contains("text.document"), "artifact kind missing from the manifest");
}

/// SDK GAP (contract 2.4): create_writer_app() now returns a bare AppDefinition -- .example(...)
/// does not exist on Editor::builder, so the "jack"/"dag.jack" example registrations this test
/// used to assert on no longer exist to assert on (App{definition,examples} no longer flows through
/// this builder path at all). Deleted rather than left compiling against a field that is gone;
/// tracked as a real regression in the migration notes, not silently dropped.
//#endregion 🔖️ManifestSanity

//#region 🔖️Interaction
/// 🕹️ The `ast` domain is declared `HierarchyProvider::Topology`, transitive on both hover and
/// selection, and scoped to writer's one window kind — the manifest side of THE TRANSITIVE
/// TEMPLATE (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
#[semio_framework_async_macros::async_test]
async fn ast_interaction_domain_is_declared_topology_and_transitive_on_the_main_window() {
    let definition = create_writer_app();
    let ast = definition.interactions.iter().find(|interaction| interaction.id == "ast").expect("ast interaction domain declared");
    assert!(matches!(ast.hierarchy, HierarchyProvider::Topology));
    assert!(ast.hover.transitive, "ast hover must be transitive for the covering-node behavior");
    assert!(ast.selection.transitive, "ast selection must be transitive for the covering-node behavior");
    let main_window = definition.window_kinds.iter().find(|window| window.id == WRITER_PLAY_WINDOW_KIND).expect("main window kind declared");
    assert!(main_window.interactions.iter().any(|interaction_ref| interaction_ref.as_str() == "ast"), "main window must reference the ast interaction domain");
}

/// 🌳️ `interaction_topology` walks the jack AST's own `children` into `TopologyNode.parent` links —
/// root has no parent, every child's parent is its syntactic parent's id.
#[semio_framework_async_macros::async_test]
async fn interaction_topology_walks_the_jack_ast_into_parent_links() {
    let document = crate::document_dsl::jack_example_document();
    let config = NoConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let topology = WriterPlayApp::interaction_topology(&doc, &cfg);
    let ast = topology.domains.get("ast").expect("ast domain present in topology");
    assert!(!ast.ordered.is_empty(), "jack document must produce a non-empty ast topology");
    let root = &ast.ordered[0];
    assert!(root.parent.is_none(), "the first (pre-order) node is the AST root and has no parent");
    assert!(ast.ordered.iter().skip(1).all(|node| node.parent.is_some()), "every non-root node must carry its syntactic parent's id");
}

/// 🌱️ A non-jack document has no AST to select — an empty topology, matching `Flat`-vs-empty
/// pruning semantics: every stale `ast` selection id gets pruned for a document with no AST.
#[semio_framework_async_macros::async_test]
async fn interaction_topology_is_empty_for_non_jack_documents() {
    let document = crate::schema::empty_writer_snapshot();
    let config = NoConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let topology = WriterPlayApp::interaction_topology(&doc, &cfg);
    assert!(topology.domains.get("ast").expect("ast domain present in topology").ordered.is_empty());
}
//#endregion 🔖️Interaction

//#region 🔖️PortTests
#[semio_framework_async_macros::async_test]
async fn writer_io_declares_the_extra_text_out_port() {
    let io = writer_io();
    let ports = io.all_ports().await;
    assert!(ports.iter().any(|port| port.id == "document:in"));
    assert!(ports.iter().any(|port| port.id == "document:out"));
    let text_out = ports.iter().find(|port| port.id == "text:out").expect("text:out port declared");
    assert_eq!(text_out.kind_id.as_deref(), Some("text.document"));
    assert_eq!(text_out.multiplicity, semio_framework_plugin::PortMultiplicity::Many);
}

#[semio_framework_async_macros::async_test]
async fn export_media_text_out_projects_the_document_as_a_chapter() {
    let document = crate::document_dsl::jack_example_document();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_view = ArtifactView::new(&document, &history);
    let media = WriterPlayApp::export_media("text:out", &doc_view).expect("export text:out");
    let MediaPayload::Structured { schema, json } = media.payload else { panic!("expected structured payload") };
    assert_eq!(schema, "text.document");
    let payload: WriterChapterPayload = serde_json::from_str(&json).expect("decode chapter payload");
    assert_eq!(payload.text, writer_text(&document));
    assert_eq!(payload.language_id, document.language_id);
}

#[semio_framework_async_macros::async_test]
async fn export_media_rejects_unknown_ports() {
    let document = crate::schema::empty_writer_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_view = ArtifactView::new(&document, &history);
    assert!(matches!(WriterPlayApp::export_media("nonsense:out", &doc_view), Err(MediaError::NotImplemented)));
}
//#endregion 🔖️PortTests

//#region 🔖️ContextMenu
/// 🗂️ GROUPED-PROGRESSIVELY-DISCLOSED-CONTEXT-MENUS: the writer text-editor context menu stays a
/// shallow, disclosed list (top-level verbs + a handful of taxonomy groups) rather than a flat wall
/// of rows, and the destructive `cut` row stays the trailing item.
#[semio_framework_async_macros::async_test]
async fn context_menu_is_grouped_and_keeps_cut_last_and_destructive() {
    let document = crate::document_dsl::jack_example_document();
    let config = NoConfig::default();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&document, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let registry = AppActionRegistry::from_definition(&create_writer_app());
    let request = ContextMenuRequest {
        menu: semio_framework_plugin::UiMenuRef { id: WRITER_PLAY_BODY_MAIN.into(), args: None },
        surface: Some(semio_framework_plugin::ContextMenuSurfaceTarget {
            surface_id: "writer.play".into(),
            kind: "textEditor".into(),
            hits: Vec::new(),
            selection: Vec::new(),
            text: Some(ContextMenuTextContext { caret: 0, has_selection: true, word: None, can_rename: true, has_completions: true }),
        }),
        window_instance_id: None,
        point: None,
    };
    let items = WriterPlayApp::context_menu(&request, &doc, &cfg, &semio_framework_plugin::ViewModel::default(), &registry);
    assert!(items.len() <= 9, "top-level writer context menu should stay progressively disclosed: {items:?}");
    assert_eq!(items.last().map(|item| item.id.as_str()), Some("writer-cut"), "cut must stay the trailing destructive item: {items:?}");
    assert_eq!(items.last().and_then(|item| item.destructive), Some(true), "trailing writer-cut must be marked destructive: {items:?}");
}

#[semio_framework_async_macros::async_test]
async fn context_menu_via_the_registry_still_starts_with_select_token() {
    let mut app = new_app_with_registry().await;
    let menu = context_menu_items(&mut app, Some(semio_framework_plugin::ContextMenuSurfaceTarget { surface_id: "writer.play".into(), kind: "textEditor".into(), hits: vec![], selection: vec![], text: None })).await;
    assert!(menu.to_string().contains("writer-select-token"), "menu should be {menu}");
}
//#endregion 🔖️ContextMenu

//#region 🔖️CrossCutting
#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    use crate::editor::writer::unit_tests::context::{new_app, render};
    let mut app = new_app().await;
    assert!(render(&mut app, "writer.play.nope").await.contains("Unknown body"));
}

/// 🌱️ `SetSnapshot` is banned outright (see `whole_document_operation`'s doc comment) — the
/// trait default correctly returns `None`; whole-document replace goes through
/// `reset_document_effect` instead, exercised by `📚️examples/🎬️demo-session`'s own command
/// tests and by `commands::text`'s `set_active_example`/`open_document` tests.
#[semio_framework_async_macros::async_test]
async fn whole_document_operation_stays_the_trait_default_none() {
    let replacement = jack_snapshot();
    assert_eq!(WriterPlayApp::whole_document_operation(replacement), None);
}

#[semio_framework_async_macros::async_test]
async fn window_engagements_expose_format_lint_placeholder() {
    let mut app = artifact_app_laws::new_app().await;
    let engagements = app.window_engagements(&context::main_window_view()).await;
    let main = engagements.get(context::WRITER_TEST_WINDOW_ID).expect("main engagement");
    let placeholder = main.input.as_ref().and_then(|i| i.placeholder.as_ref()).expect("placeholder");
    assert!(placeholder.contains("Format"));
    assert_eq!(main.possible_engagements.as_ref().map(|v| v.len()), Some(3));
}

#[semio_framework_async_macros::async_test]
async fn window_engagements_include_format_and_lint_possible_engagements() {
    let mut app = artifact_app_laws::new_app().await;
    let engagements = app.window_engagements(&context::main_window_view()).await;
    let engagement = engagements.get(context::WRITER_TEST_WINDOW_ID).expect("writer window engagement");
    let ids: Vec<&str> = engagement.possible_engagements.as_ref().expect("possible engagements").iter().map(|possible| possible.id.as_str()).collect();
    assert!(ids.contains(&"writer-format"));
    assert!(ids.contains(&"writer-lint"));
}

/// 🗣️ Cross-cutting locale check across every rendering surface (inspection, catalogue,
/// engagements, measures) at once — narrower per-node locale tests live beside each node, but this
/// is the integration-level guarantee that locale threads through the whole app consistently.
#[semio_framework_async_macros::async_test]
async fn writer_labels_resolve_native_english_by_default_across_every_surface() {
    let mut app = artifact_app_laws::new_app().await;
    let inspection = app.render(WRITER_PLAY_BODY_INSPECTION, None, &semio_framework_plugin::ViewModel::default()).await.expect("render");
    let inspection_json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(inspection).expect("render JSON");
    assert!(inspection_json.contains("\"Document\""));
    assert!(inspection_json.contains("\"Camera\""));
    let catalogue = app.render(WRITER_PLAY_BODY_CATALOGUE, None, &semio_framework_plugin::ViewModel::default()).await.expect("render");
    let catalogue_json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(catalogue).expect("render JSON");
    assert!(catalogue_json.contains("\"Language\""));
    assert!(catalogue_json.contains("Cypher-inspired"));
    let engagements = app.window_engagements(&context::main_window_view()).await;
    let engagements_json = serde_json::to_string(&engagements).unwrap();
    assert!(engagements_json.contains("\"Format\""));
    assert!(engagements_json.contains("\"Lint\""));
    let measures = app.window_measures(&context::main_window_view()).await;
    let measures_json = serde_json::to_string(&measures).unwrap();
    assert!(measures_json.contains("Font size"));
    assert!(measures_json.contains("Line numbers"));
    assert!(!measures_json.contains("Schriftgröße"));
}

//#endregion 🔖️CrossCutting
