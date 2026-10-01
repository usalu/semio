//! 🧲️ Laws of the lowpoly gumball tool: one gumball gesture is ONE `ToolTransaction` of relative selection leaves —
//! one edit, one history row stamped with its `TransactionRef`, labelled from the leaf; a gesture that moves nothing
//! leaves zero trace; two gestures are two transactions; and a gesture edited in history replays its downstream exactly
//! like a fresh fold of the edited log.

use super::*;
use crate::editor::lowpoly::unit_tests::context::{act, app, committed_edits, dispatch};
use crate::editor::lowpoly::LowpolyCommand;
use crate::{LowpolyMutation, LOWPOLY_DOCUMENT_SCHEMA};

/// 🧾️ The applied document rows of the session history, newest last.
async fn edit_rows(app: &mut crate::editor::lowpoly::unit_tests::context::LowpolyApp) -> Vec<semio_framework::kernel::HistoryEntry> {
    app.0.history_snapshot().await.expect("history snapshot").upserts.into_iter().filter(|entry| entry.kind == "mutation" && entry.applied).collect()
}

/// 🧲️ One gumball translate is one edit and one history row stamped with the gumball's transaction, labelled from its
/// `move-selection` leaf in English and German; one undo reverts it.
#[semio_framework_async_macros::async_test]
async fn one_gumball_translate_is_one_edit_one_row_and_one_transaction() {
    let mut a = app().await;
    let before = a.snapshot().expect("projection").objects[0].clone();
    let edits_before = committed_edits(&mut a).await;
    dispatch(&mut a, LowpolyCommand::TranslateSelection(translate_selection::TranslateSelection { dx: 0.5, dy: 0.0, dz: 0.0 })).await;
    assert_eq!(committed_edits(&mut a).await, edits_before + 1, "one gesture is one document edit");
    assert_ne!(a.snapshot().expect("projection").objects[0].mesh, before.mesh, "the gesture moved the mesh");
    let rows = edit_rows(&mut a).await;
    let row = rows.last().expect("the gesture's row");
    let transaction = row.transaction.as_ref().expect("the row is keyed by its tool transaction");
    assert!(transaction.id.starts_with("tx-") && transaction.tool == "s.lowpoly.lowpoly@1/*#editor#translateSelection", "the gumball tool authored it: {transaction:?}");
    assert_eq!(row.label.resolve(protocol::Terminology::Native, protocol::Locale::En).to_string(), format!("Move \"{}\" by (0.5, 0, 0)", before.id));
    assert_eq!(row.label.resolve(protocol::Terminology::Native, protocol::Locale::De).to_string(), format!("\"{}\" um (0,5; 0; 0) verschieben", before.id));
    act(&mut a, "undo", serde_json::json!({})).await;
    assert_eq!(a.snapshot().expect("projection").objects[0].mesh, before.mesh, "one undo reverts the whole gesture");
}

/// ⏸️ A gesture that moves nothing leaves zero trace; two gestures are two transactions.
#[semio_framework_async_macros::async_test]
async fn a_still_gesture_leaves_zero_trace_and_two_gestures_are_two_transactions() {
    let mut a = app().await;
    let edits_before = committed_edits(&mut a).await;
    dispatch(&mut a, LowpolyCommand::ScaleSelection(scale_selection::ScaleSelection { sx: 1.0, sy: 1.0, sz: 1.0 })).await;
    assert_eq!(committed_edits(&mut a).await, edits_before, "unit factors commit nothing");
    dispatch(&mut a, LowpolyCommand::RotateSelection(rotate_selection::RotateSelection { ax: 0.0, ay: 1.0, az: 0.0, angle: 0.5 })).await;
    dispatch(&mut a, LowpolyCommand::ScaleSelection(scale_selection::ScaleSelection { sx: 2.0, sy: 1.0, sz: 1.0 })).await;
    assert_eq!(committed_edits(&mut a).await, edits_before + 2, "two gestures are two edits");
    let rows = edit_rows(&mut a).await;
    let ids: Vec<String> = rows.iter().rev().take(2).map(|row| row.transaction.as_ref().expect("stamped").id.clone()).collect();
    assert_ne!(ids[0], ids[1], "two gestures are two transactions");
}

