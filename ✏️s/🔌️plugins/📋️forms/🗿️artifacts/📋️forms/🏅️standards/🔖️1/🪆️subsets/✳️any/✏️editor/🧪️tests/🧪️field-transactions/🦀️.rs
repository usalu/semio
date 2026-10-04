//! 🎛️ Laws of the forms inspector's continuous controls over the field leaf (design §13.1 + §17.1 of ticket
//! 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): a number scrub is ONE tool transaction of ONE absolute `change-block-field`,
//! a host cancel leaves zero trace, and a time-travel edit of the committed value replays downstream — a later field edit
//! the edited value contradicts is reported Fatal and blocks finalizing, a consistent value finalizes to the head a fresh
//! run of the edited log reaches.

use crate::editor::forms::commands::add_question::AddQuestion;
use crate::editor::forms::unit_tests::context::{dispatch, forms_app, FormsApp};
use crate::editor::forms::FormsCommand;
use crate::FormQuestion;
use semio_framework::kernel::{HistoryEntry, HistoryTimeTravelStage, Severity};
use semio_framework_plugin::artifact_app_laws::{meta, settle_registered_typed_operation};
use semio_framework_plugin::{DslValue, PluginApp};

//#region 🔖️Harness
fn object(entries: Vec<(&str, DslValue)>) -> DslValue {
    DslValue::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}

async fn number_question(app: &mut FormsApp) -> String {
    dispatch(app, FormsCommand::AddQuestion(AddQuestion { kind: "number".into(), step_id: None })).await;
    crate::schema::flatten_questions(&app.snapshot().expect("projection")).into_iter().map(|(_, question)| question).find(|question| question.kind == "number").expect("a number question").id
}

fn question(app: &FormsApp, id: &str) -> FormQuestion {
    crate::schema::locate_question(&app.snapshot().expect("projection"), id).expect("the question").question
}

/// 🎚️ One `patchQuestions` dispatch as the inspector sends it: `{questionIds, field, value}`, with `{gesture, commit}` while
/// it is a press of a number field, settled.
async fn patch(app: &mut FormsApp, id: &str, field: &str, value: f64, press: Option<(&str, bool)>) {
    let mut args = vec![("questionIds", DslValue::Array(vec![DslValue::String(id.into())])), ("field", DslValue::String(field.into())), ("value", DslValue::float(value))];
    if let Some((gesture, commit)) = press {
        args.extend([("gesture", DslValue::String(gesture.into())), ("commit", DslValue::Bool(commit))]);
    }
    app.handle_action("patchQuestions", Some(&object(args)), &meta("local")).await.expect("the field dispatch is admitted");
    settle_registered_typed_operation(app, meta("local").instance_id).await.expect("the field dispatch settles");
}

/// 🧯️ The host cancel of a press (`blur`): it carries no value and is settled by the runtime itself.
async fn cancel(app: &mut FormsApp, gesture: &str) {
    let args = object(vec![("questionIds", DslValue::Array(Vec::new())), ("field", DslValue::String("max".into())), ("gesture", DslValue::String(gesture.into())), ("abort", DslValue::String("blur".into()))]);
    app.handle_action("patchQuestions", Some(&args), &meta("local")).await.expect("the cancel is admitted");
}

/// 🧾️ The history rows that carry a document edit, oldest first.
async fn edit_rows(app: &mut FormsApp) -> Vec<HistoryEntry> {
    let mut rows: Vec<_> = app.history_snapshot().await.expect("history").upserts.into_iter().filter(|entry| entry.edit_id.is_some()).collect();
    rows.sort_by_key(|entry| entry.seq);
    rows
}

async fn history_edit(app: &mut FormsApp, verb: &str, args: Vec<(&str, DslValue)>) -> DslValue {
    app.handle_action(verb, Some(&object(args)), &meta("local")).await.unwrap_or_else(|fault| panic!("{verb}: {fault:?}")).output
}

async fn accepted(app: &mut FormsApp, verb: &str, args: Vec<(&str, DslValue)>) {
    let output = history_edit(app, verb, args).await;
    assert!(output.get("rejected").is_none(), "{verb} was refused: {output:?}");
}

async fn time_travel_stage(app: &mut FormsApp) -> Option<HistoryTimeTravelStage> {
    app.history_snapshot().await.expect("history").time_travel.map(|status| status.stage)
}

async fn pump_time_travel(app: &mut FormsApp, done: impl Fn(Option<HistoryTimeTravelStage>) -> bool) {
    for _ in 0..10_000 {
        if done(time_travel_stage(app).await) {
            return;
        }
        app.advance_typed_operation_publication().await.expect("a driver turn");
        while app.take_typed_operation_ui_progress().is_some() {}
    }
    panic!("the history edit never settled: {:?}", time_travel_stage(app).await);
}

