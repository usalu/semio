use super::*;
use semio_framework_plugin::WindowTransientOwner;

/// ⚖️ LAW: the owner is keyed by THIS window kind — a tick addressed at any other kind can never
/// capture this authority, which is what makes the addressed chain refuse a wrong-window dispatch
/// instead of silently publishing somewhere else.
#[test]
fn the_owner_is_keyed_by_the_viewer_preview_window_kind() {
    assert_eq!(Generation3dViewPreviewWindowTransientOwner::WINDOW_KIND_ID, "procedural-view-preview");
}

/// ⚖️ LAW: one publication is one admissible item whose retained bytes really follow the
/// evaluation text — a fixed-size footprint would let an unbounded eval through the store gate.
#[test]
fn the_publication_footprint_follows_the_evaluation_text() {
    let empty = preflight(&Generation3dViewTransientMutation::SetPreviewEval(SetPreviewEval { eval_text: None })).expect("an empty publication is admissible");
    let body = "x".repeat(4_096);
    let filled = preflight(&Generation3dViewTransientMutation::SetPreviewEval(SetPreviewEval { eval_text: Some(body) })).expect("a real evaluation is admissible");
    assert_eq!(empty.work_items, 1);
    assert_eq!(filled.work_items, 1);
    assert!(filled.retained_bytes > empty.retained_bytes, "the evaluation text must be weighed: {} vs {}", filled.retained_bytes, empty.retained_bytes);
}

/// ⚖️ LAW: the transfer replaces the whole state, so a republished evaluation never merges with the
/// one it displaces.
#[test]
fn the_transfer_replaces_the_published_evaluation() {
    let next = transfer(Generation3dViewTransientMutation::SetPreviewEval(SetPreviewEval { eval_text: Some("{\"a\":1}".into()) }));
    assert_eq!(next.preview_eval_text.as_deref(), Some("{\"a\":1}"));
    assert_eq!(transfer(Generation3dViewTransientMutation::SetPreviewEval(SetPreviewEval { eval_text: None })).preview_eval_text, None);
}
