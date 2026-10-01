//! ⌨️ Typing-run laws of the writer's prose editor (design §13.2 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): every
//! keystroke a splice-typing host delivers (`textSplice` + `typing`) composes into the window's ONE run, never a history
//! micro-mutation, and the run lands as ONE `splice-text` edit stamped with its `TransactionRef` and labelled with what it typed.
//! Idle and a caret jump split runs, a frozen document refuses typing with zero trace, a peer sees the run only once it commits,
//! a run edited in history replays deterministically, and typing never spends the 64-slot edit ledger.
use crate::editor::writer::commands::set_text;
use crate::editor::writer::unit_tests::context::{main_window_view, WRITER_TYPING_BUFFER};
use crate::editor::writer::{create_writer_app, WriterCommand, WriterPlayApp};
use crate::writer_text;
use semio_framework::kernel::{HistoryEntry, HistoryTimeTravelStage};
use semio_framework_plugin::artifact_app_laws::{close_registered_fixture_app, meta, new_app_with_registry_and_members, paired_registered_apps_with_members, settle_registered_typed_operation};
use semio_framework_plugin::{ActionMeta, EditorApp, Fault, InvocationResult, PluginApp, TextSplice, VcsArtifactApp, TEXT_SPLICE_CONTEXT_SCALARS, TYPING_BUFFER_ARG, TYPING_COMMIT_ARG};

type Replica = VcsArtifactApp<EditorApp<WriterPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>;

fn manifest() -> semio_framework_plugin::App {
    semio_framework_plugin::App { definition: create_writer_app(), examples: Vec::new() }
}

async fn replica() -> Replica {
    let mut app = new_app_with_registry_and_members::<EditorApp<WriterPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(manifest).await;
    app.bind_instance_id(meta("local").instance_id).await;
    app
}

fn window_meta() -> ActionMeta {
    ActionMeta { view_state: Some(main_window_view()), ..meta("local") }
}

async fn seeded(text: &str) -> Replica {
    let mut app = replica().await;
    app.dispatch_typed(WriterCommand::SetText(set_text::SetText { text: text.into() }), &window_meta()).await.expect("the seed is admitted");
    settle_registered_typed_operation(&mut app, meta("local").instance_id).await.expect("the seed publishes");
    app
}

/// ⌨️ One live typing delivery of the main window at `now_ms` (`action` + `args` + the window's `typing` buffer), published.
async fn deliver(app: &mut Replica, action: &str, mut args: Vec<(String, dsl::DslValue)>, now_ms: u64) -> Result<InvocationResult, Fault> {
    args.push((TYPING_BUFFER_ARG.into(), dsl::DslValue::String(WRITER_TYPING_BUFFER.into())));
    app.set_tool_clock_ms(Some(now_ms));
    let result = app.handle_action(action, Some(&dsl::DslValue::Object(args)), &window_meta()).await;
    if result.is_ok() {
        settle_registered_typed_operation(app, meta("local").instance_id).await.expect("the delivery publishes");
    }
    result
}

/// 🏁️ The host's commit signal (`reason`) for the main window's run at `now_ms`.
async fn end_run(app: &mut Replica, reason: &str, now_ms: u64) {
    deliver(app, "textSplice", vec![(TYPING_COMMIT_ARG.into(), dsl::DslValue::String(reason.into()))], now_ms).await.unwrap_or_else(|fault| panic!("the {reason} signal: {fault:?}"));
}

/// ⌨️ One author typing into the main window: the text it sees, its caret (scalars) and its splice sequence.
struct Author {
    view: String,
    caret: usize,
    seq: u64,
}

impl Author {
    fn at(view: &str, caret: usize) -> Self {
        Self { view: view.into(), caret, seq: 0 }
    }
}