/// 🎛️ A row carries exactly one `change-block-field` leaf, labelled from the field it sets in every language.
fn assert_one_field_leaf(row: &HistoryEntry, id: &str, en: &str, de: &str) {
    assert_eq!(row.mutations.len(), 1, "one field edit is one leaf: {:?}", row.mutations);
    let label = &row.mutations[0].label;
    assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), format!("Change {en} of question \"{id}\""));
    assert_eq!(label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), format!("{de} der Frage \"{id}\" ändern"));
}
//#endregion 🔖️Harness

//#region 🎚️ScrubLaws
/// ⚖️ LAW: a number-field press is ONE tool transaction — its ticks leave the committed question alone, the release lands
/// one edit whose one row carries the press's `TransactionRef` and ONE absolute `change-block-field` (the net value), and
/// a cancelled press leaves zero trace.
#[semio_framework_async_macros::async_test]
async fn a_number_scrub_is_one_transaction_of_one_absolute_field_leaf() {
    let mut app = forms_app().await;
    let id = number_question(&mut app).await;
    let rows = edit_rows(&mut app).await.len();
    patch(&mut app, &id, "max", 90.0, Some(("q.max:1", false))).await;
    patch(&mut app, &id, "max", 70.0, Some(("q.max:1", false))).await;
    assert_eq!(question(&app, &id).max, Some(100.0), "ticks never touch the committed question");
    assert_eq!(edit_rows(&mut app).await.len(), rows, "ticks are no history");
    patch(&mut app, &id, "max", 60.0, Some(("q.max:1", true))).await;
    assert_eq!(question(&app, &id).max, Some(60.0));
    let after = edit_rows(&mut app).await;
    assert_eq!(after.len(), rows + 1, "one press, one row");
    let row = after.last().expect("the press's row");
    let transaction = row.transaction.as_ref().expect("the press is a tool transaction");
    assert!(transaction.tool.ends_with("#patchQuestions"), "{transaction:?}");
    assert_one_field_leaf(row, &id, "maximum", "Maximum");
    patch(&mut app, &id, "max", 20.0, Some(("q.max:2", false))).await;
    cancel(&mut app, "q.max:2").await;
    assert_eq!((question(&app, &id).max, edit_rows(&mut app).await.len()), (Some(60.0), rows + 1), "a cancelled press leaves zero trace");
}

/// ⚖️ LAW: opening a history edit freezes the document — an open press is dropped with zero trace (`frozen`), its further
/// ticks are refused `timeTravel.frozen`, and its late release after the session ended stays silent.
#[semio_framework_async_macros::async_test]
async fn a_history_edit_freezes_an_open_press_with_zero_trace() {
    let mut app = forms_app().await;
    let id = number_question(&mut app).await;
    let rows = edit_rows(&mut app).await;
    let target = rows.last().and_then(|row| row.mutations.first().map(|mutation| mutation.mutation_id.clone())).expect("the added question's mutation");
    patch(&mut app, &id, "max", 90.0, Some(("q.max:6", false))).await;
    accepted(&mut app, "historyEditBegin", vec![("mutationId", DslValue::String(target))]).await;
    let args = object(vec![("questionIds", DslValue::Array(vec![DslValue::String(id.clone())])), ("field", DslValue::String("max".into())), ("value", DslValue::float(80.0)), ("gesture", DslValue::String("q.max:6".into())), ("commit", DslValue::Bool(false))]);
    let Err(refused) = app.handle_action("patchQuestions", Some(&args), &meta("local")).await else { panic!("the frozen press keeps moving: its ticks are refused") };
    assert_eq!(refused.code.0.as_str(), "timeTravel.frozen");
    accepted(&mut app, "historyEditExit", Vec::new()).await;
    pump_time_travel(&mut app, |stage| stage.is_none()).await;
    patch(&mut app, &id, "max", 70.0, Some(("q.max:6", true))).await;
    assert_eq!((question(&app, &id).max, edit_rows(&mut app).await.len()), (Some(100.0), rows.len()), "the frozen press leaves zero trace, its late release stays silent");
}
//#endregion 🎚️ScrubLaws

//#region ⏪️ReplayLaws
/// 🧪️ A number question whose maximum was scrubbed to 60 and whose minimum was then set to 50: the press's mutation id.
async fn scrubbed_then_bounded(app: &mut FormsApp) -> (String, String) {
    let id = number_question(app).await;
    patch(app, &id, "max", 80.0, Some(("q.max:3", false))).await;
    patch(app, &id, "max", 60.0, Some(("q.max:3", true))).await;
    let press = edit_rows(app).await.last().and_then(|row| row.mutations.first().map(|mutation| mutation.mutation_id.clone())).expect("the press's mutation");
    patch(app, &id, "min", 50.0, None).await;
    (id, press)
}