/// ⏪️ A gumball move edited in history replays its downstream: the preview base is the state right before the move,
/// and the Report replay re-applies the downstream relative scale and vertex move onto the edited move — exactly the
/// fresh fold of the edited log; overwrite commits it.
#[semio_framework_async_macros::async_test]
async fn a_gumball_move_edited_in_history_replays_its_downstream() {
    use protocol::OpBinary;
    let base = crate::schema::default_snapshot();
    let object_id = base.objects[0].id.clone();
    let mut store = store::ArtifactStore::<LowpolySnapshot, LowpolyMutation>::new(store::create_document_envelope::<LowpolySnapshot, LowpolyMutation>(LOWPOLY_DOCUMENT_SCHEMA, "gumball-time-travel", base.clone(), None)).await.expect("the store opens");
    store.install_document_store_owners_exact(semio_framework_plugin::bounded_document_store_owners::<LowpolySnapshot, LowpolyMutation>());
    let moved = |offset: [f32; 3]| LowpolyMutation::MoveSelection(MoveSelection { object_id: object_id.clone(), vertex_ids: Vec::new(), offset });
    let log = [moved([1.0, 0.0, 0.0]), LowpolyMutation::ScaleSelection(ScaleSelection { object_id: object_id.clone(), vertex_ids: Vec::new(), pivot: [0.0; 3], factor: [2.0, 1.0, 1.0] }), LowpolyMutation::MoveSelection(MoveSelection { object_id: object_id.clone(), vertex_ids: vec![0], offset: [0.0, 0.5, 0.0] })];
    for mutation in &log {
        store.dispatch(store::ArtifactCommand::Apply { mutations: vec![mutation.clone()], description: None, transaction: None }).await.expect("the edit applies");
    }
    let ids: Vec<protocol::MutationId> = store.mutation_ops().expect("applied operations").into_iter().map(|operation| operation.mutation_id).collect();
    let edited = moved([0.0, 0.0, -2.0]);
    let drafts: std::collections::BTreeMap<protocol::MutationId, protocol::InputReplacement> = [(ids[0].clone(), protocol::InputReplacement::Input { schema: LOWPOLY_DOCUMENT_SCHEMA.into(), payload: edited.encode_op().expect("the edited leaf encodes") })].into_iter().collect();
    let preview = store.state_before(&ids[0], &drafts).expect("the preview base folds").as_ref().clone();
    assert_eq!(preview, base, "the preview base is the state right before the edited move");
    let mut replay = store.begin_report_replay(&drafts, Some(&ids[0])).expect("the replay begins at the edited move");
    assert!(matches!(replay.step(store.replay_edits(), &mut || false).expect("the replay steps"), store::ReplayStep::Finished(_)));
    let result = replay.finish().expect("a finished replay yields its result");
    assert!(!store.replay_report(&result).expect("report").blocks_finalize(), "a re-offset move never blocks finalizing");
    let fresh = [edited, log[1].clone(), log[2].clone()].iter().fold(base.clone(), |state, mutation| protocol::apply_mutation(&state, mutation).expect("the edited log folds").0);
    assert_ne!(fresh, protocol::apply_mutation(&base, &log[0]).map(|(state, _)| state).expect("the original move applies"), "the edit changes the outcome");
    assert_eq!(result.state().expect("the replay reached a state").as_ref(), &fresh, "the replay equals the fresh fold of the edited log");
    store.commit_finished_replay(result, store::HistoryFinalization::Overwrite).await.expect("overwrite commits");
    assert_eq!(store.snapshot_ref(), &fresh, "the overwritten history folds to the edited state");
    let mut disposer = semio_framework_plugin::bounded_document_store_disposer::<LowpolySnapshot, LowpolyMutation>();
    for _ in 0..4_096 {
        if disposer.terminal_is_empty(&store) {
            break;
        }
        disposer.close_step(&mut store, 1, 1 << 20).expect("the store retires");
    }
    assert!(disposer.terminal_is_empty(&store), "the standalone store retires to its terminal-empty shell");
}
