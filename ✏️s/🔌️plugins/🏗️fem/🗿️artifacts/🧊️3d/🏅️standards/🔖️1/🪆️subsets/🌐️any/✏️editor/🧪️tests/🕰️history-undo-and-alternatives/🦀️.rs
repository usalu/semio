//! 🕰️ History laws of the `s.fem.fem3d` play app: undo and redo never wedge the app, and the History panel actions Create Alternative and Commit Checkpoint work on a document that has no explicit checkpoint yet.

use crate::context::{close, dispatch, fem3d_demo_app, history_verb, Fem3dApp};
use semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dCommand;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_node;

async fn add(app: &mut Fem3dApp, x: f64) {
    dispatch(app, Fem3dCommand::AddNode(add_node::AddNode { x, y: 0.0, z: 0.0 })).await;
}

fn nodes(app: &Fem3dApp) -> usize {
    app.snapshot().expect("snapshot").nodes.iter().count()
}

/// ↩️ LAW: an undo reverts the last edit, a redo restores it, and the app keeps accepting edits and further history moves afterwards.
#[semio_framework_async_macros::async_test]
async fn undo_and_redo_round_trip_then_the_app_keeps_editing_3d() {
    let mut app = fem3d_demo_app().await;
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
async fn create_alternative_without_an_explicit_checkpoint_branches_from_the_head_3d() {
    let mut app = fem3d_demo_app().await;
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
async fn create_alternative_with_pending_edits_checkpoints_them_3d() {
    let mut app = fem3d_demo_app().await;
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
async fn commit_checkpoint_works_with_and_without_pending_edits_3d() {
    let mut app = fem3d_demo_app().await;
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

/// 🎟️ LAW: a long session of gumball gestures (streamed, committed and aborted), gumball flags, playback ticks, edits, undos, redos, renders and example switches never fills the document, config, draft or interaction Store's snapshot read issuer: the reads they hold stay far below the pump pressure mark and a fresh read is still admitted at the end.
#[semio_framework_async_macros::async_test]
async fn a_long_mixed_session_never_exhausts_the_snapshot_read_issuers_3d() {
    use crate::context::{edit, render};
    use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::set_active_example;
    use semio_s_artifact_fem_3d_editor::editor::fem3d::commands::{gumball::{set_transform_gumball_flag, translate_selection}, result_animation_tick::ResultAnimationTick, set_result_animation::SetResultAnimation};
    let mut app = fem3d_demo_app().await;
    let held = |app: &Fem3dApp| [
        app.stores.store.returned_snapshot_read_count() + app.stores.store.outstanding_snapshot_read_count(),
        app.stores.config_store.returned_snapshot_read_count() + app.stores.config_store.outstanding_snapshot_read_count(),
        app.stores.draft_store.returned_snapshot_read_count() + app.stores.draft_store.outstanding_snapshot_read_count(),
        app.stores.interaction_store.returned_snapshot_read_count() + app.stores.interaction_store.outstanding_snapshot_read_count(),
    ];
    let node = || semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::fem3d_entity_target("node", "n20_l1").unwrap();
    let gumball = |dx: f64, phase: &str| Fem3dCommand::TranslateSelection(translate_selection::TranslateSelection { ids: vec![node()], dx, dy: 0.0, dz: 0.0, phase: Some(phase.to_string()), reason: None });
    let mut peak = 0usize;
    for round in 0..10usize {
        for step in 0..4 { dispatch(&mut app, gumball(0.01 * step as f64, "stream")).await; }
        dispatch(&mut app, gumball(0.2, if round % 2 == 0 { "commit" } else { "abort" })).await;
        dispatch(&mut app, Fem3dCommand::SetTransformGumballFlag(set_transform_gumball_flag::SetTransformGumballFlag { flag: "rotate".into(), pressed: Some(round % 2 == 0) })).await;
        dispatch(&mut app, Fem3dCommand::SetResultAnimation(SetResultAnimation { phase: None, playing: Some(round % 2 == 0), speed: None, loop_mode: None, waveform: None, field: None, value: None, window_id: None })).await;
        dispatch(&mut app, Fem3dCommand::ResultAnimationTick(ResultAnimationTick { window_id: "results-left".into() })).await;
        add(&mut app, round as f64).await;
        render(&mut app, edit::windows::model::FEM3D_BODY_MODEL);
        render(&mut app, edit::windows::results::FEM3D_BODY_RESULTS);
        history_verb(&mut app, "undo").await;
        if round % 3 == 0 { history_verb(&mut app, "redo").await; }
        if round % 5 == 4 {
            let example = if round % 10 == 4 { "concrete-forest" } else { semio_s_artifact_fem_3d_io::examples::demo::ID };
            dispatch(&mut app, Fem3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: example.into() })).await;
        }
        peak = peak.max(held(&app).into_iter().max().unwrap_or(0));
    }
    assert!(peak < store::os_store::snapshot_read::SNAPSHOT_READ_RETURN_PRESSURE, "held snapshot reads reached {peak}");
    assert!(app.stores.store.snapshot_read().is_ok(), "the issuer still admits a fresh read after the long session");
    close(&mut app);
}
