use super::translate_selection::{handle, TranslateSelection};
use crate::editor::fem2d::unit_tests::context::{close, dispatch, dispatch_rows, fem2d_mounted_app, history_verb, render, Fem2dApp};
use crate::editor::fem2d::Fem2dCommand;
use crate::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_framework::kernel::HistoryEntry;
use semio_framework_plugin::{ArtifactView, ConfigView, HistoryView, NoConfig};
use store::ArtifactDsl;

type Fem2dSnapshot = crate::Fem2dSnapshot;

fn demo() -> Fem2dSnapshot {
    Fem2dSnapshot::parse_dsl(crate::editor::fem2d::FEM2D_EXAMPLE_DSL).expect("demo document parses")
}

fn translate(dx: f64, phase: Option<&str>) -> Fem2dCommand {
    Fem2dCommand::TranslateSelection(TranslateSelection { ids: vec!["n1".into()], dx, dy: 0.0, dz: 0.0, phase: phase.map(str::to_string), reason: None })
}

fn x_of(app: &Fem2dApp, id: &str) -> f64 {
    app.snapshot().expect("snapshot").nodes.iter().find(|node| node.id == id).expect("node").x
}

fn english(row: &HistoryEntry) -> String {
    row.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En).to_string()
}

/// 🧭️ The unmounted route commits a one-shot as the relative `move-selection` leaf — never a per-tick amend — and
/// refuses a streamed phase, which needs the retained route's transient.
#[test]
fn the_unmounted_route_commits_one_relative_leaf_and_refuses_a_stream() {
    let doc = demo();
    let history = HistoryView::empty();
    let view = ArtifactView::new(&doc, &history);
    let cfg = ConfigView { snapshot: &NoConfig::default(), window: None };
    let emit = handle(&TranslateSelection { ids: vec!["n1".into()], dx: 0.5, dy: -0.25, dz: 0.0, phase: None, reason: None }, &view, &cfg).expect("emit");
    let [Fem2dMutation::MoveSelection(leaf)] = emit.artifact_mutations.as_slice() else { panic!("one relative leaf: {:?}", emit.artifact_mutations) };
    assert_eq!((leaf.node_ids.clone(), leaf.dx, leaf.dy), (vec!["n1".to_string()], 0.5, -0.25));
    assert!(handle(&TranslateSelection { ids: vec!["n1".into()], dx: 0.5, dy: 0.0, dz: 0.0, phase: Some("stream".into()), reason: None }, &view, &cfg).is_err());
}

/// 🛠️ LAW: one gumball drag is one edit and one history row keyed by its tool transaction, labelled from the leaf.
#[semio_framework_async_macros::async_test]
async fn one_drag_is_one_edit_one_row_and_one_transaction() {
    let mut app = fem2d_mounted_app();
    let before = x_of(&app, "n1");
    let rows = dispatch_rows(&mut app, translate(0.5, None)).await;
    assert_eq!(rows.len(), 1, "one drag, one row: {rows:?}");
    let transaction = rows[0].transaction.as_ref().expect("the row is keyed by its tool transaction");
    assert_eq!(transaction.tool, "s.fem.fem2d@1/*#editor#translateSelection");
    assert_eq!((rows[0].op_count, rows[0].mutations.len()), (1, 1));
    assert!(rows[0].mutations[0].editable, "the yielded leaf is history-editable");
    assert_eq!(english(&rows[0]), "Move 1 node by (0.5, 0)");
    assert_eq!(x_of(&app, "n1"), before + 0.5);
    close(&mut app);
}

/// 🌊️ LAW: a gesture streamed over several dispatches is ONE transaction — previewed by both canvas windows while it
/// is open, never history — and its commit publishes the net leaf as one edit; undo restores the node exactly.
#[semio_framework_async_macros::async_test]
async fn a_streamed_gesture_is_one_transaction_with_a_preview() {
    let mut app = fem2d_mounted_app();
    let before = x_of(&app, "n1");
    let idle = render(&mut app, crate::editor::fem2d::modes::edit::windows::model::BODY_KEY);
    assert!(dispatch_rows(&mut app, translate(0.25, Some("stream"))).await.is_empty(), "a stream tick is no history");
    assert!(dispatch_rows(&mut app, translate(0.5, Some("stream"))).await.is_empty());
    assert_eq!(x_of(&app, "n1"), before, "the committed document never moves mid-gesture");
    let previewed = render(&mut app, crate::editor::fem2d::modes::edit::windows::model::BODY_KEY);
    assert_ne!(previewed, idle, "the model window paints the open gesture's preview");
    let rows = dispatch_rows(&mut app, translate(0.0, Some("commit"))).await;
    assert_eq!(rows.len(), 1, "the commit is one row: {rows:?}");
    assert_eq!(english(&rows[0]), "Move 1 node by (0.75, 0)", "the row holds the NET leaf");
    assert_eq!(x_of(&app, "n1"), before + 0.75);
    history_verb(&mut app, "undo").await;
    assert_eq!(x_of(&app, "n1"), before, "undo restores the node exactly");
    close(&mut app);
}

/// 🧯️ LAW: a host abort drops the open gesture with zero trace, and two drags are two transactions.
#[semio_framework_async_macros::async_test]
async fn an_abort_leaves_zero_trace_and_two_drags_are_two_transactions() {
    let mut app = fem2d_mounted_app();
    let before = x_of(&app, "n1");
    dispatch(&mut app, translate(0.25, Some("stream"))).await;
    let aborted = dispatch_rows(&mut app, Fem2dCommand::TranslateSelection(TranslateSelection { ids: vec!["n1".into()], dx: 0.0, dy: 0.0, dz: 0.0, phase: Some("abort".into()), reason: Some("blur".into()) })).await;
    assert!(aborted.is_empty(), "an abort publishes nothing");
    assert_eq!(x_of(&app, "n1"), before);
    let first = dispatch_rows(&mut app, translate(0.5, None)).await;
    let second = dispatch_rows(&mut app, translate(0.5, None)).await;
    assert_ne!(first[0].transaction.as_ref().expect("first").id, second[0].transaction.as_ref().expect("second").id, "consecutive drags never share a transaction");
    assert_eq!(x_of(&app, "n1"), before + 1.0);
    close(&mut app);
}
