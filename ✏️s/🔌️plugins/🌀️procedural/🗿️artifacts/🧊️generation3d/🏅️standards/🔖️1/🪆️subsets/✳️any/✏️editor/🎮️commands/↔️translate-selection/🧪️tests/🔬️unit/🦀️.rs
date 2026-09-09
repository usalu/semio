use super::*;
use crate::editor::generation3d::commands::{rotate_selection, scale_selection};
use crate::editor::generation3d::testkit::{app, dispatch};
use crate::editor::generation3d::Generation3dCommand;
use crate::widget_id;
use semio_framework_artifact_flow_flow::Widget;

#[semio_framework_async_macros::async_test]
async fn translate_selection_persists_transform_into_flow_graph() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app().await;
    let before = app.snapshot().expect("snapshot");
    assert!(before.fixture.synapses.iter().any(|synapse| synapse.from == "extrude" && synapse.to == "column-preview"));
    dispatch(&mut app, Generation3dCommand::TranslateSelection(TranslateSelection { node_ids: vec!["extrude".into()], dx: 1.0, dy: 2.0, dz: 3.0 })).await;
    let projection = app.snapshot().expect("snapshot");
    let transform_id = "extrude__gumball_translate";
    let transform = projection.fixture.widgets.iter().find(|widget| widget_id(widget) == transform_id).expect("transform neuron created");
    assert!(matches!(transform, Widget::Neuron { neuron_kind, .. } if neuron_kind == "brep.xform.translate"));
    let offset = gumball_widget_offset(&host_from_fixture(&projection.fixture), transform_id);
    assert_eq!(offset, [1.0, 2.0, 3.0]);

    // Re-grabbing the same transform accumulates the delta instead of creating a second node.
    dispatch(&mut app, Generation3dCommand::TranslateSelection(TranslateSelection { node_ids: vec![transform_id.into()], dx: 1.0, dy: 0.0, dz: 0.0 })).await;
    let projection2 = app.snapshot().expect("snapshot");
    assert_eq!(projection2.fixture.widgets.iter().filter(|widget| widget_id(widget) == transform_id).count(), 1);
    assert_eq!(gumball_widget_offset(&host_from_fixture(&projection2.fixture), transform_id), [2.0, 2.0, 3.0]);
}

#[semio_framework_async_macros::async_test]
async fn rotate_and_scale_selection_persist_into_flow_graph() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut rotate_app = app().await;
    dispatch(&mut rotate_app, Generation3dCommand::RotateSelection(rotate_selection::RotateSelection { node_ids: vec!["extrude".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: std::f64::consts::FRAC_PI_2 })).await;
    let rotated = rotate_app.snapshot().expect("snapshot");
    let rotate_id = "extrude__gumball_rotate";
    assert!(rotated.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { id, neuron_kind, .. } if id == rotate_id && neuron_kind == "brep.xform.rotate")));

    let mut scale_app = app().await;
    dispatch(&mut scale_app, Generation3dCommand::ScaleSelection(scale_selection::ScaleSelection { node_ids: vec!["extrude".into()], sx: 2.0, sy: 2.0, sz: 2.0 })).await;
    let scaled = scale_app.snapshot().expect("snapshot");
    let scale_id = "extrude__gumball_scale";
    assert!(scaled.fixture.widgets.iter().any(|widget| matches!(widget, Widget::Neuron { id, neuron_kind, .. } if id == scale_id && neuron_kind == "brep.xform.scale")));
}