/// ⌨️ Types `run` at the author's caret as the React host does: one `textSplice` per keystroke against the author's own view,
/// `step_ms` apart from `now_ms` on; answers the clock after the last keystroke.
async fn type_run(app: &mut Replica, author: &mut Author, run: &str, now_ms: u64, step_ms: u64) -> u64 {
    let mut now = now_ms;
    for scalar in run.chars() {
        let chars: Vec<char> = author.view.chars().collect();
        let next: String = chars[..author.caret].iter().copied().chain(std::iter::once(scalar)).chain(chars[author.caret..].iter().copied()).collect();
        let splice = TextSplice::from_edit(&author.view, &next, TEXT_SPLICE_CONTEXT_SCALARS).expect("a keystroke changes the text");
        author.seq += 1;
        let caret = next.chars().take(author.caret + 1).map(char::len_utf8).sum::<usize>() as u64;
        let args = vec![
            ("start".to_string(), dsl::DslValue::uint(u64::from(splice.start))),
            ("deleted".to_string(), dsl::DslValue::String(splice.deleted)),
            ("insert".to_string(), dsl::DslValue::String(splice.insert)),
            ("before".to_string(), dsl::DslValue::String(splice.before)),
            ("after".to_string(), dsl::DslValue::String(splice.after)),
            ("seq".to_string(), dsl::DslValue::uint(author.seq)),
            ("anchor".to_string(), dsl::DslValue::uint(caret)),
            ("caret".to_string(), dsl::DslValue::uint(caret)),
        ];
        deliver(app, "textSplice", args, now).await.unwrap_or_else(|fault| panic!("keystroke {scalar:?}: {fault:?}"));
        author.view = next;
        author.caret += 1;
        now += step_ms;
    }
    now
}

/// 🧾️ The history rows that carry a document edit, oldest first.
async fn edit_rows(app: &mut Replica) -> Vec<HistoryEntry> {
    let mut rows: Vec<_> = app.history_snapshot().await.expect("history").upserts.into_iter().filter(|entry| entry.edit_id.is_some()).collect();
    rows.sort_by_key(|entry| entry.seq);
    rows
}

/// 🔎️ Every edit row as `seq action edit transaction`, for a failing count to name the rows it saw.
fn describe(rows: &[HistoryEntry]) -> Vec<String> {
    rows.iter().map(|row| format!("{} {} {:?} {:?}", row.seq, row.action_id, row.edit_id, row.transaction.as_ref().map(|transaction| &transaction.id))).collect()
}

fn committed(app: &Replica) -> String {
    writer_text(&app.snapshot().expect("projection"))
}

fn rendered(app: &Replica) -> String {
    writer_text(&app.rendered_snapshot())
}

async fn undo(app: &mut Replica) {
    let admitted = app.handle_action("undo", None, &meta("local")).await.expect("the undo is admitted");
    semio_framework_plugin::app::settle_framework_reserved_admission(app, admitted).await.expect("the undo commits");
    settle_registered_typed_operation(app, meta("local").instance_id).await.expect("the undo publishes");
}

