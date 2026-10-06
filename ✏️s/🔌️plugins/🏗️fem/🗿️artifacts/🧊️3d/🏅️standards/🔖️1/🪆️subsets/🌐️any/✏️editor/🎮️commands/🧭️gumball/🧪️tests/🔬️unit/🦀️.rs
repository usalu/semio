use super::*;
use crate::editor::fem3d::modes::edit::windows::model;
use crate::editor::fem3d::unit_tests::context::{close, dispatch, dispatch_rows, fem3d_demo_app, history_verb, render, view, Fem3dApp};
use crate::editor::fem3d::Fem3dCommand;
use semio_framework::kernel::HistoryEntry;
use semio_framework_plugin::{ArtifactView, ConfigView, HistoryView, NoConfig, ViewModel};

fn demo() -> Fem3dSnapshot {
    crate::standards::v1::subsets::any::io::text::snapshot::fem3d_demo_snapshot()
}

fn translate(dx: f64, phase: Option<&str>) -> Fem3dCommand {
    Fem3dCommand::TranslateSelection(translate_selection::TranslateSelection { ids: vec!["n20_l1".into()], dx, dy: 0.0, dz: 0.0, phase: phase.map(str::to_string), reason: None })
}

fn node_position(app: &Fem3dApp, id: &str) -> [f64; 3] {
    let snapshot = app.snapshot().expect("snapshot");
    let node = snapshot.nodes.iter().find(|node| node.id == id).expect("node");
    [node.x, node.y, node.z]
}

fn english(row: &HistoryEntry) -> String {
    row.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En).to_string()
}

