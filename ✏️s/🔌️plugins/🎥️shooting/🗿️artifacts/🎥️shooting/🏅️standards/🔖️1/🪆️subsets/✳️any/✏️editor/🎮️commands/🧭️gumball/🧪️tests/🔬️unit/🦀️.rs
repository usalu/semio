//! 🛠️ Laws of the gumball tool's one promise: one gumball gesture is one `ToolTransaction` — one edit, one history row
//! keyed by its `TransactionRef`, one editable relative leaf labelled from the leaf — a gesture that moves nothing
//! leaves zero trace, and the leaf's inputs replay relative to whatever base precedes it.

use super::*;
use crate::editor::shooting::unit_tests::context::{dispatch, dispatch_rows, history_verb, shooting_app, ShootingApp, SHOOTING_TEST_INSTANCE};
use crate::editor::shooting::{ShootingCommand, SHOOTING_INTERACTION_DOMAIN};
use protocol::{Mutation, MutationDiff};
use semio_framework::kernel::HistoryEntry;
use semio_framework_plugin::{artifact_app_laws, PluginApp};

fn translate(asset_ids: Vec<String>, dx: f64) -> ShootingCommand {
    ShootingCommand::TranslateSelection(translate_selection::TranslateSelection { asset_ids, dx, dy: 0.0, dz: 0.0 })
}

fn english(row: &HistoryEntry) -> String {
    row.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En).to_string()
}

fn german(row: &HistoryEntry) -> String {
    row.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De).to_string()
}

fn origin(app: &ShootingApp, asset_id: &str) -> [f64; 3] {
    app.snapshot().expect("snapshot").assets.iter().find(|asset| asset.id == asset_id).expect("asset").origin
}

/// 🛠️ LAW: one gumball drag is one edit and one history row keyed by the tool's transaction, whose one op is the
/// editable relative `drag-assets` leaf, labelled from the leaf in English and German.
#[semio_framework_async_macros::async_test]
async fn one_gumball_drag_is_one_edit_one_row_and_one_transaction() {
    let mut app = shooting_app().await;
    let asset_id = app.snapshot().expect("snapshot").assets[0].id.clone();
    let before = origin(&app, &asset_id);
    let rows = dispatch_rows(&mut app, translate(vec![asset_id.clone()], 2.5)).await;
    assert_eq!(rows.len(), 1, "one drag, one row: {rows:?}");
    let transaction = rows[0].transaction.as_ref().expect("the row is keyed by its tool transaction");
    assert!(transaction.id.starts_with("tx-"), "a minted transaction id: {transaction:?}");
    assert_eq!(transaction.tool, format!("{SHOOTING_PLAY_APP_ID}#translateSelection"));
    assert_eq!((rows[0].op_count, rows[0].mutations.len()), (1, 1), "one leaf, one mutation row");
    assert!(rows[0].mutations[0].editable, "the yielded leaf is history-editable");
    assert_eq!((english(&rows[0]), german(&rows[0])), ("Drag 1 asset".to_string(), "1 Asset ziehen".to_string()), "the row reads the leaf's label");
    assert_eq!(origin(&app, &asset_id), [before[0] + 2.5, before[1], before[2]], "the leaf moved the asset relative to its base");
}

/// 🔁️ LAW: two drags are two transactions — two rows with distinct refs — and one undo reverts only the second.
#[semio_framework_async_macros::async_test]
async fn two_drags_are_two_transactions() {
    let mut app = shooting_app().await;
    let asset_id = app.snapshot().expect("snapshot").assets[0].id.clone();
    let before = origin(&app, &asset_id);
    let first = dispatch_rows(&mut app, translate(vec![asset_id.clone()], 1.0)).await;
    let second = dispatch_rows(&mut app, translate(vec![asset_id.clone()], 2.0)).await;
    let (first, second) = (first[0].transaction.clone().expect("first ref"), second[0].transaction.clone().expect("second ref"));
    assert_ne!(first.id, second.id, "consecutive drags never share a transaction");
    history_verb(&mut app, "undo").await;
    assert_eq!(origin(&app, &asset_id), [before[0] + 1.0, before[1], before[2]], "undo reverts exactly the second drag");
}

/// 🔄️ LAW: a rotate and a scale gesture each commit ONE transaction holding their own relative leaf.
#[semio_framework_async_macros::async_test]
async fn rotate_and_scale_gestures_are_one_transaction_each() {
    let mut app = shooting_app().await;
    let asset_id = app.snapshot().expect("snapshot").assets[0].id.clone();
    let rotated = dispatch_rows(&mut app, ShootingCommand::RotateSelection(rotate_selection::RotateSelection { asset_ids: vec![asset_id.clone()], ax: 0.0, ay: 0.0, az: 1.0, angle: 0.5 })).await;
    let scaled = dispatch_rows(&mut app, ShootingCommand::ScaleSelection(scale_selection::ScaleSelection { asset_ids: vec![asset_id], sx: 2.0, sy: 2.0, sz: 2.0 })).await;
    assert_eq!((rotated.len(), scaled.len()), (1, 1));
    assert!(rotated[0].transaction.as_ref().is_some_and(|transaction| transaction.tool.ends_with("#rotateSelection")));
    assert!(scaled[0].transaction.as_ref().is_some_and(|transaction| transaction.tool.ends_with("#scaleSelection")));
    assert_eq!((english(&rotated[0]), english(&scaled[0])), ("Rotate 1 asset".to_string(), "Scale 1 asset".to_string()));
}

