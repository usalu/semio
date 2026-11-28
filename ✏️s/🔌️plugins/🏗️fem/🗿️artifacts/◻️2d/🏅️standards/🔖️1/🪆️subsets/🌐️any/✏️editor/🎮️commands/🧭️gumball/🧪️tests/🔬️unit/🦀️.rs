use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::gumball::translate_selection::handle;
use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::gumball::translate_selection::TranslateSelection;
use crate::context::close;
use crate::context::dispatch;
use crate::context::dispatch_rows;
use crate::context::fem2d_mounted_app;
use crate::context::history_verb;
use crate::context::render;
use crate::context::Fem2dApp;
use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dCommand;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_framework::kernel::HistoryEntry;
use semio_plugin::ArtifactView;
use semio_plugin::ConfigView;
use semio_plugin::HistoryView;
use semio_plugin::NoConfig;

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotRetiring;

fn demo() -> Fem2dSnapshotRetiring {
    Fem2dSnapshotRetiring::new(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::default_fem2d_snapshot().expect("the bundled demo fixture parses"))
}

fn translate(dx: f64, phase: Option<&str>) -> Fem2dCommand {
    Fem2dCommand::TranslateSelection(TranslateSelection { ids: vec!["scope:1:4:node2:n1".into()], dx, dy: 0.0, dz: 0.0, phase: phase.map(str::to_string), reason: None })
}

fn x_of(app: &Fem2dApp, id: &str) -> f64 {
    app.snapshot_ref().nodes.iter().find(|node| node.id.as_str() == id).expect("node").x
}

fn english(row: &HistoryEntry) -> String {
    row.label.resolve(semio_ui_locale::Terminology::Native, semio_ui_locale::Locale::En).to_string()
}

/// 🧭️ The unmounted route commits a one-shot as the relative `move-selection` leaf — never a per-tick amend — and
/// refuses a streamed phase, which needs the retained route's transient.
#[test]
fn the_unmounted_route_commits_one_relative_leaf_and_refuses_a_stream() {
    let doc = demo();
    let history = HistoryView::empty();
    let view = ArtifactView::new(&*doc, &history);
    let cfg = ConfigView { snapshot: &NoConfig::default(), window: None };
    let emit = handle(&TranslateSelection { ids: vec!["scope:1:4:node2:n1".into()], dx: 0.5, dy: -0.25, dz: 0.0, phase: None, reason: None }, &view, &cfg).expect("emit");
    let [Fem2dMutation::MoveSelection(leaf)] = emit.artifact_mutations.as_slice() else { panic!("one relative leaf: {:?}", emit.artifact_mutations) };
    assert_eq!((leaf.node_ids.clone(), leaf.dx, leaf.dy), (vec!["n1".to_string()], 0.5, -0.25));
    assert!(handle(&TranslateSelection { ids: vec!["scope:1:4:node2:n1".into()], dx: 0.5, dy: 0.0, dz: 0.0, phase: Some("stream".into()), reason: None }, &view, &cfg).is_err());
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

/// 🌊️ LAW: a gesture streamed over several dispatches (every tick CUMULATIVE from the press) is ONE transaction —
/// previewed by both canvas windows while it is open, never history — and its commit publishes the net leaf as one edit;
/// undo restores the node exactly.
#[semio_framework_async_macros::async_test]
async fn a_streamed_gesture_is_one_transaction_with_a_preview() {
    let mut app = fem2d_mounted_app();
    let before = x_of(&app, "n1");
    let idle = render(&mut app, semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::BODY_KEY);
    assert!(dispatch_rows(&mut app, translate(0.25, Some("stream"))).await.is_empty(), "a stream tick is no history");
    assert!(dispatch_rows(&mut app, translate(0.75, Some("stream"))).await.is_empty());
    assert_eq!(x_of(&app, "n1"), before, "the committed document never moves mid-gesture");
    let previewed = render(&mut app, semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model::BODY_KEY);
    assert_ne!(previewed, idle, "the model window paints the open gesture's preview");
    let rows = dispatch_rows(&mut app, translate(0.75, Some("commit"))).await;
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
    let aborted = dispatch_rows(&mut app, Fem2dCommand::TranslateSelection(TranslateSelection { ids: vec!["scope:1:4:node2:n1".into()], dx: 0.0, dy: 0.0, dz: 0.0, phase: Some("abort".into()), reason: Some("blur".into()) })).await;
    assert!(aborted.is_empty(), "an abort publishes nothing");
    assert_eq!(x_of(&app, "n1"), before);
    let first = dispatch_rows(&mut app, translate(0.5, None)).await;
    let second = dispatch_rows(&mut app, translate(0.5, None)).await;
    assert_ne!(first[0].transaction.as_ref().expect("first").id, second[0].transaction.as_ref().expect("second").id, "consecutive drags never share a transaction");
    assert_eq!(x_of(&app, "n1"), before + 1.0);
    close(&mut app);
}

fn window_view(kind: &str) -> semio_plugin::ViewModel {
    semio_plugin::ViewModel { window_id: Some("model-left".into()), window_instances: vec![semio_plugin::ViewWindowInstance { id: "model-left".into(), window_kind_id: kind.into() }], ..semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native) }
}

