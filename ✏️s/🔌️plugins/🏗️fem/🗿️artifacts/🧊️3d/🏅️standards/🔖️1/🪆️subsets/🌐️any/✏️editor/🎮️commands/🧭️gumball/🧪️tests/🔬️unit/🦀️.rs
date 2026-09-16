use super::*;
use crate::editor::fem3d::modes::edit::windows::model;
use crate::editor::fem3d::unit_tests::context::{dispatch, fem3d_app, view, Fem3dApp};
use crate::editor::fem3d::Fem3dCommand;
use semio_framework_plugin::{ArtifactView, ConfigView, HistoryView, NoConfig, ViewModel};

fn demo() -> Fem3dSnapshot {
    crate::standards::v1::subsets::any::schema::snapshot::text::fem3d_boot_snapshot()
}

fn node_position(app: &Fem3dApp, id: &str) -> [f64; 3] {
    let snapshot = app.snapshot().expect("snapshot");
    let node = snapshot.nodes.iter().find(|node| node.id == id).expect("node");
    [node.x, node.y, node.z]
}

/// 🪢️ LAW: every drag step spells whole-record replaces under the one coalesce key of its kind and
/// refreshes both window bodies — the results window re-solves while the drag is still going.
#[test]
fn translate_selection_coalesces_with_the_gumball_key_and_refreshes_both_windows() {
    let doc = demo();
    let history = HistoryView::empty();
    let view = ArtifactView::new(&doc, &history);
    let cfg = ConfigView { snapshot: &NoConfig::default(), window: None };
    let emit = translate_selection::handle(&translate_selection::TranslateSelection { ids: vec!["n20_l1".into()], dx: 0.5, dy: -0.25, dz: 0.0 }, &view, &cfg).expect("emit");
    assert_eq!(emit.coalesce_key.as_deref(), Some(COALESCE_TRANSLATE));
    let [Fem3dMutation::ReplaceNode(replace)] = emit.artifact_mutations.as_slice() else { panic!("one replace") };
    assert_eq!(replace.id, "n20_l1");
    let UiDirtyScope::Partial { window_bodies, .. } = &emit.ui_scope else { panic!("partial scope") };
    assert_eq!(window_bodies, &vec![model::FEM3D_BODY_MODEL.to_string(), results_window::FEM3D_BODY_RESULTS.to_string()]);
    let idle = translate_selection::handle(&translate_selection::TranslateSelection { ids: vec!["n20_l1".into()], dx: 0.0, dy: 0.0, dz: 0.0 }, &view, &cfg).expect("emit");
    assert!(idle.artifact_mutations.is_empty() && idle.coalesce_key.is_none(), "a zero step publishes nothing");
    let rotate = rotate_selection::handle(&rotate_selection::RotateSelection { ids: vec!["n20_l1".into(), "n00_l1".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 0.3 }, &view, &cfg).expect("emit");
    assert_eq!(rotate.coalesce_key.as_deref(), Some(COALESCE_ROTATE));
    let scale = scale_selection::handle(&scale_selection::ScaleSelection { ids: vec!["sol1".into()], sx: 2.0, sy: 1.0, sz: 1.0 }, &view, &cfg).expect("emit");
    assert_eq!(scale.coalesce_key.as_deref(), Some(COALESCE_SCALE));
}

/// 🕹️ LAW: a drag of incremental steps accumulates on the live app, and the typed route resolves an
/// empty id list against the live framework selection.
#[semio_framework_async_macros::async_test]
async fn a_drag_of_incremental_steps_accumulates_on_the_live_app() {
    let mut app = fem3d_app();
    let before = node_position(&app, "n20_l1");
    for _ in 0..3 {
        dispatch(&mut app, Fem3dCommand::TranslateSelection(translate_selection::TranslateSelection { ids: vec!["n20_l1".into()], dx: 0.1, dy: 0.0, dz: 0.0 })).await;
    }
    let after = node_position(&app, "n20_l1");
    assert!((after[0] - before[0] - 0.3).abs() < 1e-9, "three steps of 0.1 m: {before:?} → {after:?}");
    let history = HistoryView::empty();
    let snapshot = app.snapshot().expect("snapshot");
    let doc = ArtifactView::new(&snapshot, &history);
    let cfg = ConfigView { snapshot: &NoConfig::default(), window: None };
    let mut interaction = protocol::InteractionState::default();
    interaction.selection.insert(crate::editor::fem3d::interaction::FEM3D_INTERACTION_DOMAIN.into(), protocol::DomainSelection { ids: vec!["n20_l1".into()], ..Default::default() });
    let live = crate::editor::fem3d::fem3d_route(&Fem3dCommand::TranslateSelection(translate_selection::TranslateSelection { ids: Vec::new(), dx: 0.0, dy: 0.0, dz: 1.0 }), &doc, &cfg, || interaction.selection.get(crate::editor::fem3d::interaction::FEM3D_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()).unwrap_or_default(), None).expect("route");
    let [Fem3dMutation::ReplaceNode(replace)] = live.artifact_mutations.as_slice() else { panic!("the live selection moved") };
    assert_eq!(replace.id, "n20_l1");
    dispatch(&mut app, Fem3dCommand::SetTransformGumballFlag(set_transform_gumball_flag::SetTransformGumballFlag { flag: "rotate".into(), pressed: Some(false) })).await;
}

/// 🎚️ LAW: the handle flags live on the model window's config, are refused on a results window and
/// for an unknown flag, and toggle when no explicit value is given.
#[semio_framework_async_macros::async_test]
async fn gumball_flags_are_model_window_config() {
    let doc = demo();
    let history = HistoryView::empty();
    let view_doc = ArtifactView::new(&doc, &history);
    let _ = view_doc;
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let model_view = view(model::FEM3D_WINDOW_MODEL);
    let emit = set_transform_gumball_flag::handle_window(&set_transform_gumball_flag::SetTransformGumballFlag { flag: "rotate".into(), pressed: None }, &cfg, &model_view).expect("toggle");
    assert_eq!(emit.window_config_mutations.len(), 1);
    assert_eq!(emit.window_config_mutations[0].window_id(), "model-left");
    let results_view = view(results_window::FEM3D_WINDOW_RESULTS);
    assert!(set_transform_gumball_flag::handle_window(&set_transform_gumball_flag::SetTransformGumballFlag { flag: "rotate".into(), pressed: None }, &cfg, &results_view).is_err());
    assert!(set_transform_gumball_flag::handle_window(&set_transform_gumball_flag::SetTransformGumballFlag { flag: "mirror".into(), pressed: None }, &cfg, &model_view).is_err());
    assert!(set_transform_gumball_flag::handle_window(&set_transform_gumball_flag::SetTransformGumballFlag { flag: "rotate".into(), pressed: None }, &cfg, &ViewModel::default()).is_err());
}

/// 🧲️ LAW: the host's drag brackets are accepted from the model window and complete EMPTY — no
/// mutation, no config write, no refresh — because every pose already landed as its own
/// `translateSelection`/`rotateSelection`/`scaleSelection` step; a bracket the shell refused as
/// undeclared is exactly what an un-dragged gumball looked like in the browser.
#[semio_framework_async_macros::async_test]
async fn transform_brackets_complete_empty() {
    let doc = demo();
    let history = HistoryView::empty();
    let view_doc = ArtifactView::new(&doc, &history);
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    for command in [Fem3dCommand::TransformBegin(transform_begin::TransformBegin {}), Fem3dCommand::TransformEnd(transform_end::TransformEnd {})] {
        let emit = crate::editor::fem3d::fem3d_route(&command, &view_doc, &cfg, Vec::new, Some(&view(model::FEM3D_WINDOW_MODEL))).expect("a bracket completes");
        assert!(emit.artifact_mutations.is_empty() && emit.window_config_mutations.is_empty() && emit.effects.is_empty(), "{}: a bracket writes nothing", command.command_id());
        assert!(matches!(emit.ui_scope, UiDirtyScope::None), "{}: a bracket refreshes nothing", command.command_id());
    }
    let mut app = fem3d_app();
    let before = app.snapshot().expect("snapshot");
    dispatch(&mut app, Fem3dCommand::TransformBegin(transform_begin::TransformBegin {})).await;
    dispatch(&mut app, Fem3dCommand::TransformEnd(transform_end::TransformEnd {})).await;
    assert_eq!(app.snapshot().expect("snapshot"), before, "the brackets leave the document untouched through the retained lane too");
}