async fn history_edit(app: &mut Replica, verb: &str, args: Vec<(&str, dsl::DslValue)>) {
    let args = dsl::DslValue::Object(args.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
    let result = app.handle_action(verb, Some(&args), &window_meta()).await.unwrap_or_else(|fault| panic!("{verb}: {fault:?}"));
    assert!(result.output.get("rejected").is_none(), "{verb} was refused: {:?}", result.output);
}

async fn time_travel_stage(app: &mut Replica) -> Option<HistoryTimeTravelStage> {
    app.history_snapshot().await.expect("history").time_travel.map(|status| status.stage)
}

async fn pump_time_travel(app: &mut Replica, done: impl Fn(Option<HistoryTimeTravelStage>) -> bool) {
    for _ in 0..10_000 {
        if done(time_travel_stage(app).await) {
            return;
        }
        app.advance_typed_operation_publication().await.expect("a driver turn");
        while app.take_typed_operation_ui_progress().is_some() {}
    }
    panic!("the history edit never settled: {:?}", time_travel_stage(app).await);
}

/// ⚖️ LAW: one typing run is ONE edit and ONE history row. Nothing lands while it is open (every render reads it), its one
/// row carries the run's `TransactionRef` (tool `<appId>#textSplice`) and ONE net `splice-text` labelled with what it typed in
/// every language, and ONE undo reverts the whole run.
#[semio_framework_async_macros::async_test]
async fn one_typing_run_is_one_edit_and_one_row_with_its_transaction() {
    let mut app = seeded("Doc: \nend").await;
    let rows_before = edit_rows(&mut app).await.len();
    let mut author = Author::at("Doc: \nend", 5);
    let now = type_run(&mut app, &mut author, "hello", 1_000, 120).await;
    assert_eq!((committed(&app), rendered(&app)), ("Doc: \nend".to_string(), "Doc: hello\nend".to_string()), "the open run is the overlay, never the committed document");
    assert_eq!(edit_rows(&mut app).await.len(), rows_before, "no history micro-mutation while typing");
    end_run(&mut app, "idle", now + 750).await;
    assert_eq!(committed(&app), "Doc: hello\nend");
    let rows = edit_rows(&mut app).await;
    assert_eq!(rows.len(), rows_before + 1, "one run, one edit, one row: {:?}", describe(&rows));
    let row = rows.last().expect("the run's row");
    let transaction = row.transaction.as_ref().expect("the row is the run's tool transaction");
    assert!(transaction.id.starts_with("tx-") && transaction.tool.ends_with("#textSplice"), "{transaction:?}");
    assert_eq!(row.mutations.len(), 1, "one net splice for the whole run");
    assert_eq!(row.label.resolve(protocol::Terminology::Native, protocol::Locale::En), "Type “hello”");
    assert_eq!(row.label.resolve(protocol::Terminology::Native, protocol::Locale::De), "„hello“ tippen");
    undo(&mut app).await;
    assert_eq!(committed(&app), "Doc: \nend", "one undo reverts the whole run");
    close_registered_fixture_app(&mut app);
}

/// ⚖️ LAW: two runs separated by idle are two transactions. The first keystroke after the idle bound commits the run before
/// it (the window's own lapse, no host signal needed) and opens the next.
#[semio_framework_async_macros::async_test]
async fn two_runs_separated_by_idle_are_two_transactions() {
    let mut app = seeded("").await;
    let rows_before = edit_rows(&mut app).await.len();
    let mut author = Author::at("", 0);
    let after_first = type_run(&mut app, &mut author, "ab", 1_000, 100).await;
    let after_second = type_run(&mut app, &mut author, "cd", after_first + 2_000, 100).await;
    assert_eq!(edit_rows(&mut app).await.len(), rows_before + 1, "the pause committed the first run");
    end_run(&mut app, "blur", after_second).await;
    let rows = edit_rows(&mut app).await;
    assert_eq!(rows.len(), rows_before + 2);
    let ids: Vec<_> = rows[rows_before..].iter().map(|row| row.transaction.as_ref().expect("a run row").id.clone()).collect();
    assert_ne!(ids[0], ids[1], "two runs, two transactions");
    assert_eq!(committed(&app), "abcd");
    close_registered_fixture_app(&mut app);
}

/// ⚖️ LAW: a caret jump splits the run, by the host's commit signal on a pure caret move and by the run's own algebra when
/// the next keystroke does not continue it, so each place typed at is its own edit.
#[semio_framework_async_macros::async_test]
async fn a_caret_jump_splits_the_run() {
    let text = "first line of the document\nsecond line of the document\nthird line of the document";
    let mut app = seeded(text).await;
    let rows_before = edit_rows(&mut app).await.len();
    let mut author = Author::at(text, 5);
    let now = type_run(&mut app, &mut author, "X", 1_000, 50).await;
    end_run(&mut app, "selectionJump", now).await;
    assert_eq!(edit_rows(&mut app).await.len(), rows_before + 1, "the host's caret-move signal ended the run");
    author.caret = author.view.chars().count();
    let now = type_run(&mut app, &mut author, "Y", now + 50, 50).await;
    author.caret = 0;
    let now = type_run(&mut app, &mut author, "Z", now, 50).await;
    assert_eq!(edit_rows(&mut app).await.len(), rows_before + 2, "a keystroke that does not continue the run splits it on its own");
    end_run(&mut app, "blur", now).await;
    assert_eq!(edit_rows(&mut app).await.len(), rows_before + 3);
    assert_eq!(committed(&app), format!("Z{}Y", text.replacen("first", "firstX", 1)));
    close_registered_fixture_app(&mut app);
}

/// ⚖️ LAW: a run open when a history edit begins commits first, so no typed text is lost to it; while the history edit freezes
/// the document a typing delivery is refused `timeTravel.frozen` with zero trace: no run opens, nothing renders, no row.
#[semio_framework_async_macros::async_test]
async fn a_frozen_document_refuses_typing_with_zero_trace() {
    let mut app = seeded("abc").await;
    let target = edit_rows(&mut app).await.last().and_then(|row| row.mutations.first().map(|mutation| mutation.mutation_id.clone())).expect("the seed's mutation");
    let rows_before = edit_rows(&mut app).await.len();
    let mut author = Author::at("abc", 3);
    let now = type_run(&mut app, &mut author, "d", 1_000, 50).await;
    history_edit(&mut app, "historyEditBegin", vec![("mutationId", dsl::DslValue::String(target))]).await;
    assert_eq!((edit_rows(&mut app).await.len(), committed(&app)), (rows_before + 1, "abcd".to_string()), "the open run committed before the history edit froze the document");
    let splice = TextSplice::from_edit("abcd", "abcde", TEXT_SPLICE_CONTEXT_SCALARS).expect("a keystroke");
    let args = vec![
        ("start".to_string(), dsl::DslValue::uint(u64::from(splice.start))),
        ("deleted".to_string(), dsl::DslValue::String(splice.deleted)),
        ("insert".to_string(), dsl::DslValue::String(splice.insert)),
        ("before".to_string(), dsl::DslValue::String(splice.before)),
        ("after".to_string(), dsl::DslValue::String(splice.after)),
        ("seq".to_string(), dsl::DslValue::uint(author.seq + 1)),
        ("anchor".to_string(), dsl::DslValue::uint(5)),
        ("caret".to_string(), dsl::DslValue::uint(5)),
    ];
    let refused = deliver(&mut app, "textSplice", args, now + 10).await;
    assert!(refused.as_ref().is_err_and(|fault| fault.code.0 == "timeTravel.frozen"), "typing into a frozen document is refused: {refused:?}");
    assert_eq!((edit_rows(&mut app).await.len(), rendered(&app)), (rows_before + 1, "abcd".to_string()), "zero trace: no run renders, no row");
    end_run(&mut app, "blur", now + 20).await;
    assert_eq!(edit_rows(&mut app).await.len(), rows_before + 1, "no run opened that a later signal could commit");
    history_edit(&mut app, "historyEditExit", Vec::new()).await;
    pump_time_travel(&mut app, |stage| stage.is_none()).await;
    close_registered_fixture_app(&mut app);
}

/// ⚖️ LAW: a peer never sees an open run in its document or history; the run reaches it as ONE edit once it commits.
#[semio_framework_async_macros::async_test]
async fn a_peer_sees_the_run_only_once_it_commits() {
    let (mut a, mut b) = paired_registered_apps_with_members::<EditorApp<WriterPlayApp>, semio_s_artifact_stdio_semio::SemioMembers, _, _>("mem://writer-typing-runs", || async { manifest() }).await;
    let peer_rows = edit_rows(&mut b).await.len();
    let mut author = Author::at("", 0);
    let now = type_run(&mut a, &mut author, "shared", 1_000, 80).await;
    a.tick_backbone().await.expect("a announces");
    b.tick_backbone().await.expect("b folds");
    assert_eq!((writer_text(&b.snapshot().expect("b")), edit_rows(&mut b).await.len()), (String::new(), peer_rows), "the open run never leaves its author");
    end_run(&mut a, "blur", now).await;
    a.tick_backbone().await.expect("a announces the run");
    b.tick_backbone().await.expect("b folds the run");
    assert_eq!(writer_text(&b.snapshot().expect("b")), "shared");
    let rows = edit_rows(&mut b).await;
    assert_eq!(rows.len(), peer_rows + 1, "the run reaches the peer as ONE edit");
    assert!(rows.last().and_then(|row| row.transaction.as_ref()).is_some_and(|transaction| transaction.tool.ends_with("#textSplice")), "the peer's row names the run's transaction");
    a.detach_backbone().await.expect("a releases its backbone");
    b.detach_backbone().await.expect("b releases its backbone");
    close_registered_fixture_app(&mut a);
    close_registered_fixture_app(&mut b);
}

/// ⚖️ LAW: time travel edits the run's committed text, never the keystrokes. The run's net `/insert` superseded with other
/// text replays deterministically: a later run elsewhere still lands, and the overwritten log reaches exactly the head a fresh
/// session typing the edited run reaches.
#[semio_framework_async_macros::async_test]
async fn a_run_edited_in_history_replays_deterministically() {
    let seed = format!("Doc: \n{}\nend", "lorem ipsum dolor sit amet ".repeat(3));
    let session = |insert: &'static str| {
        let seed = seed.clone();
        async move {
            let mut app = seeded(&seed).await;
            let mut author = Author::at(&seed, 5);
            let now = type_run(&mut app, &mut author, insert, 1_000, 60).await;
            end_run(&mut app, "idle", now + 750).await;
            author.caret = author.view.chars().count();
            let now = type_run(&mut app, &mut author, "!", now + 1_000, 60).await;
            end_run(&mut app, "blur", now).await;
            app
        }
    };
    let mut app = session("hello").await;
    let rows = edit_rows(&mut app).await;
    let edited = rows[rows.len() - 2].mutations.first().map(|mutation| mutation.mutation_id.clone()).expect("the first run's net splice");
    history_edit(&mut app, "historyEditBegin", vec![("mutationId", dsl::DslValue::String(edited))]).await;
    history_edit(&mut app, "historyEditInput", vec![("path", dsl::DslValue::String("/insert".into())), ("value", dsl::DslValue::String("howdy".into()))]).await;
    history_edit(&mut app, "historyEditAccept", Vec::new()).await;
    pump_time_travel(&mut app, |stage| stage != Some(HistoryTimeTravelStage::Replaying)).await;
    assert_eq!(time_travel_stage(&mut app).await, Some(HistoryTimeTravelStage::Reviewing));
    assert_eq!(committed(&app), seed.replacen("Doc: ", "Doc: hello", 1) + "!", "reviewing never touches the committed document");
    history_edit(&mut app, "historyEditFinalize", Vec::new()).await;
    history_edit(&mut app, "historyEditCommit", vec![("choice", dsl::DslValue::String("overwrite".into()))]).await;
    pump_time_travel(&mut app, |stage| stage.is_none()).await;
    assert_eq!(committed(&app), seed.replacen("Doc: ", "Doc: howdy", 1) + "!", "the overwrite folds the edited run and replays the later one");
    let mut fresh = session("howdy").await;
    assert_eq!(app.snapshot().expect("edited head"), fresh.snapshot().expect("fresh head"), "the edited log equals a fresh session typing the edited run");
    close_registered_fixture_app(&mut app);
    close_registered_fixture_app(&mut fresh);
}

