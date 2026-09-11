use super::*;
use crate::editor::generation3d::testkit::{empty_history_view, retire_flow_eval_session};
use semio_framework_plugin::{ArtifactView, ConfigView};

/// 🔺️ One triangle as the `pack` mesh body the extension now ships — base64 of
/// `brep_geometry::encode_mesh_pack`, not a JSON number array.
fn tessellated_triangle_pack() -> String {
    let mesh = semio_framework_plugin::MeshData { positions: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0], indices: vec![0, 1, 2], ..Default::default() };
    semio_framework_os_flow::brep_geometry::encode_base64(&semio_framework_os_flow::brep_geometry::encode_mesh_pack(&mesh).expect("mesh pack"))
}

/// 📨️ One complete-in-a-single-chunk `tessellate` response envelope.
fn complete_envelope(pack: &str) -> String {
    format!(r#"{{"done":true,"cancellable":false,"phase":"complete","unitsDone":3,"unitsTotal":3,"facesDone":1,"facesTotal":1,"chunk":0,"chunks":1,"meshPack":"{pack}"}}"#)
}

/// ⚖️ LAW: a `tessellate` extension result carried back as `flowTessellateResolve` lands on exactly the
/// geometry handle `preview_tessellate_invocations` recorded for that `nodeHash`, clearing the
/// in-flight entry — the half of the brep preview round trip that lives inside the app.
#[test]
fn tessellate_result_resolves_the_pending_handle() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let snapshot = Generation3dSnapshot::default();
    let history = empty_history_view();
    let config = Generation3dConfig::default();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let mut session = FlowEvalSession::new();
    let pack = tessellated_triangle_pack();
    let node_hash = semio_framework_os_flow::preview_tessellate_node_hash("brep:solid-1", 0.01_f64.to_bits());
    assert!(session.note_pending_tessellate(node_hash, "brep:solid-1".into()), "a fresh handle must be admitted as in-flight");
    assert_eq!(session.preview_mesh_pack("brep:solid-1"), None, "no mesh may exist before the extension answers");
    let emit = handle(&FlowTessellateResolve { window_id: "procedural-preview-test".into(), window_kind_id: crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into(), node_hash, output_json: complete_envelope(&pack) }, &doc, &cfg, &mut session).expect("flowTessellateResolve");
    assert!(emit.effects.is_empty(), "a completed tessellation terminates the round trip and emits no further effect");
    assert_eq!(session.preview_mesh_pack("brep:solid-1"), Some(pack.as_str()), "the resolved mesh body must be readable under its handle");
    assert!(!session.note_pending_tessellate(node_hash, "brep:solid-1".into()), "a resolved handle must never be re-requested");
    retire_flow_eval_session(session);
}

/// ⚖️ LAW: an unfinished budgeted step re-arms the tick chain instead of dropping the tessellation,
/// and reports monotone progress on the way.
#[test]
fn a_partial_step_re_arms_the_tick_chain_with_progress() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let snapshot = Generation3dSnapshot::default();
    let history = empty_history_view();
    let config = Generation3dConfig::default();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let mut session = FlowEvalSession::new();
    let node_hash = semio_framework_os_flow::preview_tessellate_node_hash("brep:solid-2", 0.01_f64.to_bits());
    assert!(session.note_pending_tessellate(node_hash, "brep:solid-2".into()));
    let working = r#"{"done":false,"cancellable":true,"phase":"meshingFaces","unitsDone":12,"unitsTotal":30,"facesDone":4,"facesTotal":6}"#;
    let emit = handle(&FlowTessellateResolve { window_id: "procedural-preview-test".into(), window_kind_id: crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into(), node_hash, output_json: working.into() }, &doc, &cfg, &mut session).expect("flowTessellateResolve");
    assert_eq!(emit.effects.len(), 1, "an unfinished tessellation must re-arm the tick chain");
    let status = session.preview_tessellate_status();
    assert_eq!((status.units_done, status.units_total), (12, 30));
    assert_eq!((status.faces_done, status.faces_total), (4, 6));
    assert_eq!(status.phase.tag(), "meshingFaces");
    assert!(session.preview_mesh_pack("brep:solid-2").is_none(), "no mesh may land before the job completes");
    retire_flow_eval_session(session);
}

