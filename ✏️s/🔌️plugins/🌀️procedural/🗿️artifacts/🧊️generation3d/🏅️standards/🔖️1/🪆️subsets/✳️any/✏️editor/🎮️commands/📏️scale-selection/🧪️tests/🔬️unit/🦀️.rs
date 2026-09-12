use super::*;
use crate::editor::generation3d::unit_tests::context::{self, app, app_with_registry, dispatch};
use crate::editor::generation3d::Generation3dCommand;
use crate::widget_id;
use semio_framework_artifact_flow_flow::Widget;

const SCALE_ID: &str = "extrude__gumball_scale";

/// 📏️ A scale is UNIFORM — the three axis factors average into one `factor` param — and a second grab
/// MULTIPLIES into the same neuron instead of splicing another one.
#[semio_framework_async_macros::async_test]
async fn scale_selection_multiplies_one_uniform_transform_neuron_in_the_flow_graph() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    dispatch(&mut app, Generation3dCommand::ScaleSelection(ScaleSelection { node_ids: vec!["extrude".into()], sx: 1.0, sy: 2.0, sz: 3.0 })).await;
    let once = context::snapshot(&app);
    let neuron = once.fixture.widgets.iter().find(|widget| widget_id(widget) == SCALE_ID).expect("scale neuron spliced");
    assert!(matches!(neuron, Widget::Neuron { neuron_kind, .. } if neuron_kind == "brep.xform.scale"));
    assert_eq!(with_host(&once.fixture, |host| gumball_widget_number_param(host, SCALE_ID, "factor", 1.0)), 2.0);

    dispatch(&mut app, Generation3dCommand::ScaleSelection(ScaleSelection { node_ids: vec![SCALE_ID.into()], sx: 3.0, sy: 3.0, sz: 3.0 })).await;
    let twice = context::snapshot(&app);
    assert_eq!(twice.fixture.widgets.iter().filter(|widget| widget_id(widget) == SCALE_ID).count(), 1);
    assert_eq!(with_host(&twice.fixture, |host| gumball_widget_number_param(host, SCALE_ID, "factor", 1.0)), 6.0);
}

/// 🎯️ The context-menu shape: no ids in the payload, the FRAMEWORK-owned `graph` selection decides.
#[semio_framework_async_macros::async_test]
async fn an_ids_less_scale_transforms_the_framework_owned_graph_selection() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    context::select_graph(&mut app, "node", &["extrude"]).await;
    dispatch(&mut app, Generation3dCommand::ScaleSelection(ScaleSelection { node_ids: Vec::new(), sx: 4.0, sy: 4.0, sz: 4.0 })).await;
    let projection = context::snapshot(&app);
    assert!(projection.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { id, neuron_kind, .. } if id == SCALE_ID && neuron_kind == "brep.xform.scale")));
    assert_eq!(with_host(&projection.fixture, |host| gumball_widget_number_param(host, SCALE_ID, "factor", 1.0)), 4.0);
    drop(projection);
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

/// 🧯️ A scale that names nothing at all changes nothing.
#[semio_framework_async_macros::async_test]
async fn a_scale_with_no_target_leaves_the_graph_untouched() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    let before = context::snapshot(&app).fixture.widgets.len();
    dispatch(&mut app, Generation3dCommand::ScaleSelection(ScaleSelection { node_ids: Vec::new(), sx: 2.0, sy: 2.0, sz: 2.0 })).await;
    let after = context::snapshot(&app);
    assert_eq!(after.fixture.widgets.len(), before);
    assert!(!after.fixture.widgets.iter().any(|widget| widget_id(widget) == SCALE_ID));
}