/// ⚖️ LAW: 200 typed characters in one uninterrupted run, far more than the 64-slot edit ledger holds, never spend the ledger:
/// the run lands as ONE edit and ONE row, and ONE undo reverts it.
#[semio_framework_async_macros::async_test]
async fn two_hundred_typed_characters_are_one_edit() {
    let typed: String = "The quick brown fox jumps over the lazy dog. Zwölf Boxkämpfer jagen Viktor quer über den großen Sylter Deich. ".chars().cycle().take(200).collect();
    assert!(typed.chars().count() > 64 * 3, "the run must outlast the edit ledger");
    let mut app = seeded("").await;
    let rows_before = edit_rows(&mut app).await.len();
    let edits_before = app.edit_transactions().len();
    let mut author = Author::at("", 0);
    let now = type_run(&mut app, &mut author, &typed, 10_000, 40).await;
    assert_eq!((rendered(&app), app.edit_transactions().len()), (typed.clone(), edits_before), "every key renders, none lands");
    end_run(&mut app, "idle", now + 750).await;
    assert_eq!(committed(&app), typed);
    assert_eq!((edit_rows(&mut app).await.len(), app.edit_transactions().len()), (rows_before + 1, edits_before + 1), "one run, one edit, one row");
    undo(&mut app).await;
    assert_eq!(committed(&app), "", "one undo reverts all 200 characters");
    close_registered_fixture_app(&mut app);
}
