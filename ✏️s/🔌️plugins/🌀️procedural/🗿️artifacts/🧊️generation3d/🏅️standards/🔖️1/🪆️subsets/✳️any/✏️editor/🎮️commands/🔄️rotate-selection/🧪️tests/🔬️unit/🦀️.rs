use super::*;
use crate::editor::generation3d::testkit::{self, app, app_with_registry, dispatch};
use crate::editor::generation3d::Generation3dCommand;
use crate::widget_id;
use semio_framework_artifact_flow_flow::Widget;

const ROTATE_ID: &str = "extrude__gumball_rotate";

/// 🔄️ A rotate names its own transform neuron once and ACCUMULATES into it — a second grab must not
/// splice a second node, or a gumball drag would grow the graph one neuron per frame.
#[semio_framework_async_macros::async_test]
async fn rotate_selection_accumulates_one_transform_neuron_in_the_flow_graph() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app().await;
    dispatch(&mut app, Generation3dCommand::RotateSelection(RotateSelection { node_ids: vec!["extrude".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: std::f64::consts::FRAC_PI_2 })).await;
    let once = testkit::snapshot(&app);
    let neuron = once.fixture.widgets.iter().find(|widget| widget_id(widget) == ROTATE_ID).expect("rotate neuron spliced");
    assert!(matches!(neuron, Widget::Neuron { neuron_kind, .. } if neuron_kind == "brep.xform.rotate"));
    assert_eq!(with_host(&once.fixture, |host| gumball_widget_number_param(host, ROTATE_ID, "angle", 0.0)), std::f64::consts::FRAC_PI_2);

    dispatch(&mut app, Generation3dCommand::RotateSelection(RotateSelection { node_ids: vec![ROTATE_ID.into()], ax: 0.0, ay: 0.0, az: 1.0, angle: std::f64::consts::FRAC_PI_2 })).await;
    let twice = testkit::snapshot(&app);
    assert_eq!(twice.fixture.widgets.iter().filter(|widget| widget_id(widget) == ROTATE_ID).count(), 1);
    assert_eq!(with_host(&twice.fixture, |host| gumball_widget_number_param(host, ROTATE_ID, "angle", 0.0)), std::f64::consts::PI);
}

/// 🎯️ An ids-less rotate is the real context-menu shape: the payload carries nothing and the command
/// reads the FRAMEWORK-owned `graph` selection instead (`apply_selected`, fed from
/// `protocol::InteractionState`). Before this test that fallback was exercised only as a menu-row
/// presence assertion, never as dispatch-and-assert-the-mutation.
#[semio_framework_async_macros::async_test]
async fn an_ids_less_rotate_transforms_the_framework_owned_graph_selection() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app_with_registry().await;
    testkit::select_graph(&mut app, "node", &["extrude"]).await;
    dispatch(&mut app, Generation3dCommand::RotateSelection(RotateSelection { node_ids: Vec::new(), ax: 0.0, ay: 0.0, az: 1.0, angle: std::f64::consts::FRAC_PI_4 })).await;
    let projection = testkit::snapshot(&app);
    assert!(projection.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { id, neuron_kind, .. } if id == ROTATE_ID && neuron_kind == "brep.xform.rotate")));
    assert_eq!(with_host(&projection.fixture, |host| gumball_widget_number_param(host, ROTATE_ID, "angle", 0.0)), std::f64::consts::FRAC_PI_4);
    drop(projection);
    semio_framework_plugin::testkit::close_registered_fixture_app(&mut *app);
}

/// 🧯️ A rotate that names nothing at all changes nothing — no neuron, no mutation, no coalesce key.
#[semio_framework_async_macros::async_test]
async fn a_rotate_with_no_target_leaves_the_graph_untouched() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app().await;
    let before = testkit::snapshot(&app).fixture.widgets.len();
    dispatch(&mut app, Generation3dCommand::RotateSelection(RotateSelection { node_ids: Vec::new(), ax: 0.0, ay: 0.0, az: 1.0, angle: 1.0 })).await;
    let after = testkit::snapshot(&app);
    assert_eq!(after.fixture.widgets.len(), before);
    assert!(!after.fixture.widgets.iter().any(|widget| widget_id(widget) == ROTATE_ID));
}