/// 🫥️ LAW: a gesture that moves nothing — an identity pose, a missing target, an empty selection — leaves zero trace.
#[semio_framework_async_macros::async_test]
async fn a_gesture_that_moves_nothing_leaves_zero_trace() {
    let mut app = shooting_app().await;
    let asset_id = app.snapshot().expect("snapshot").assets[0].id.clone();
    let before = app.snapshot().expect("snapshot");
    for command in [
        translate(vec![asset_id.clone()], 0.0),
        translate(vec!["ghost".into()], 1.0),
        translate(Vec::new(), 1.0),
        ShootingCommand::RotateSelection(rotate_selection::RotateSelection { asset_ids: vec![asset_id.clone()], ax: 0.0, ay: 0.0, az: 0.0, angle: 1.0 }),
        ShootingCommand::ScaleSelection(scale_selection::ScaleSelection { asset_ids: vec![asset_id.clone()], sx: 0.0, sy: 1.0, sz: 1.0 }),
    ] {
        assert!(!dispatch(&mut app, command).await.edited_document(), "no edit");
    }
    assert_eq!(app.snapshot().expect("snapshot"), before, "the document is untouched");
}

/// 🎯️ LAW: an id-less gesture on the retained route moves the live `assets` selection — the host's gumball sends the
/// grabbed ids, a palette or agent dispatch may send none.
#[semio_framework_async_macros::async_test]
async fn an_id_less_gesture_moves_the_live_selection() {
    let mut app = shooting_app().await;
    let asset_id = app.snapshot().expect("snapshot").assets[0].id.clone();
    let before = origin(&app, &asset_id);
    let targets = serde_json::to_string(&serde_json::json!([{ "granularity": "asset", "id": asset_id }])).expect("targets");
    app.handle_action("interactionSelect", Some(&semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::json!({ "domainId": SHOOTING_INTERACTION_DOMAIN, "targets": targets.as_str(), "merge": "replace" }))), &artifact_app_laws::meta("local")).await.expect("interactionSelect");
    artifact_app_laws::settle_registered_typed_operation(&mut *app, SHOOTING_TEST_INSTANCE).await.expect("the selection settles");
    let captured = app.interaction_state().await;
    eprintln!("[DEBUG] Shooting idless before dispatch selection={:?}", captured.selection.get(SHOOTING_INTERACTION_DOMAIN).map(|selection| (&selection.granularity, &selection.ids)));
    let rows = dispatch_rows(&mut app, translate(Vec::new(), 3.0)).await;
    let after_dispatch = app.interaction_state().await;
    eprintln!("[DEBUG] Shooting idless after dispatch rows={} selection={:?}", rows.len(), after_dispatch.selection.get(SHOOTING_INTERACTION_DOMAIN).map(|selection| (&selection.granularity, &selection.ids)));
    assert_eq!(rows.len(), 1, "the selection fallback commits one transaction");
    assert_eq!(origin(&app, &asset_id), [before[0] + 3.0, before[1], before[2]]);
}

/// ⏪️ LAW: the yielded leaf is parametric — its payload value edited as time travel edits it (`dx` 1 → 5) decodes back
/// into a leaf that applies relative to whatever base precedes it, on the committed base and on a base an upstream
/// edit already moved.
#[semio_framework_async_macros::async_test]
async fn the_drag_leaf_replays_its_edited_offset_relative_to_any_base() {
    let base = crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    let asset_id = base.assets[0].id.clone();
    let (transaction, mutations) = shooting_gumball_commit("translateSelection", "seed", GumballToolRequest::on(&base, ShootingMutation::DragAssets(crate::mutations::drag_assets::DragAssets { asset_ids: vec![asset_id.clone()], dx: 1.0, dy: 0.0, dz: 0.0 }))).expect("a moving gesture commits");
    assert!(transaction.id.starts_with("tx-"));
    let mut value = Mutation::<ShootingSnapshot>::payload_value(&mutations[0]);
    let semio_framework_value::DslValue::Object(entries) = &mut value else { panic!("a leaf payload is an object") };
    entries.iter_mut().find(|(key, _)| key == "dx").expect("dx input").1 = semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(5.0));
    let edited = Mutation::<ShootingSnapshot>::with_payload_value(&mutations[0], value).expect("the edited payload decodes");
    let apply = |snapshot: &ShootingSnapshot, operation: &ShootingMutation| operation.diff(snapshot).into_parts().0.apply(snapshot).expect("the leaf applies");
    let start = base.assets[0].origin;
    assert_eq!(apply(&base, &edited).assets[0].origin, [start[0] + 5.0, start[1], start[2]], "the edited offset replays on the committed base");
    let moved = apply(&base, &ShootingMutation::DragAssets(crate::mutations::drag_assets::DragAssets { asset_ids: vec![asset_id], dx: 0.0, dy: 7.0, dz: 0.0 }));
    assert_eq!(apply(&moved, &edited).assets[0].origin, [start[0] + 5.0, start[1] + 7.0, start[2]], "the edited offset replays relative to a moved base");
}