/// ⚖️ LAW: a mesh body split across chunks only lands once its last chunk arrives; every earlier
/// chunk re-arms one more round trip and advances the chunk cursor.
#[test]
fn a_chunked_mesh_body_only_lands_on_its_last_chunk() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let snapshot = Generation3dSnapshot::default();
    let history = empty_history_view();
    let config = Generation3dConfig::default();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let mut session = FlowEvalSession::new();
    let pack = tessellated_triangle_pack();
    let split = pack.len() / 2;
    let node_hash = semio_framework_os_flow::preview_tessellate_node_hash("brep:solid-3", 0.01_f64.to_bits());
    assert!(session.note_pending_tessellate(node_hash, "brep:solid-3".into()));
    let first = format!(r#"{{"done":true,"phase":"complete","unitsDone":3,"unitsTotal":3,"facesDone":1,"facesTotal":1,"chunk":0,"chunks":2,"meshPack":"{}"}}"#, &pack[..split]);
    let emit = handle(&FlowTessellateResolve { window_id: "procedural-preview-test".into(), window_kind_id: crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into(), node_hash, output_json: first }, &doc, &cfg, &mut session).expect("first chunk");
    assert_eq!(emit.effects.len(), 1, "a partial mesh body must ask for the next chunk");
    assert_eq!(session.next_tessellate_chunk(node_hash), 1);
    assert!(session.preview_mesh_pack("brep:solid-3").is_none());
    assert!(session.note_pending_tessellate(node_hash, "brep:solid-3".into()), "the continuation re-admits the handle");
    let second = format!(r#"{{"done":true,"phase":"complete","unitsDone":3,"unitsTotal":3,"facesDone":1,"facesTotal":1,"chunk":1,"chunks":2,"meshPack":"{}"}}"#, &pack[split..]);
    let emit = handle(&FlowTessellateResolve { window_id: "procedural-preview-test".into(), window_kind_id: crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into(), node_hash, output_json: second }, &doc, &cfg, &mut session).expect("second chunk");
    assert!(emit.effects.is_empty(), "the last chunk completes the transfer");
    assert_eq!(session.preview_mesh_pack("brep:solid-3"), Some(pack.as_str()), "the reassembled body must equal the original");
    retire_flow_eval_session(session);
}

/// ⚖️ LAW: the per-tick liveness sweep (`retain_preview_meshes`, which every `flowEvalTick` runs
/// before re-arming) must NOT discard a tessellation that is only half transferred. Judging liveness
/// by the pending table — which a partial answer empties — restarted the transfer from chunk zero on
/// every tick, i.e. a multi-chunk mesh could never land.
#[test]
fn the_liveness_sweep_preserves_a_half_transferred_mesh_body() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let snapshot = Generation3dSnapshot::default();
    let history = empty_history_view();
    let config = Generation3dConfig::default();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let mut session = FlowEvalSession::new();
    let pack = tessellated_triangle_pack();
    let split = pack.len() / 2;
    let node_hash = semio_framework_os_flow::preview_tessellate_node_hash("brep:solid-5", 0.01_f64.to_bits());
    assert!(session.note_pending_tessellate(node_hash, "brep:solid-5".into()));
    let first = format!(r#"{{"done":true,"phase":"complete","unitsDone":3,"unitsTotal":3,"chunk":0,"chunks":2,"meshPack":"{}"}}"#, &pack[..split]);
    handle(&FlowTessellateResolve { window_id: "procedural-preview-test".into(), window_kind_id: crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into(), node_hash, output_json: first }, &doc, &cfg, &mut session).expect("first chunk");
    let live: std::collections::HashSet<String> = ["brep:solid-5".to_string()].into_iter().collect();
    session.retain_preview_meshes(&live);
    assert_eq!(session.next_tessellate_chunk(node_hash), 1, "the sweep must not rewind the chunk cursor");
    assert!(session.note_pending_tessellate(node_hash, "brep:solid-5".into()));
    let second = format!(r#"{{"done":true,"phase":"complete","unitsDone":3,"unitsTotal":3,"chunk":1,"chunks":2,"meshPack":"{}"}}"#, &pack[split..]);
    handle(&FlowTessellateResolve { window_id: "procedural-preview-test".into(), window_kind_id: crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into(), node_hash, output_json: second }, &doc, &cfg, &mut session).expect("second chunk");
    assert_eq!(session.preview_mesh_pack("brep:solid-5"), Some(pack.as_str()), "the body must survive the sweep intact");
    retire_flow_eval_session(session);
}

/// ⚖️ LAW: once the handle IS dead, the sweep does drop its half-received body — a superseded
/// tessellation must not leak its partial transfer.
#[test]
fn the_liveness_sweep_drops_a_dead_handles_partial_body() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let snapshot = Generation3dSnapshot::default();
    let history = empty_history_view();
    let config = Generation3dConfig::default();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let mut session = FlowEvalSession::new();
    let pack = tessellated_triangle_pack();
    let node_hash = semio_framework_os_flow::preview_tessellate_node_hash("brep:solid-6", 0.01_f64.to_bits());
    assert!(session.note_pending_tessellate(node_hash, "brep:solid-6".into()));
    let first = format!(r#"{{"done":true,"phase":"complete","unitsDone":3,"unitsTotal":3,"chunk":0,"chunks":2,"meshPack":"{}"}}"#, &pack[..pack.len() / 2]);
    handle(&FlowTessellateResolve { window_id: "procedural-preview-test".into(), window_kind_id: crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into(), node_hash, output_json: first }, &doc, &cfg, &mut session).expect("first chunk");
    session.retain_preview_meshes(&std::collections::HashSet::new());
    assert_eq!(session.next_tessellate_chunk(node_hash), 0, "a dead handle's transfer must be forgotten entirely");
    retire_flow_eval_session(session);
}

/// ⚖️ LAW: a blocking validate-gate response never becomes a mesh — it becomes a typed diagnostic on
/// the handle, and the handle is not re-requested.
#[test]
fn an_invalid_solid_becomes_a_typed_diagnostic_not_a_mesh() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let snapshot = Generation3dSnapshot::default();
    let history = empty_history_view();
    let config = Generation3dConfig::default();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let mut session = FlowEvalSession::new();
    let node_hash = semio_framework_os_flow::preview_tessellate_node_hash("brep:solid-4", 0.01_f64.to_bits());
    assert!(session.note_pending_tessellate(node_hash, "brep:solid-4".into()));
    let invalid = r#"{"done":true,"cancellable":false,"phase":"invalid","diagnostics":[{"entity":"shell-1","code":"shell-not-closed","message":"shell 1 is not closed"}]}"#;
    handle(&FlowTessellateResolve { window_id: "procedural-preview-test".into(), window_kind_id: crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into(), node_hash, output_json: invalid.into() }, &doc, &cfg, &mut session).expect("flowTessellateResolve");
    assert!(session.preview_mesh_pack("brep:solid-4").is_none(), "broken topology must never reach the renderer");
    let diagnostics = session.preview_diagnostics("brep:solid-4").expect("a typed diagnostic must be stored");
    assert!(diagnostics.contains("shell-not-closed"), "the diagnostic must carry the machine-readable code, got {diagnostics}");
    assert!(!session.note_pending_tessellate(node_hash, "brep:solid-4".into()), "a rejected handle must not be retried forever");
    retire_flow_eval_session(session);
}

/// ⚖️ LAW: a result for a `nodeHash` this session never requested changes nothing — the continuation is
/// idempotent against a stale or foreign completion.
#[test]
fn unknown_node_hash_resolves_nothing() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let snapshot = Generation3dSnapshot::default();
    let history = empty_history_view();
    let config = Generation3dConfig::default();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let mut session = FlowEvalSession::new();
    handle(&FlowTessellateResolve { window_id: "procedural-preview-test".into(), window_kind_id: crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into(), node_hash: 42, output_json: complete_envelope(&tessellated_triangle_pack()) }, &doc, &cfg, &mut session).expect("flowTessellateResolve");
    assert_eq!(session.preview_mesh_pack("brep:solid-1"), None);
    retire_flow_eval_session(session);
}
