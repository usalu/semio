use super::*;
use crate::editor_domain::editor_laws::context::{empty_history_view, retire_flow_eval_session};

fn preview_eval_run_fixture_json() -> &'static str { crate::runtime_fixture::text("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/⏯️preview-eval-run.json") }

/// ⚖️ LAW: the editor's release hop emits exactly the fixture's two kernel release invocations when the
/// geometry extension is contributed, and nothing when it is not — the same answer the shared
/// `🧵️preview-eval` law gives, reached through the command row the run's close dispatches.
#[test]
fn the_release_hop_emits_the_declared_kernel_release_through_the_command_row() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let fixture: serde_json::Value = serde_json::from_str(preview_eval_run_fixture_json()).expect("fixture");
    let snapshot = semio_s_artifact_procedural_3d_model::default_snapshot();
    let config = Generation3dConfig::default();
    let history = empty_history_view();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let mut session = FlowEvalSession::new();
    let payload = FlowEvalRelease { window_id: "procedural-preview-test".into(), window_kind_id: "procedural-preview".into() };
    let emit = handle(&payload, &doc, &cfg, &mut session).expect("the release hop never faults");
    let expected: Vec<&str> = fixture["release"]["capabilities"].as_array().unwrap().iter().filter_map(serde_json::Value::as_str).collect();
    let capabilities: Vec<&str> = emit.extension_invocations.iter().map(|invocation| invocation.capability.as_str()).collect();
    println!("[STATS] release hop capabilities={capabilities:?} addressable={}", semio_s_artifact_procedural_3d_preview_eval::geometry_extension_address().is_ok());
    if semio_s_artifact_procedural_3d_preview_eval::geometry_extension_address().is_ok() {
        assert_eq!(capabilities, expected);
    } else {
        assert!(capabilities.is_empty(), "an unaddressable geometry extension has no actor to tell");
    }
    assert!(emit.artifact_mutations.is_empty() && emit.effects.is_empty(), "the release hop publishes nothing and arms nothing");
    snapshot.retire_cold();
    retire_flow_eval_session(session);
}
