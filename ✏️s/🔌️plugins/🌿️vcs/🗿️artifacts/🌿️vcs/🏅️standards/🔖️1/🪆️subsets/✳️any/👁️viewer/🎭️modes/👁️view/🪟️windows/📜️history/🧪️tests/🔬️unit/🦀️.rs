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
    let document = crate::VcsSnapshot { title: "Demo".into(), ..crate::VcsSnapshot::default() };
    let view = history_tree_view(&document, &history);
    assert_eq!(view.roots.len(), 1, "the document is the one root row");
    assert_eq!((view.roots[0].id.as_str(), view.roots[0].label.as_str()), (VCS_DOCUMENT_NODE_ID, "Demo"));
    let checkpoints = &view.roots[0].children;
    assert_eq!(checkpoints.len(), 1, "exactly one root checkpoint under the document");
    assert_eq!(checkpoints[0].id, "c1");
    assert_eq!(checkpoints[0].children.len(), 1, "c2 nests under its parent c1");
    assert_eq!(checkpoints[0].children[0].id, "c2");
}

/// ⚖️ LAW: a document with no checkpoints still paints its own row — every freshly opened viewer used
/// to paint an EMPTY tree (0 text, S15 session 11), because the kit renders no rows for an empty
/// roster. An untitled document reads as the language-neutral `—`.
#[semio_framework_async_macros::async_test]
async fn a_document_without_checkpoints_still_paints_its_own_row() {
    let genesis = crate::VcsSnapshot::default();
    let view = history_tree_view(&genesis, &HistoryView::empty());
    assert_eq!(view.roots.len(), 1);
    assert_eq!((view.roots[0].id.as_str(), view.roots[0].label.as_str()), (VCS_DOCUMENT_NODE_ID, genesis.title.as_str()));
    assert!(view.roots[0].children.is_empty());
    let untitled = crate::VcsSnapshot { title: String::new(), ..genesis.clone() };
    assert_eq!(history_tree_view(&untitled, &HistoryView::empty()).roots[0].label, VCS_UNTITLED_LABEL);
    let projection = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(render(&genesis, &HistoryView::empty()).expect("render"))).expect("projection");
    assert!(projection.contains(&genesis.title), "{projection}");
}
