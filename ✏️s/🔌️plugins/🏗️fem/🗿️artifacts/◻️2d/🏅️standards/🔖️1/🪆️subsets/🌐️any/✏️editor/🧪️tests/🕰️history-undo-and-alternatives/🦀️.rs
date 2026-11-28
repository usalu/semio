//! 🕰️ History laws of the `s.fem.fem2d` play app: undo and redo never wedge the app, and the History panel actions Create Alternative and Commit Checkpoint work on a document that has no explicit checkpoint yet.

use crate::context::{close, dispatch, fem2d_mounted_app, history_verb, Fem2dApp};
use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dCommand;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_node;

async fn add(app: &mut Fem2dApp, x: f64) {
    dispatch(app, Fem2dCommand::AddNode(add_node::AddNode { x, y: 0.0 })).await;
}

fn nodes(app: &Fem2dApp) -> usize {
    app.snapshot().expect("snapshot").nodes.len()
}

/// ↩️ LAW: an undo reverts the last edit, a redo restores it, and the app keeps accepting edits and further history moves afterwards.
#[semio_framework_async_macros::async_test]
async fn undo_and_redo_round_trip_then_the_app_keeps_editing_2d() {
    let mut app = fem2d_mounted_app();
    let base = nodes(&app);
    add(&mut app, 1.0).await;
    add(&mut app, 2.0).await;
    assert_eq!(nodes(&app), base + 2);
    history_verb(&mut app, "undo").await;
    assert_eq!(nodes(&app), base + 1, "undo reverts the last edit");
    history_verb(&mut app, "redo").await;
    assert_eq!(nodes(&app), base + 2, "redo restores it");
    history_verb(&mut app, "undo").await;
    history_verb(&mut app, "undo").await;
    assert_eq!(nodes(&app), base, "two undos revert both edits");
    add(&mut app, 3.0).await;
    assert_eq!(nodes(&app), base + 1, "a new edit after undo lands");
    history_verb(&mut app, "undo").await;
    assert_eq!(nodes(&app), base, "and is undoable again");
    close(&mut app);
}

/// 🌿️ LAW: Create Alternative from a document with no checkpoint branches from its current head, and the branch keeps its own undoable edits.
#[semio_framework_async_macros::async_test]
async fn create_alternative_without_an_explicit_checkpoint_branches_from_the_head_2d() {
    let mut app = fem2d_mounted_app();
    let base = nodes(&app);
    history_verb(&mut app, "createAlternative").await;
    add(&mut app, 1.0).await;
    assert_eq!(nodes(&app), base + 1);
    history_verb(&mut app, "undo").await;
    assert_eq!(nodes(&app), base, "the alternative's edits are undoable");
    close(&mut app);
}

/// 🌿️ LAW: Create Alternative with edits but no checkpoint commits them into a checkpoint first and keeps them.
#[semio_framework_async_macros::async_test]
async fn create_alternative_with_pending_edits_checkpoints_them_2d() {
    let mut app = fem2d_mounted_app();
    let base = nodes(&app);
    add(&mut app, 1.0).await;
    history_verb(&mut app, "createAlternative").await;
    assert_eq!(nodes(&app), base + 1, "the pending edit survives the branch point");
    add(&mut app, 2.0).await;
    assert_eq!(nodes(&app), base + 2);
    history_verb(&mut app, "undo").await;
    assert_eq!(nodes(&app), base + 1);
    close(&mut app);
}

/// 🚩️ LAW: Commit Checkpoint works with pending edits and is a no-op without any, and the app stays live and undoable after both.
#[semio_framework_async_macros::async_test]
async fn commit_checkpoint_works_with_and_without_pending_edits_2d() {
    let mut app = fem2d_mounted_app();
    let base = nodes(&app);
    history_verb(&mut app, "commitCheckpoint").await;
    add(&mut app, 1.0).await;
    history_verb(&mut app, "commitCheckpoint").await;
    history_verb(&mut app, "commitCheckpoint").await;
    add(&mut app, 2.0).await;
    assert_eq!(nodes(&app), base + 2);
    history_verb(&mut app, "undo").await;
    assert_eq!(nodes(&app), base + 1);
    close(&mut app);
}

/// 🗑️ LAW: undo of a cascading delete (a node with every element that references it) restores the document exactly, and redo deletes the same entities again, across two undo and redo rounds.
#[semio_framework_async_macros::async_test]
async fn undo_and_redo_of_a_cascading_delete_restore_and_remove_the_same_entities_2d() {
    use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::remove_selection::RemoveSelection;
    use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::fem2d_entity_target_id;
    use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemElement;
    let mut app = fem2d_mounted_app();
    let before = app.snapshot().expect("snapshot");
    let node = before.nodes.iter().find(|node| before.elements.iter().any(|element| matches!(element, FemElement::Bar { start, .. } | FemElement::Beam { start, .. } if *start == *node.id))).expect("a node with elements").id.to_string();
    let elements: Vec<String> = before.elements.iter().filter_map(|element| match element { FemElement::Bar { id, start, end, .. } | FemElement::Beam { id, start, end, .. } if *start == node || *end == node => Some(id.clone()), _ => None }).collect();
    let mut ids: Vec<String> = elements.iter().map(|id| fem2d_entity_target_id("element", id).expect("element target")).collect();
    ids.push(fem2d_entity_target_id("node", &node).expect("node target"));
    dispatch(&mut app, Fem2dCommand::RemoveSelection(RemoveSelection { ids })).await;
    let deleted = app.snapshot().expect("snapshot");
    assert!(deleted.nodes.len() < before.nodes.len(), "the delete removed the node");
    for round in 0..2 {
        history_verb(&mut app, "undo").await;
        assert_eq!(app.snapshot().expect("snapshot"), before, "undo restores the document exactly (round {round})");
        history_verb(&mut app, "redo").await;
        assert_eq!(app.snapshot().expect("snapshot"), deleted, "redo deletes the same entities again (round {round})");
    }
    close(&mut app);
}
