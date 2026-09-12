use super::*;
use crate::editor::generation3d::testkit::{empty_history_view, retire_flow_eval_session};
use semio_framework_plugin::{ArtifactView, ConfigView};

/// ⚖️ LAW: folding the geometry extension's cancel acknowledgement arms NOTHING. The gesture already
/// left every latch quiescent; an acknowledgement that re-armed would restart the chain the user
/// stopped, which is exactly the defect `flowTessellateResolve` would have introduced had the cancel
/// reused it as its response action.
#[test]
fn the_cancel_acknowledgement_arms_nothing() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let snapshot = Generation3dSnapshot::default();
    let history = empty_history_view();
    let config = Generation3dConfig::default();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let mut session = FlowEvalSession::new();
    let hash = semio_framework_os_flow::preview_tessellate_node_hash("brep:solid-1", 0.05_f64.to_bits());
    assert!(session.note_pending_tessellate(hash, "brep:solid-1".into()));
    session.note_window_extensions_in_flight("procedural-preview-test", 1);
    session.cancel_preview_evaluation("procedural-preview-test");
    let payload = FlowTessellateCancelResolve { window_id: "procedural-preview-test".into(), window_kind_id: crate::editor::generation3d::modes::edit::windows::preview::GENERATION_3D_PLAY_WINDOW_PREVIEW.into(), output_json: r#"{"ok":true,"retired":1}"#.into(), ok: true };
    let emit = handle(&payload, &doc, &cfg, &mut session).expect("flowTessellateCancelResolve");
    assert!(emit.effects.is_empty(), "the acknowledgement emits no self-redispatch");
    assert!(emit.extension_invocations.is_empty(), "the acknowledgement emits no further extension work");
    assert!(!session.window_tick_is_armed("procedural-preview-test"));
    assert!(!session.window_tick_owed("procedural-preview-test"));
    assert!(session.preview_cancelled(), "the cancelled banner survives its own acknowledgement");
    retire_flow_eval_session(session);
}
