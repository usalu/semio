use super::*;
use crate::editor::gis3d::modes::view::windows::terrain;
use crate::editor::gis3d::unit_tests::context::{app, close, dispatch, history_verb, main_window_view, render, Gis3dApp};
use semio_framework_plugin::artifact_app_laws::{meta, settle_registered_typed_operation};
use crate::editor::gis3d::Gis3dCommand;
use semio_framework_plugin::PluginApp;

#[semio_framework_async_macros::async_test]
async fn seeds_exaggeration_from_the_terrain_fixture() {
    let mut app = app().await;
    assert_eq!(app.snapshot().expect("projection").exaggeration, 1.5);
    close(&mut app);
}

//#region 🎚️Scrub
/// 🎚️ The dispatching Terrain window's meta, the window a slider press belongs to.
fn window_meta() -> semio_framework_plugin::ActionMeta {
    semio_framework_plugin::ActionMeta { view_state: Some(main_window_view()), ..meta("local") }
}

fn number(value: f64) -> dsl::DslValue {
    dsl::DslValue::float(value)
}

/// 🎚️ One dispatch of the exaggeration slider as the hosts send it: `{value, gesture, commit}`, or a host cancel
/// `{gesture, abort}` that carries no value and is settled by the runtime itself.
async fn slide(app: &mut Gis3dApp, args: Vec<(&str, dsl::DslValue)>) {
    let aborting = args.iter().any(|(key, _)| *key == "abort");
    let args = dsl::DslValue::Object(args.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
    app.handle_action("setExaggeration", Some(&args), &window_meta()).await.expect("the slider dispatch is admitted");
    if !aborting {
        settle_registered_typed_operation(app, window_meta().instance_id).await.expect("the slider dispatch settles");
    }
}

async fn tick(app: &mut Gis3dApp, gesture: &str, value: f64) {
    slide(app, vec![("value", number(value)), ("gesture", dsl::DslValue::String(gesture.into())), ("commit", dsl::DslValue::Bool(false))]).await;
}

async fn release(app: &mut Gis3dApp, gesture: &str, value: f64) {
    slide(app, vec![("value", number(value)), ("gesture", dsl::DslValue::String(gesture.into())), ("commit", dsl::DslValue::Bool(true))]).await;
}

/// 🧾️ The history rows that carry a document edit, oldest first.
async fn edit_rows(app: &mut Gis3dApp) -> Vec<semio_framework::kernel::HistoryEntry> {
    let mut rows: Vec<_> = app.history_snapshot().await.expect("history").upserts.into_iter().filter(|entry| entry.edit_id.is_some()).collect();
    rows.sort_by_key(|entry| entry.seq);
    rows
}

fn exaggeration(app: &Gis3dApp) -> f64 {
    app.snapshot().expect("projection").exaggeration
}

/// ⚖️ LAW: one slider press is ONE tool transaction — its ticks publish nothing (the committed document stays, the
/// render previews the value), the release publishes ONE edit whose one history row carries the press's
/// `TransactionRef` (tool `<appId>#setExaggeration`) and is labelled from the leaf in every language, and one undo
/// restores the value before the press.
#[semio_framework_async_macros::async_test]
async fn a_slider_press_is_one_transaction_one_edit_and_one_row() {
    let mut app = app().await;
    let rows_before = edit_rows(&mut app).await.len();
    let resting = render(&mut app, terrain::GIS3D_PLAY_BODY_COMPOSITE).await;
    tick(&mut app, "terrain.exaggeration:1", 2.0).await;
    tick(&mut app, "terrain.exaggeration:1", 2.5).await;
    assert_eq!(exaggeration(&app), 1.5, "ticks never touch the committed document");
    assert_eq!(edit_rows(&mut app).await.len(), rows_before, "ticks are no history");
    assert_ne!(render(&mut app, terrain::GIS3D_PLAY_BODY_COMPOSITE).await, resting, "the render previews the scrubbed value");
    release(&mut app, "terrain.exaggeration:1", 3.0).await;
    assert_eq!(exaggeration(&app), 3.0);
    let rows = edit_rows(&mut app).await;
    assert_eq!(rows.len(), rows_before + 1, "one press, one edit, one row");
    let row = rows.last().expect("the press's row");
    let transaction = row.transaction.as_ref().expect("the row is the press's tool transaction");
    assert!(transaction.id.starts_with("tx-") && transaction.tool.ends_with("#setExaggeration"), "{transaction:?}");
    assert_eq!(row.mutations.len(), 1, "one absolute leaf: the net value");
    assert_eq!(row.label.resolve(protocol::Terminology::Native, protocol::Locale::En), "Change terrain exaggeration to 3");
    assert_eq!(row.label.resolve(protocol::Terminology::Native, protocol::Locale::De), "Geländeüberhöhung auf 3 ändern");
    history_verb(&mut app, "undo").await;
    assert_eq!(exaggeration(&app), 1.5, "one undo restores the value before the press");
    close(&mut app);
}

/// ⚖️ LAW: a host cancel leaves zero trace — no edit, no row, the render back on the committed document — and a late
/// tick or release of the cancelled press stays silent.
#[semio_framework_async_macros::async_test]
async fn a_cancelled_press_leaves_zero_trace() {
    let mut app = app().await;
    let rows_before = edit_rows(&mut app).await.len();
    let resting = render(&mut app, terrain::GIS3D_PLAY_BODY_COMPOSITE).await;
    tick(&mut app, "terrain.exaggeration:2", 4.0).await;
    slide(&mut app, vec![("gesture", dsl::DslValue::String("terrain.exaggeration:2".into())), ("abort", dsl::DslValue::String("blur".into()))]).await;
    assert_eq!(render(&mut app, terrain::GIS3D_PLAY_BODY_COMPOSITE).await, resting, "the preview is gone");
    release(&mut app, "terrain.exaggeration:2", 4.5).await;
    assert_eq!(exaggeration(&app), 1.5);
    assert_eq!(edit_rows(&mut app).await.len(), rows_before, "zero trace");
    close(&mut app);
}

/// ⚖️ LAW: two presses are two transactions — two edits, two rows, two distinct refs — and a dispatch without a
/// press (palette, agent) stays one plain edit.
#[semio_framework_async_macros::async_test]
async fn two_presses_are_two_transactions() {
    let mut app = app().await;
    let rows_before = edit_rows(&mut app).await.len();
    tick(&mut app, "terrain.exaggeration:3", 2.0).await;
    release(&mut app, "terrain.exaggeration:3", 2.2).await;
    tick(&mut app, "terrain.exaggeration:4", 3.0).await;
    release(&mut app, "terrain.exaggeration:4", 3.3).await;
    let rows = edit_rows(&mut app).await;
    assert_eq!(rows.len(), rows_before + 2);
    let refs: Vec<_> = rows[rows_before..].iter().map(|row| row.transaction.clone().expect("a transaction row")).collect();
    assert_ne!(refs[0].id, refs[1].id, "two presses, two transactions");
    slide(&mut app, vec![("value", number(4.0))]).await;
    let rows = edit_rows(&mut app).await;
    assert_eq!((rows.len(), rows.last().and_then(|row| row.transaction.clone())), (rows_before + 3, None), "a dispatch without a press is a plain edit");
    assert_eq!(exaggeration(&app), 4.0);
    close(&mut app);
}

async fn history_edit(app: &mut Gis3dApp, verb: &str, args: Vec<(&str, dsl::DslValue)>) {
    let args = dsl::DslValue::Object(args.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
    let result = app.handle_action(verb, Some(&args), &window_meta()).await.unwrap_or_else(|fault| panic!("{verb}: {fault:?}"));
    assert!(result.output.get("rejected").is_none(), "{verb} was refused: {:?}", result.output);
}

async fn time_travel_stage(app: &mut Gis3dApp) -> Option<semio_framework::kernel::HistoryTimeTravelStage> {
    app.history_snapshot().await.expect("history").time_travel.map(|status| status.stage)
}

async fn pump_time_travel(app: &mut Gis3dApp, done: impl Fn(Option<semio_framework::kernel::HistoryTimeTravelStage>) -> bool) {
    for _ in 0..10_000 {
        if done(time_travel_stage(app).await) {
            return;
        }
        app.advance_typed_operation_publication().await.expect("a driver turn");
        while app.take_typed_operation_ui_progress().is_some() {}
    }
    panic!("the history edit never settled: {:?}", time_travel_stage(app).await);
}

/// ⚖️ LAW: time travel edits the press's committed value, never the slider: the absolute leaf superseded with another
/// value replays deterministically — the later press, itself absolute, still wins at the head, and withdrawing nothing
/// else the overwrite leaves exactly the head a fresh run of the edited presses reaches.
#[semio_framework_async_macros::async_test]
async fn a_committed_press_edited_in_history_replays_deterministically() {
    let mut app = app().await;
    release(&mut app, "terrain.exaggeration:5", 3.0).await;
    let edited = edit_rows(&mut app).await.last().and_then(|row| row.mutations.first().map(|mutation| mutation.mutation_id.clone())).expect("the press's mutation");
    history_edit(&mut app, "historyEditBegin", vec![("mutationId", dsl::DslValue::String(edited))]).await;
    history_edit(&mut app, "historyEditInput", vec![("path", dsl::DslValue::String("/newExaggeration".into())), ("value", number(2.0))]).await;
    history_edit(&mut app, "historyEditAccept", Vec::new()).await;
    pump_time_travel(&mut app, |stage| stage != Some(semio_framework::kernel::HistoryTimeTravelStage::Replaying)).await;
    assert_eq!(time_travel_stage(&mut app).await, Some(semio_framework::kernel::HistoryTimeTravelStage::Reviewing));
    assert_eq!(exaggeration(&app), 3.0, "reviewing never touches the committed document");
    history_edit(&mut app, "historyEditFinalize", Vec::new()).await;
    history_edit(&mut app, "historyEditCommit", vec![("choice", dsl::DslValue::String("overwrite".into()))]).await;
    pump_time_travel(&mut app, |stage| stage.is_none()).await;
    assert_eq!(exaggeration(&app), 2.0, "the overwrite folds the edited value");
    let mut fresh = app().await;
    release(&mut fresh, "terrain.exaggeration:6", 2.0).await;
    assert_eq!(app.snapshot().expect("edited head"), fresh.snapshot().expect("fresh head"), "the edited log equals a fresh run of the edited press");
    close(&mut app);
    close(&mut fresh);
}
//#endregion 🎚️Scrub

#[semio_framework_async_macros::async_test]
async fn set_exaggeration_is_a_document_operation_not_config_state() {
    let mut app = app().await;
    let result = dispatch(&mut app, Gis3dCommand::SetExaggeration(set_exaggeration::SetExaggeration { exaggeration: 2.0 })).await;
    assert_eq!(result.lanes.iter().filter(|lane| **lane == semio_framework_plugin::app::TypedOperationResultLane::Artifact).count(), 1, "exaggeration publishes exactly one undoable document operation");
    close(&mut app);
}
