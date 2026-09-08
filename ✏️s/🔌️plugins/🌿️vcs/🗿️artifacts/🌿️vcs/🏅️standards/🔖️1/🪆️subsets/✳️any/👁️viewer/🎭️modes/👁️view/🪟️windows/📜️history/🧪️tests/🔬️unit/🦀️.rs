
use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_declares_a_tree_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
}

#[semio_framework_async_macros::async_test]
async fn render_nests_checkpoints_under_their_parent() {
    let history = HistoryView {
        columns: vec![
            store::HistoryColumn { checkpoint_id: "c1".into(), timestamp: "t1".into(), labels: Vec::new(), authors: Vec::new(), parent_checkpoint_id: None, description: Some("root".into()), lane: 0, alternative_ids: Vec::new() },
            store::HistoryColumn { checkpoint_id: "c2".into(), timestamp: "t2".into(), labels: Vec::new(), authors: Vec::new(), parent_checkpoint_id: Some("c1".into()), description: Some("child".into()), lane: 0, alternative_ids: Vec::new() },
        ],
        ..HistoryView::empty()
    };
    let view = history_tree_view(&history);
    assert_eq!(view.roots.len(), 1, "exactly one root checkpoint");
    assert_eq!(view.roots[0].id, "c1");
    assert_eq!(view.roots[0].children.len(), 1, "c2 nests under its parent c1");
    assert_eq!(view.roots[0].children[0].id, "c2");
}
