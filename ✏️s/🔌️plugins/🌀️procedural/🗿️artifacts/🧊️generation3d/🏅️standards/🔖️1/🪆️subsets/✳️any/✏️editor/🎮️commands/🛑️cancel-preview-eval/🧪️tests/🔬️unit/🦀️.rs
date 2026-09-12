use super::*;
use crate::editor::generation3d::testkit::{empty_history_view, retire_flow_eval_session};
use semio_framework_plugin::{ArtifactView, ConfigView};

const PREVIEW_CANCEL_FIXTURE_JSON: &str = include_str!("../../../../../🧫️fixtures/🛑️preview-cancel.json");
const GEOMETRY_EXTENSION_PLUGIN_ID: &str = "flow-extension-brep";
const CANCEL_WINDOW_ID: &str = "procedural-preview-test";

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct PreviewCancelFixture {
    format: String,
    version: u8,
    cancel_action: String,
    cancel_capability: Vec<String>,
    cancel_response_action: String,
    geometry_extension_id: String,
    phase_labels: std::collections::BTreeMap<String, PhaseLabel>,
    rows: Vec<PreviewCancelRow>,
    host_door: HostDoor,
}

#[derive(serde::Deserialize)]
struct PhaseLabel {
    en: String,
    de: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct PreviewCancelRow {
    id: String,
    sequence: Vec<PreviewCancelEvent>,
    status_before: ExpectedStatus,
    status_after: ExpectedStatus,
    invocations: Vec<ExpectedInvocation>,
    ticks_after_cancel: usize,
    chunk_cursor_after_cancel: u32,
    resumes_on_gesture: bool,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct PreviewCancelEvent {
    event: String,
    #[serde(default)]
    handle: String,
    #[serde(default)]
    phase: String,
    #[serde(default)]
    units_done: u32,
    #[serde(default)]
    units_total: u32,
    #[serde(default)]
    faces_done: u32,
    #[serde(default)]
    faces_total: u32,
    #[serde(default)]
    chunk: u32,
    #[serde(default)]
    chunks: u32,
    #[serde(default)]
    count: usize,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExpectedStatus {
    phase: String,
    cancellable: bool,
    units_done: u32,
    units_total: u32,
    faces_done: u32,
    faces_total: u32,
    in_flight: u32,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ExpectedInvocation {
    extension: String,
    capability: String,
    response_action: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct HostDoor {
    answer_status: String,
    fault_code: String,
    aborted_at_turn_boundary: bool,
}

fn preview_cancel_fixture() -> PreviewCancelFixture {
    let fixture: PreviewCancelFixture = serde_json::from_str(PREVIEW_CANCEL_FIXTURE_JSON).expect("preview cancel fixture");
    assert_eq!(fixture.format, "semio.generation3d.preview-cancel");
    assert_eq!(fixture.version, 1);
    fixture
}

fn node_hash(handle: &str) -> u64 {
    semio_framework_os_flow::preview_tessellate_node_hash(handle, 0.05_f64.to_bits())
}

/// 📈️ The status object the preview window would publish RIGHT NOW, read through the same pure
/// projection the surface uses, over a geometry address that is deliberately supplied rather than
/// looked up (see `preview_progress_status_json_for`'s own doc).
fn observed_status(session: &FlowEvalSession) -> serde_json::Value {
    serde_json::from_str(&crate::editor::generation3d::preview_progress_status_json_for(Some(session), Ok(GEOMETRY_EXTENSION_PLUGIN_ID.to_string()))).expect("status json")
}

fn assert_status(observed: &serde_json::Value, expected: &ExpectedStatus, labels: &std::collections::BTreeMap<String, PhaseLabel>, row: &str, when: &str) {
    assert_eq!(observed["phase"].as_str(), Some(expected.phase.as_str()), "{row}/{when}: phase");
    assert_eq!(observed["cancellable"].as_bool(), Some(expected.cancellable), "{row}/{when}: cancellable");
    assert_eq!(observed["progress"]["unitsDone"].as_u64(), Some(u64::from(expected.units_done)), "{row}/{when}: unitsDone");
    assert_eq!(observed["progress"]["unitsTotal"].as_u64(), Some(u64::from(expected.units_total)), "{row}/{when}: unitsTotal");
    assert_eq!(observed["progress"]["facesDone"].as_u64(), Some(u64::from(expected.faces_done)), "{row}/{when}: facesDone");
    assert_eq!(observed["progress"]["facesTotal"].as_u64(), Some(u64::from(expected.faces_total)), "{row}/{when}: facesTotal");
    assert_eq!(observed["progress"]["inFlight"].as_u64(), Some(u64::from(expected.in_flight)), "{row}/{when}: inFlight");
    if let Some(label) = labels.get(&expected.phase) {
        assert_eq!(observed["phaseLabel"]["en"].as_str(), Some(label.en.as_str()), "{row}/{when}: English label");
        assert_eq!(observed["phaseLabel"]["de"].as_str(), Some(label.de.as_str()), "{row}/{when}: German label — a surface carries both, with no default language");
    }
}

/// ⚖️ LAW: the whole cancellation contract, replayed from `🧫️fixtures/🛑️preview-cancel.json`.
///
/// For each row: the sequence runs against a real retained [`FlowEvalSession`], the status object is
/// read through the surface's own projection before and after the cancel, the emitted extension
/// invocations are compared against the fixture, and the chain is then proved to be quiescent (no
/// further tick is owed) yet resumable (a later gesture arms exactly one).
#[test]
fn the_cancel_gesture_obeys_its_fixture_end_to_end() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let fixture = preview_cancel_fixture();
    assert_eq!(fixture.cancel_action, "cancelPreviewEval");
    assert_eq!(fixture.geometry_extension_id, crate::preview_eval::GENERATION_3D_GEOMETRY_EXTENSION_ID);
    let snapshot = Generation3dSnapshot::default();
    let history = empty_history_view();
    let config = Generation3dConfig::default();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    for row in &fixture.rows {
        let mut session = FlowEvalSession::new();
        let mut emitted: Vec<(String, String, String)> = Vec::new();
        let mut cursor_handle = String::new();
        let mut saw_cancel = false;
        for event in &row.sequence {
            match event.event.as_str() {
                "admit" => {
                    assert!(session.note_pending_tessellate(node_hash(&event.handle), event.handle.clone()), "{}: {} must be admitted", row.id, event.handle);
                    cursor_handle = event.handle.clone();
                }
                "step" => {
                    let body = serde_json::json!({ "phase": event.phase, "unitsDone": event.units_done, "unitsTotal": event.units_total, "facesDone": event.faces_done, "facesTotal": event.faces_total });
                    session.resolve_preview_tessellate(node_hash(&event.handle), &body.to_string());
                }
                "chunk" => {
                    let body = serde_json::json!({ "phase": "complete", "unitsDone": 24, "unitsTotal": 24, "facesDone": 9, "facesTotal": 9, "chunk": event.chunk, "chunks": event.chunks, "meshPack": "AAAA" });
                    session.resolve_preview_tessellate(node_hash(&event.handle), &body.to_string());
                    cursor_handle = event.handle.clone();
                }
                "parkExtension" => session.note_window_extensions_in_flight(CANCEL_WINDOW_ID, event.count),
                "cancel" => {
                    if !saw_cancel {
                        assert_status(&observed_status(&session), &row.status_before, &fixture.phase_labels, &row.id, "before");
                        saw_cancel = true;
                    }
                    let payload = CancelPreviewEval { window_id: CANCEL_WINDOW_ID.into(), window_kind_id: crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into() };
                    let emit = handle(&payload, &doc, &cfg, &mut session).expect("cancelPreviewEval");
                    assert!(emit.effects.is_empty(), "{}: a cancel emits no self-redispatch", row.id);
                    for invocation in &emit.extension_invocations {
                        emitted.push((invocation.extension_id.clone(), invocation.capability.clone(), invocation.response_action.clone()));
                    }
                }
                "lateAnswer" => {
                    let body = serde_json::json!({ "phase": "complete", "unitsDone": 1, "unitsTotal": 1, "chunk": 0, "chunks": 1, "meshPack": "AAAA" });
                    assert_eq!(
                        session.resolve_preview_tessellate(node_hash(&event.handle), &body.to_string()),
                        semio_framework_os_flow::PreviewTessellateOutcome::Unknown,
                        "{}: a response for a retired job must be ignored",
                        row.id
                    );
                }
                other => panic!("{}: unknown preview-cancel event {other}", row.id),
            }
        }
        assert_status(&observed_status(&session), &row.status_after, &fixture.phase_labels, &row.id, "after");
        assert_eq!(emitted.len(), row.invocations.len(), "{}: emitted invocation count", row.id);
        for (observed, expected) in emitted.iter().zip(&row.invocations) {
            assert_eq!(observed.0, GEOMETRY_EXTENSION_PLUGIN_ID, "{}: an invocation addresses the OWNING PLUGIN of {}", row.id, expected.extension);
            assert_eq!(observed.1, expected.capability, "{}: capability", row.id);
            assert!(fixture.cancel_capability.contains(&observed.1), "{}: {} is one of the fixture's declared cancel capabilities {:?}", row.id, observed.1, fixture.cancel_capability);
            assert_eq!(observed.2, expected.response_action, "{}: response action", row.id);
            assert_eq!(observed.2, fixture.cancel_response_action, "{}: the fixture's one cancel response action", row.id);
        }
        assert!(!session.window_tick_is_armed(CANCEL_WINDOW_ID), "{}: a cancel arms nothing", row.id);
        assert_eq!(session.window_extensions_in_flight(CANCEL_WINDOW_ID), 0, "{}: the latch is quiesced", row.id);
        assert!(!session.window_tick_owed(CANCEL_WINDOW_ID), "{}: {} ticks may be owed after a cancel", row.id, row.ticks_after_cancel);
        if !cursor_handle.is_empty() {
            assert_eq!(session.next_tessellate_chunk(node_hash(&cursor_handle)), row.chunk_cursor_after_cancel, "{}: the chunk cursor is dropped with the body it addresses", row.id);
        }
        assert_eq!(session.arm_window_tick(CANCEL_WINDOW_ID), row.resumes_on_gesture, "{}: a later gesture must re-arm exactly one tick", row.id);
        session.begin_window_tick(CANCEL_WINDOW_ID);
        assert!(!session.preview_cancelled(), "{}: a tick that begins is work resuming, so the cancelled banner retires", row.id);
        retire_flow_eval_session(session);
    }
    assert_eq!(fixture.host_door.answer_status, "cancelled");
    assert_eq!(fixture.host_door.fault_code, "extension.request-cancelled");
    assert!(fixture.host_door.aborted_at_turn_boundary);
}

/// ⚖️ LAW: an unaddressable geometry extension has no actor to tell — the local half of the cancel
/// still happens and NOTHING is emitted, because an invocation nobody can route is a dispatch fault,
/// not a cancellation.
#[test]
fn an_unaddressable_kernel_cancels_locally_and_emits_nothing() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut session = FlowEvalSession::new();
    assert!(session.note_pending_tessellate(node_hash("brep:solid-9"), "brep:solid-9".into()));
    let payload = CancelPreviewEval { window_id: CANCEL_WINDOW_ID.into(), window_kind_id: crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into() };
    let invocations = crate::preview_eval::cancel_preview_eval_for(&payload, &mut session, Err(semio_framework_os_flow::FlowExtensionAddressMiss { extension_id: crate::preview_eval::GENERATION_3D_GEOMETRY_EXTENSION_ID.to_string(), contributed: Vec::new() }));
    assert!(invocations.is_empty(), "there is no actor to address");
    assert_eq!(session.preview_tessellate_status().in_flight, 0, "the local half of the cancel still happened");
    assert!(session.preview_cancelled());
    retire_flow_eval_session(session);
}