/// 🧭️ The unmounted route commits a one-shot as the relative `move-selection` leaf — never a per-step amend — refreshes
/// both world windows, publishes nothing for a step that moves nothing, and refuses a streamed phase, which needs the
/// retained route's transient.
#[test]
fn the_unmounted_route_commits_one_relative_leaf_and_refuses_a_stream() {
    let doc = demo();
    let history = HistoryView::empty();
    let view = ArtifactView::new(&doc, &history);
    let cfg = ConfigView { snapshot: &NoConfig::default(), window: None };
    let emit = translate_selection::handle(&translate_selection::TranslateSelection { ids: vec!["n20_l1".into()], dx: 0.5, dy: -0.25, dz: 0.0, phase: None, reason: None }, &view, &cfg).expect("emit");
    let [Fem3dMutation::MoveSelection(leaf)] = emit.artifact_mutations.as_slice() else { panic!("one relative leaf: {:?}", emit.artifact_mutations) };
    assert_eq!((leaf.node_ids.clone(), leaf.dx, leaf.dy), (vec!["n20_l1".to_string()], 0.5, -0.25));
    let UiDirtyScope::Partial { window_bodies, .. } = &emit.ui_scope else { panic!("partial scope") };
    assert_eq!(window_bodies, &vec![model::FEM3D_BODY_MODEL.to_string(), results_window::FEM3D_BODY_RESULTS.to_string()]);
    let idle = translate_selection::handle(&translate_selection::TranslateSelection { ids: vec!["n20_l1".into()], dx: 0.0, dy: 0.0, dz: 0.0, phase: None, reason: None }, &view, &cfg).expect("emit");
    assert!(idle.artifact_mutations.is_empty(), "a zero step publishes nothing");
    let rotate = rotate_selection::handle(&rotate_selection::RotateSelection { ids: vec!["n20_l1".into(), "n00_l1".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 0.3, phase: None, reason: None }, &view, &cfg).expect("emit");
    assert!(matches!(rotate.artifact_mutations.as_slice(), [Fem3dMutation::MoveSelection(leaf)] if leaf.angle == 0.3));
    let scale = scale_selection::handle(&scale_selection::ScaleSelection { ids: vec!["sol1".into()], sx: 2.0, sy: 1.0, sz: 1.0, phase: None, reason: None }, &view, &cfg).expect("emit");
    assert!(matches!(scale.artifact_mutations.as_slice(), [Fem3dMutation::MoveSelection(leaf)] if leaf.solid_ids == ["sol1"] && leaf.sx == 2.0));
    assert!(translate_selection::handle(&translate_selection::TranslateSelection { ids: vec!["n20_l1".into()], dx: 0.5, dy: 0.0, dz: 0.0, phase: Some("stream".into()), reason: None }, &view, &cfg).is_err());
}

/// 🕹️ LAW: an id-less payload moves the live framework selection.
#[test]
fn an_id_less_payload_moves_the_live_selection() {
    let doc = demo();
    let history = HistoryView::empty();
    let view = ArtifactView::new(&doc, &history);
    let cfg = ConfigView { snapshot: &NoConfig::default(), window: None };
    let live = crate::editor::fem3d::fem3d_route(&Fem3dCommand::TranslateSelection(translate_selection::TranslateSelection { ids: Vec::new(), dx: 0.0, dy: 0.0, dz: 1.0, phase: None, reason: None }), &view, &cfg, || vec!["n20_l1".into()], None).expect("route");
    let [Fem3dMutation::MoveSelection(leaf)] = live.artifact_mutations.as_slice() else { panic!("the live selection moved") };
    assert_eq!(leaf.node_ids, vec!["n20_l1".to_string()]);
}

/// 🛠️ LAW: one gumball drag is one edit and one history row keyed by its tool transaction, labelled from the leaf.
#[semio_framework_async_macros::async_test]
async fn one_drag_is_one_edit_one_row_and_one_transaction() {
    let mut app = fem3d_demo_app().await;
    let before = node_position(&app, "n20_l1");
    let rows = dispatch_rows(&mut app, translate(0.5, None)).await;
    assert_eq!(rows.len(), 1, "one drag, one row: {rows:?}");
    let transaction = rows[0].transaction.as_ref().expect("the row is keyed by its tool transaction");
    assert_eq!(transaction.tool, "s.fem.fem3d@1/*#editor#translateSelection");
    assert_eq!((rows[0].op_count, rows[0].mutations.len()), (1, 1));
    assert!(rows[0].mutations[0].editable, "the yielded leaf is history-editable");
    assert_eq!(english(&rows[0]), "Move 1 node by (0.5, 0, 0)");
    assert_eq!(node_position(&app, "n20_l1")[0], before[0] + 0.5);
    close(&mut app);
}

/// 🌊️ LAW: a gesture streamed over several dispatches is ONE transaction — previewed by the model window while it is
/// open, never history — and its commit publishes the net leaf as one edit; undo restores the node exactly.
#[semio_framework_async_macros::async_test]
async fn a_streamed_gesture_is_one_transaction_with_a_preview() {
    let mut app = fem3d_demo_app().await;
    let before = node_position(&app, "n20_l1");
    let idle = render(&mut app, model::FEM3D_BODY_MODEL);
    assert!(dispatch_rows(&mut app, translate(0.25, Some("stream"))).await.is_empty(), "a stream tick is no history");
    assert!(dispatch_rows(&mut app, translate(0.5, Some("stream"))).await.is_empty());
    assert_eq!(node_position(&app, "n20_l1"), before, "the committed document never moves mid-gesture");
    let previewed = render(&mut app, model::FEM3D_BODY_MODEL);
    assert_ne!(previewed, idle, "the model window paints the open gesture's preview");
    let rows = dispatch_rows(&mut app, translate(0.0, Some("commit"))).await;
    assert_eq!(rows.len(), 1, "the commit is one row: {rows:?}");
    assert_eq!(english(&rows[0]), "Move 1 node by (0.75, 0, 0)", "the row holds the NET leaf");
    assert_eq!(node_position(&app, "n20_l1")[0], before[0] + 0.75);
    history_verb(&mut app, "undo").await;
    assert_eq!(node_position(&app, "n20_l1"), before, "undo restores the node exactly");
    close(&mut app);
}

/// 🧯️ LAW: a host abort drops the open gesture with zero trace, and two drags are two transactions.
#[semio_framework_async_macros::async_test]
async fn an_abort_leaves_zero_trace_and_two_drags_are_two_transactions() {
    let mut app = fem3d_demo_app().await;
    let before = node_position(&app, "n20_l1");
    dispatch(&mut app, translate(0.25, Some("stream"))).await;
    let aborted = dispatch_rows(&mut app, Fem3dCommand::TranslateSelection(translate_selection::TranslateSelection { ids: vec!["n20_l1".into()], dx: 0.0, dy: 0.0, dz: 0.0, phase: Some("abort".into()), reason: Some("captureLost".into()) })).await;
    assert!(aborted.is_empty(), "an abort publishes nothing");
    assert_eq!(node_position(&app, "n20_l1"), before);
    let first = dispatch_rows(&mut app, translate(0.5, None)).await;
    let second = dispatch_rows(&mut app, translate(0.5, None)).await;
    assert_ne!(first[0].transaction.as_ref().expect("first").id, second[0].transaction.as_ref().expect("second").id, "consecutive drags never share a transaction");
    assert_eq!(node_position(&app, "n20_l1")[0], before[0] + 1.0);
    close(&mut app);
}

/// 🎚️ LAW: the handle flags live on the model window's config, are refused on a results window and
/// for an unknown flag, and toggle when no explicit value is given.
#[semio_framework_async_macros::async_test]
async fn gumball_flags_are_model_window_config() {
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let model_view = view(model::FEM3D_WINDOW_MODEL);
    let emit = set_transform_gumball_flag::handle_window(&set_transform_gumball_flag::SetTransformGumballFlag { flag: "rotate".into(), pressed: None }, &cfg, &model_view).expect("toggle");
    assert_eq!(emit.window_config_mutations.len(), 1);
    assert_eq!(emit.window_config_mutations[0].window_id(), "model-left");
    let results_view = view(results_window::FEM3D_WINDOW_RESULTS);
    assert!(set_transform_gumball_flag::handle_window(&set_transform_gumball_flag::SetTransformGumballFlag { flag: "rotate".into(), pressed: None }, &cfg, &results_view).is_err());
    assert!(set_transform_gumball_flag::handle_window(&set_transform_gumball_flag::SetTransformGumballFlag { flag: "mirror".into(), pressed: None }, &cfg, &model_view).is_err());
    assert!(set_transform_gumball_flag::handle_window(&set_transform_gumball_flag::SetTransformGumballFlag { flag: "rotate".into(), pressed: None }, &cfg, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).is_err());
    let mut app = fem3d_demo_app().await;
    dispatch(&mut app, Fem3dCommand::SetTransformGumballFlag(set_transform_gumball_flag::SetTransformGumballFlag { flag: "rotate".into(), pressed: Some(false) })).await;
    close(&mut app);
}

/// 🛠️ LAW — the World3d live-consumer API end to end (`🌐️World3dHost/🧫️fixtures/🛠️gumball-live-protocol.json`): every
/// verb dispatch the host owes for a scripted gesture, decoded from its wire args exactly as the host sends them (the
/// targets swapped for one fem3d node), publishes exactly the fixture's guest edits and moves the node by its offset — a
/// streamed gesture is ONE edit, an aborted one leaves zero trace, a gesture that never moved publishes nothing.
#[semio_framework_async_macros::async_test]
async fn the_world3d_gumball_live_protocol_lands_as_its_guest_edits() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧫️fixtures/🛠️gumball-live-protocol.json"))).expect("the protocol fixture parses");
    let cases = fixture["cases"].as_array().expect("cases");
    let mut consumed = 0;
    for case in cases.iter().filter(|case| case.get("guest").is_some()) {
        let name = case["name"].as_str().expect("name");
        let mut app = fem3d_demo_app().await;
        let before = node_position(&app, "n20_l1");
        let mut edits = 0;
        for dispatch in case["steps"].as_array().expect("steps").iter().map(|step| &step["dispatch"]).filter(|dispatch| !dispatch.is_null()) {
            let mut args = dispatch["args"].clone();
            args["ids"] = serde_json::json!(["n20_l1"]);
            let args: semio_framework_value::DslValue = semio_framework_pack_json::from_json_str(&args.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("wire args parse");
            let action = dispatch["action"].as_str().expect("action");
            let command = <EditorApp<Fem3dPlayApp> as semio_framework_plugin::ArtifactApp>::command_from_action(action, Some(&args)).await.unwrap_or_else(|fault| panic!("{name}: {action} decodes from the host's wire args: {}", fault.message));
            edits += dispatch_rows(&mut app, command).await.len();
        }
        let after = node_position(&app, "n20_l1");
        let offset: Vec<f64> = case["guest"]["offset"].as_array().expect("offset").iter().map(|value| value.as_f64().expect("number")).collect();
        assert_eq!(edits as u64, case["guest"]["edits"].as_u64().expect("edits"), "{name}: the published edits");
        assert_eq!([after[0] - before[0], after[1] - before[1], after[2] - before[2]], [offset[0], offset[1], offset[2]], "{name}: the node's offset");
        close(&mut app);
        consumed += 1;
    }
    assert!(consumed >= 7, "every guest case of the protocol fixture is consumed: {consumed}");
}