/// ⚖️ LAW: time travel edits the scrubbed VALUE, never the control: lowering the committed maximum below the later
/// minimum replays the downstream `change-block-field` as a Fatal `mutation.invariant` that blocks finalizing; exiting
/// leaves zero trace.
#[semio_framework_async_macros::async_test]
async fn a_history_edit_contradicting_a_downstream_field_reports_it_fatal_and_blocks() {
    let mut app = forms_app().await;
    let (id, press) = scrubbed_then_bounded(&mut app).await;
    accepted(&mut app, "historyEditBegin", vec![("mutationId", DslValue::String(press.clone()))]).await;
    accepted(&mut app, "historyEditInput", vec![("path", DslValue::String("/value".into())), ("value", DslValue::float(40.0))]).await;
    accepted(&mut app, "historyEditAccept", Vec::new()).await;
    pump_time_travel(&mut app, |stage| stage != Some(HistoryTimeTravelStage::Replaying)).await;
    let status = app.history_snapshot().await.expect("history").time_travel.expect("a live session");
    assert_eq!((status.stage, status.blocking, status.worst), (HistoryTimeTravelStage::Reviewing, true, Some(Severity::Fatal)), "{status:?}");
    let rows = edit_rows(&mut app).await;
    let downstream = rows.last().and_then(|row| row.mutations.first()).expect("the minimum's row");
    assert_eq!((downstream.worst, downstream.messages.first().map(|message| message.code.as_str())), (Some(Severity::Fatal), Some("mutation.invariant")), "{downstream:?}");
    assert_eq!(history_edit(&mut app, "historyEditFinalize", Vec::new()).await.get("rejected").and_then(DslValue::as_str), Some("timeTravel.blocked"));
    accepted(&mut app, "historyEditExit", Vec::new()).await;
    pump_time_travel(&mut app, |stage| stage.is_none()).await;
    assert_eq!((question(&app, &id).min, question(&app, &id).max), (Some(50.0), Some(60.0)), "exiting the session leaves zero trace");
}

/// ⚖️ LAW: a consistent edit of the scrubbed maximum replays the downstream minimum cleanly and the overwrite folds the
/// edited value under it — the head equals a fresh run of the edited presses.
#[semio_framework_async_macros::async_test]
async fn a_history_edit_of_a_scrubbed_value_replays_downstream_and_overwrites() {
    let mut app = forms_app().await;
    let (id, press) = scrubbed_then_bounded(&mut app).await;
    accepted(&mut app, "historyEditBegin", vec![("mutationId", DslValue::String(press))]).await;
    accepted(&mut app, "historyEditInput", vec![("path", DslValue::String("/value".into())), ("value", DslValue::float(75.0))]).await;
    accepted(&mut app, "historyEditAccept", Vec::new()).await;
    pump_time_travel(&mut app, |stage| stage != Some(HistoryTimeTravelStage::Replaying)).await;
    let status = app.history_snapshot().await.expect("history").time_travel.expect("a live session");
    assert_eq!((status.stage, status.blocking), (HistoryTimeTravelStage::Reviewing, false), "{status:?}");
    assert_eq!(question(&app, &id).max, Some(60.0), "reviewing never touches the committed document");
    accepted(&mut app, "historyEditFinalize", Vec::new()).await;
    accepted(&mut app, "historyEditCommit", vec![("choice", DslValue::String("overwrite".into()))]).await;
    pump_time_travel(&mut app, |stage| stage.is_none()).await;
    assert_eq!((question(&app, &id).min, question(&app, &id).max), (Some(50.0), Some(75.0)), "the overwrite folds the edited value under the downstream minimum");
    let mut fresh = forms_app().await;
    let fresh_id = number_question(&mut fresh).await;
    patch(&mut fresh, &fresh_id, "max", 75.0, Some(("q.max:4", true))).await;
    patch(&mut fresh, &fresh_id, "min", 50.0, None).await;
    let strip = |question: FormQuestion| FormQuestion { id: String::new(), ..question };
    assert_eq!(strip(question(&app, &id)), strip(question(&fresh, &fresh_id)), "the edited log equals a fresh run of the edited presses");
    let rows = edit_rows(&mut app).await;
    let minimum = rows.iter().find(|row| row.mutations.iter().any(|mutation| mutation.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En).starts_with("Change minimum"))).expect("the minimum's row survives the overwrite");
    assert_one_field_leaf(minimum, &id, "minimum", "Minimum");
}
//#endregion ⏪️ReplayLaws