/// ⏪️ LAW: a gumball drag edited in history replays its downstream through the store: the preview base is the state
/// right before the drag, and the Report replay re-applies the downstream relative turn and scaling onto the edited
/// drag — exactly the fresh fold of the edited log; overwrite commits it.
#[semio_framework_async_macros::async_test]
async fn a_gumball_drag_edited_in_history_replays_its_downstream() {
    use crate::mutations::{drag_assets::DragAssets, rotate_assets::RotateAssets, scale_assets::ScaleAssets};
    use protocol::OpBinary;
    let base = crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    let asset_ids = vec![base.assets[0].id.clone()];
    let drag = |dx: f64, dy: f64| ShootingMutation::DragAssets(DragAssets { asset_ids: asset_ids.clone(), dx, dy, dz: 0.0 });
    let log = [drag(1.0, 0.0), ShootingMutation::RotateAssets(RotateAssets { asset_ids: asset_ids.clone(), ax: 0.0, ay: 0.0, az: 1.0, angle: 0.5 }), ShootingMutation::ScaleAssets(ScaleAssets { asset_ids: asset_ids.clone(), sx: 2.0, sy: 1.0, sz: 1.0 })];
    let mut store = store::ArtifactStore::<ShootingSnapshot, ShootingMutation>::new(store::create_document_envelope::<ShootingSnapshot, ShootingMutation>(crate::SHOOTING_DOCUMENT_SCHEMA, "gumball-time-travel", base.clone(), None), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.expect("the store opens");
    store.install_document_store_owners_exact(semio_framework_plugin::bounded_document_store_owners::<ShootingSnapshot, ShootingMutation>());
    for mutation in &log {
        store.dispatch(store::ArtifactCommand::Apply { mutations: vec![mutation.clone()], transaction: None }).await.expect("the edit applies");
    }
    let ids: Vec<protocol::MutationId> = store.mutation_ops().expect("applied operations").into_iter().map(|operation| operation.mutation_id).collect();
    let edited = drag(-2.0, 4.0);
    let drafts: std::collections::BTreeMap<protocol::MutationId, protocol::InputReplacement> = [(ids[0].clone(), protocol::InputReplacement::Input { schema: crate::SHOOTING_DOCUMENT_SCHEMA.into(), payload: edited.encode_op().expect("the edited leaf encodes") })].into_iter().collect();
    assert_eq!(store.state_before(&ids[0], &drafts).expect("the preview base folds").as_ref(), &base, "the preview base is the state right before the edited drag");
    let mut replay = store.begin_report_replay(&drafts, Some(&ids[0])).expect("the replay begins at the edited drag");
    assert!(matches!(replay.step(store.replay_edits(), &mut || false).expect("the replay steps"), store::ReplayStep::Finished(_)));
    let result = replay.finish().expect("a finished replay yields its result");
    assert!(!store.replay_report(&result).expect("report").blocks_finalize(), "a re-offset drag never blocks finalizing");
    let fold = |mutations: &[ShootingMutation]| mutations.iter().fold(base.clone(), |snapshot, mutation| crate::mutations::apply_shooting_mutation(&snapshot, mutation).expect("the log folds"));
    let fresh = fold(&[edited, log[1].clone(), log[2].clone()]);
    assert_ne!(fresh, fold(&log), "the edit changes the outcome");
    assert_eq!(result.state().expect("the replay reached a state").as_ref(), &fresh, "the replay equals the fresh fold of the edited log");
    store.commit_finished_replay(result, store::HistoryFinalization::Overwrite).await.expect("overwrite commits");
    assert_eq!(store.snapshot_ref(), &fresh, "the overwritten history folds to the edited state");
    let mut disposer = semio_framework_plugin::bounded_document_store_disposer::<ShootingSnapshot, ShootingMutation>();
    for _ in 0..4_096 {
        if disposer.terminal_is_empty(&store) {
            break;
        }
        disposer.close_step(&mut store, 1, 1 << 20).expect("the store retires");
    }
    assert!(disposer.terminal_is_empty(&store), "the standalone store retires to its terminal-empty shell");
}
