use super::*;
use crate::editor::generation3d::unit_tests::context::{self, app, app_with_registry, dispatch};
use crate::editor::generation3d::Generation3dCommand;
use crate::widget_id;
use semio_framework_artifact_flow_flow::Widget;

const ROTATE_ID: &str = "extrude__gumball_rotate";

#[semio_framework_async_macros::async_test]
async fn rotate_selection_composes_world_axes_and_preserves_the_result_selection() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    context::select_graph(&mut app, "node", &["extrude"]).await;
    dispatch(&mut app, Generation3dCommand::RotateSelection(RotateSelection { node_ids: Vec::new(), ax: 1.0, ay: 0.0, az: 0.0, angle: std::f64::consts::FRAC_PI_2 })).await;
    dispatch(&mut app, Generation3dCommand::RotateSelection(RotateSelection { node_ids: Vec::new(), ax: 0.0, ay: 1.0, az: 0.0, angle: std::f64::consts::FRAC_PI_2 })).await;
    let snapshot = context::snapshot(&app);
    let params = with_host(&snapshot.host_snapshot, |host| gumball_widget_json(host, ROTATE_ID).unwrap());
    let axis = params.get("params").unwrap().get("axis").unwrap();
    for (key, expected) in [("x", 1.0), ("y", 1.0), ("z", -1.0)] {
        assert!((axis.get(key).unwrap().as_f64().unwrap() - expected / 3.0_f64.sqrt()).abs() < 1e-12);
    }
    let angle = with_host(&snapshot.host_snapshot, |host| gumball_widget_number_param(host, ROTATE_ID, "angle", 0.0));
    assert!((angle - std::f64::consts::TAU / 3.0).abs() < 1e-12);
    drop(snapshot);
    dispatch(&mut app, Generation3dCommand::TranslateSelection(crate::editor::generation3d::commands::translate_selection::TranslateSelection { node_ids: Vec::new(), dx: 1.0, dy: 0.0, dz: 0.0 })).await;
    let snapshot = context::snapshot(&app);
    assert!(snapshot.host_snapshot.synapses.iter().any(|wire| wire.from == ROTATE_ID && wire.to == "extrude__gumball_rotate__gumball_translate"));
    drop(snapshot);
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
    eprintln!("[DEBUG] rotation gestures: world-axis composition and chained transform selection verified");
}

#[test]
fn rotate_selection_reports_invalid_parameters_and_targets_atomically() {
    let _serial = crate::test_serial::lock();
    let snapshot = crate::standards::v1::subsets::any::schema::example_snapshot(crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_MESH_WORKBENCH).unwrap();
    let count = snapshot.host_snapshot.widgets.len();
    assert!(rotate_ids(&snapshot.host_snapshot, &["extrude".into()], 0.0, 0.0, 0.0, 1.0).is_err());
    assert!(rotate_ids(&snapshot.host_snapshot, &["extrude".into(), "missing".into()], 0.0, 0.0, 1.0, 1.0).is_err());
    assert_eq!(snapshot.host_snapshot.widgets.len(), count);
    snapshot.retire_cold();
}

/// 🔄️ A rotate names its own transform neuron once and ACCUMULATES into it — a second grab must not
/// splice a second node, or a gumball drag would grow the graph one neuron per frame.
#[semio_framework_async_macros::async_test]
async fn rotate_selection_accumulates_one_transform_neuron_in_the_flow_graph() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    dispatch(&mut app, Generation3dCommand::RotateSelection(RotateSelection { node_ids: vec!["extrude".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: std::f64::consts::FRAC_PI_2 })).await;
    let once = context::snapshot(&app);
    let neuron = once.host_snapshot.widgets.iter().find(|widget| widget_id(widget) == ROTATE_ID).expect("rotate neuron spliced");
    assert!(matches!(neuron, Widget::Neuron { neuron_kind, .. } if neuron_kind == "brep.xform.rotate"));
    assert!((with_host(&once.host_snapshot, |host| gumball_widget_number_param(host, ROTATE_ID, "angle", 0.0)) - std::f64::consts::FRAC_PI_2).abs() < 1e-12);

    dispatch(&mut app, Generation3dCommand::RotateSelection(RotateSelection { node_ids: vec![ROTATE_ID.into()], ax: 0.0, ay: 0.0, az: 1.0, angle: std::f64::consts::FRAC_PI_2 })).await;
    let twice = context::snapshot(&app);
    assert_eq!(twice.host_snapshot.widgets.iter().filter(|widget| widget_id(widget) == ROTATE_ID).count(), 1);
    assert!((with_host(&twice.host_snapshot, |host| gumball_widget_number_param(host, ROTATE_ID, "angle", 0.0)) - std::f64::consts::PI).abs() < 1e-12);
}

/// 🎯️ An ids-less rotate is the real context-menu shape: the payload carries nothing and the command
/// reads the FRAMEWORK-owned `graph` selection instead (`apply_selected`, fed from
/// `protocol::InteractionState`). Before this test that fallback was exercised only as a menu-row
/// presence assertion, never as dispatch-and-assert-the-mutation.
#[semio_framework_async_macros::async_test]
async fn an_ids_less_rotate_transforms_the_framework_owned_graph_selection() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    context::select_graph(&mut app, "node", &["extrude"]).await;
    dispatch(&mut app, Generation3dCommand::RotateSelection(RotateSelection { node_ids: Vec::new(), ax: 0.0, ay: 0.0, az: 1.0, angle: std::f64::consts::FRAC_PI_4 })).await;
    let projection = context::snapshot(&app);
    assert!(projection.host_snapshot.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { id, neuron_kind, .. } if id == ROTATE_ID && neuron_kind == "brep.xform.rotate")));
    assert!((with_host(&projection.host_snapshot, |host| gumball_widget_number_param(host, ROTATE_ID, "angle", 0.0)) - std::f64::consts::FRAC_PI_4).abs() < 1e-12);
    drop(projection);
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

/// 🧯️ A rotate that names nothing at all changes nothing — no neuron, no mutation, no coalesce key.
#[semio_framework_async_macros::async_test]
async fn a_rotate_with_no_target_leaves_the_graph_untouched() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    let before = context::snapshot(&app).host_snapshot.widgets.len();
    dispatch(&mut app, Generation3dCommand::RotateSelection(RotateSelection { node_ids: Vec::new(), ax: 0.0, ay: 0.0, az: 1.0, angle: 1.0 })).await;
    let after = context::snapshot(&app);
    assert_eq!(after.host_snapshot.widgets.len(), before);
    assert!(!after.host_snapshot.widgets.iter().any(|widget| widget_id(widget) == ROTATE_ID));
}
