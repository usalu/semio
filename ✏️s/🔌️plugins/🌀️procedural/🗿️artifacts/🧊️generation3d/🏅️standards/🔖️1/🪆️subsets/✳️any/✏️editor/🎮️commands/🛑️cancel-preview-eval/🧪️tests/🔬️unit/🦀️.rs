use super::*;
use crate::editor::generation3d::testkit::{empty_history_view, retire_flow_eval_session};
use semio_framework_plugin::{ArtifactView, ConfigView};

/// ⚖️ LAW: `cancelPreviewEval` retires every in-flight tessellation — the pending table empties, the
/// phase reads `cancelled`, the status stops advertising itself as cancellable, and a later
/// `flowTessellateResolve` for the retired hash is a no-op rather than a panic.
#[test]
fn cancel_retires_every_in_flight_tessellation_without_panicking() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let snapshot = Generation3dSnapshot::default();
    let history = empty_history_view();
    let config = Generation3dConfig::default();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let mut session = FlowEvalSession::new();
    let first = semio_framework_os_flow::preview_tessellate_node_hash("brep:solid-1", 0.05_f64.to_bits());
    let second = semio_framework_os_flow::preview_tessellate_node_hash("brep:solid-2", 0.05_f64.to_bits());
    assert!(session.note_pending_tessellate(first, "brep:solid-1".into()));
    assert!(session.note_pending_tessellate(second, "brep:solid-2".into()));
    assert_eq!(session.preview_tessellate_status().in_flight, 2);
    assert!(session.preview_tessellate_status().is_cancellable(), "two in-flight jobs must advertise a cancel affordance");
    let emit = handle(&CancelPreviewEval {}, &doc, &cfg, &mut session).expect("cancelPreviewEval");
    assert!(emit.effects.is_empty(), "a cancel emits no follow-up work");
    assert_eq!(session.preview_tessellate_status().in_flight, 0, "no request may stay in flight after a cancel");
    assert!(!session.preview_tessellate_status().is_cancellable(), "nothing is left to cancel");
    let stale = r#"{"done":true,"phase":"complete","unitsDone":3,"unitsTotal":3,"chunk":0,"chunks":1,"meshPack":""}"#;
    assert_eq!(session.resolve_preview_tessellate(first, stale), semio_framework_os_flow::PreviewTessellateOutcome::Unknown, "a response for a retired job must be ignored");
    retire_flow_eval_session(session);
}

/// ⚖️ LAW: cancelling twice is a no-op, never a fault — the gesture is idempotent.
#[test]
fn cancelling_twice_is_idempotent() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let snapshot = Generation3dSnapshot::default();
    let history = empty_history_view();
    let config = Generation3dConfig::default();
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg = ConfigView { snapshot: &config, window: None };
    let mut session = FlowEvalSession::new();
    handle(&CancelPreviewEval {}, &doc, &cfg, &mut session).expect("first cancel");
    handle(&CancelPreviewEval {}, &doc, &cfg, &mut session).expect("second cancel");
    assert_eq!(session.preview_tessellate_status().in_flight, 0);
    retire_flow_eval_session(session);
}