/// 🎚️ LAW: the handle flags live on the model window's config, are refused on a results window and
/// for an unknown flag, and toggle when no explicit value is given. The window config is what the model
/// body and the options rail read, so a toggle changes the drawing as well as the pressed state.
#[semio_framework_async_macros::async_test]
async fn gumball_flags_are_model_window_config() {
    use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::gumball::set_transform_gumball_flag::handle_window;
    use semio_s_artifact_fem_2d_editor::editor::fem2d::commands::gumball::set_transform_gumball_flag::SetTransformGumballFlag;
    use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model;
    use semio_s_artifact_fem_2d_editor_results_window::editor::fem2d::modes::edit::windows::results;
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let toggle = |flag: &str| SetTransformGumballFlag { flag: flag.into(), pressed: None };
    let emit = handle_window(&toggle("rotate"), &cfg, &window_view(model::WINDOW_KIND_ID)).expect("toggle");
    assert_eq!(emit.window_config_mutations.len(), 1);
    assert_eq!(emit.window_config_mutations[0].window_id(), "model-left");
    assert!(handle_window(&toggle("rotate"), &cfg, &window_view(results::WINDOW_KIND_ID)).is_err());
    assert!(handle_window(&toggle("mirror"), &cfg, &window_view(model::WINDOW_KIND_ID)).is_err());
    assert!(handle_window(&toggle("rotate"), &cfg, &semio_plugin::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)).is_err());
    let mut app = fem2d_mounted_app();
    dispatch(&mut app, Fem2dCommand::SetTransformGumballFlag(SetTransformGumballFlag { flag: "rotate".into(), pressed: Some(false) })).await;
    close(&mut app);
}

/// 🌊️ LAW (T1 accuracy): stream ticks admitted faster than they publish (a fast pointer) never wedge the app, and a tick
/// that went stale terminates as its own refusal WITHOUT costing the gesture anything: the host sends every tick and the
/// commit cumulative from the press, so whichever ticks survive, the commit moves the node by exactly the full gesture
/// (0.875) as one edit, and undo restores it.
#[semio_framework_async_macros::async_test]
async fn stream_ticks_in_flight_together_never_lose_the_gesture() {
    let mut app = fem2d_mounted_app();
    let before = x_of(&app, "n1");
    crate::context::dispatch_unsettled(&mut app, translate(0.25, Some("stream"))).await;
    crate::context::dispatch_unsettled(&mut app, translate(0.75, Some("stream"))).await;
    crate::context::dispatch_unsettled(&mut app, translate(0.875, Some("stream"))).await;
    let settled = crate::context::settle_pending(&mut app).await;
    assert!(settled.as_ref().err().is_none_or(|refusal| refusal.contains("interactive-job.stale")), "only the benign interactive-job.stale refusal may end an overlapping tick: {settled:?}");
    assert_eq!(x_of(&app, "n1"), before, "the committed document never moves mid-gesture");
    let rows = dispatch_rows(&mut app, translate(0.875, Some("commit"))).await;
    assert_eq!(rows.len(), 1, "the commit is one row: {rows:?}");
    assert_eq!(x_of(&app, "n1"), before + 0.875, "the committed gesture moved the node by exactly the full offset");
    history_verb(&mut app, "undo").await;
    assert_eq!(x_of(&app, "n1"), before, "and undo restores it exactly");
    close(&mut app);
}
