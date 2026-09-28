use super::*;
use crate::editor::generation3d::unit_tests::context::{self, app, app_with_registry, dispatch};
use crate::editor::generation3d::Generation3dCommand;
use crate::widget_id;
use semio_framework_artifact_flow_flow::Widget;

const SCALE_ID: &str = "extrude__gumball_scale";

fn scale_factors(snapshot: &FlowHostSnapshot) -> [f64; 3] {
    with_host(snapshot, |host| {
        let widget = gumball_widget_json(host, SCALE_ID).unwrap();
        let factor = widget.get("params").unwrap().get("factor").unwrap();
        ["x", "y", "z"].map(|key| factor.get(key).unwrap().as_f64().unwrap())
    })
}

/// 📏️ Each axis scales independently and repeated gestures compose in the same widget.
#[semio_framework_async_macros::async_test]
async fn scale_selection_multiplies_each_axis_in_one_transform_neuron() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    dispatch(&mut app, Generation3dCommand::ScaleSelection(ScaleSelection { node_ids: vec!["extrude".into()], sx: 1.0, sy: 2.0, sz: 3.0 })).await;
    let once = context::snapshot(&app);
    let neuron = once.host_snapshot.widgets.iter().find(|widget| widget_id(widget) == SCALE_ID).expect("scale neuron spliced");
    assert!(matches!(neuron, Widget::Neuron { neuron_kind, .. } if neuron_kind == "brep.xform.scale"));
    assert_eq!(scale_factors(&once.host_snapshot), [1.0, 2.0, 3.0]);

    dispatch(&mut app, Generation3dCommand::ScaleSelection(ScaleSelection { node_ids: vec![SCALE_ID.into()], sx: 3.0, sy: 3.0, sz: 3.0 })).await;
    let twice = context::snapshot(&app);
    assert_eq!(twice.host_snapshot.widgets.iter().filter(|widget| widget_id(widget) == SCALE_ID).count(), 1);
    assert_eq!(scale_factors(&twice.host_snapshot), [3.0, 6.0, 9.0]);
}

/// 🎯️ The context-menu shape: no ids in the payload, the FRAMEWORK-owned `graph` selection decides.
#[semio_framework_async_macros::async_test]
async fn an_ids_less_scale_transforms_the_framework_owned_graph_selection() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app_with_registry().await;
    context::select_graph(&mut app, "node", &["extrude"]).await;
    dispatch(&mut app, Generation3dCommand::ScaleSelection(ScaleSelection { node_ids: Vec::new(), sx: 4.0, sy: 4.0, sz: 4.0 })).await;
    let projection = context::snapshot(&app);
    assert!(projection.host_snapshot.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { id, neuron_kind, .. } if id == SCALE_ID && neuron_kind == "brep.xform.scale")));
    assert_eq!(scale_factors(&projection.host_snapshot), [4.0; 3]);
    drop(projection);
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut *app);
}

/// 🧯️ A scale that names nothing at all changes nothing.
#[semio_framework_async_macros::async_test]
async fn a_scale_with_no_target_leaves_the_graph_untouched() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    let before = context::snapshot(&app).host_snapshot.widgets.len();
    dispatch(&mut app, Generation3dCommand::ScaleSelection(ScaleSelection { node_ids: Vec::new(), sx: 2.0, sy: 2.0, sz: 2.0 })).await;
    let after = context::snapshot(&app);
    assert_eq!(after.host_snapshot.widgets.len(), before);
    assert!(!after.host_snapshot.widgets.iter().any(|widget| widget_id(widget) == SCALE_ID));
}
