use super::*;
use crate::editor::generation3d::testkit::{empty_history_view, retire_flow_eval_session};
use semio_framework_plugin::{ArtifactView, ConfigView};

/// 🔺️ One triangle — `preview_mesh_json_has_geometry` admits `indices > 0 && positions >= 9`.
const TESSELLATED_TRIANGLE_JSON: &str = r#"{"positions":[0.0,0.0,0.0,1.0,0.0,0.0,0.0,1.0,0.0],"indices":[0,1,2],"normals":[],"edgePositions":[],"faceIds":[]}"#;

/// ⚖️ LAW: a `tessellate` extension result carried back as `flowTessellateResolve` lands on exactly the
/// geometry handle `preview_tessellate_effects` recorded for that `nodeHash`, clearing the in-flight
/// entry — the half of the brep preview round trip that lives inside the app.
#[test]
fn tessellate_result_resolves_the_pending_handle() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let snapshot = Generation3dSnapshot::default();
    let history = empty_history_view();
    let config = Generation3dConfig::default();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let mut session = FlowEvalSession::new();
    let node_hash = semio_framework_os_flow::preview_tessellate_node_hash("brep:solid-1", 0.01_f64.to_bits());
    assert!(session.note_pending_tessellate(node_hash, "brep:solid-1".into()), "a fresh handle must be admitted as in-flight");
    assert_eq!(session.preview_mesh_json("brep:solid-1"), None, "no mesh may exist before the extension answers");
    let emit = handle(&FlowTessellateResolve { node_hash, output_json: TESSELLATED_TRIANGLE_JSON.into() }, &doc, &cfg, &mut session).expect("flowTessellateResolve");
    assert!(emit.effects.is_empty(), "a tessellate resolution terminates the round trip and emits no further effect");
    assert_eq!(session.preview_mesh_json("brep:solid-1"), Some(TESSELLATED_TRIANGLE_JSON), "the resolved mesh must be readable under its handle");
    assert!(!session.note_pending_tessellate(node_hash, "brep:solid-1".into()), "a resolved handle must never be re-requested");
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
    handle(&FlowTessellateResolve { node_hash: 42, output_json: TESSELLATED_TRIANGLE_JSON.into() }, &doc, &cfg, &mut session).expect("flowTessellateResolve");
    assert_eq!(session.preview_mesh_json("brep:solid-1"), None);
    retire_flow_eval_session(session);
}
