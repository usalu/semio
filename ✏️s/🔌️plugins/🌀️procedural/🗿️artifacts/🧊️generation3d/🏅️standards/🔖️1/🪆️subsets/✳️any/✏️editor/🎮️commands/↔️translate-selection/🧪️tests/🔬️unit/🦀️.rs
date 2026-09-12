use super::*;
use crate::editor::generation3d::testkit::{app, dispatch};
use crate::editor::generation3d::Generation3dCommand;
use crate::widget_id;
use semio_framework_artifact_flow_flow::Widget;
use crate::editor::generation3d::testkit;

#[semio_framework_async_macros::async_test]
async fn translate_selection_persists_transform_into_flow_graph() {
    let _serial = crate::editor::generation3d::test_support::lock();
    let mut app = app().await;
    let before = testkit::snapshot(&app);
    assert!(before.fixture.synapses.iter().any(|synapse| synapse.from == "extrude" && synapse.to == "column-preview"));
    dispatch(&mut app, Generation3dCommand::TranslateSelection(TranslateSelection { node_ids: vec!["extrude".into()], dx: 1.0, dy: 2.0, dz: 3.0 })).await;
    let projection = testkit::snapshot(&app);
    let transform_id = "extrude__gumball_translate";
    let transform = projection.fixture.widgets.iter().find(|widget| widget_id(widget) == transform_id).expect("transform neuron created");
    assert!(matches!(transform, Widget::Neuron { neuron_kind, .. } if neuron_kind == "brep.xform.translate"));
    let offset = with_host(&projection.fixture, |host| gumball_widget_offset(host, transform_id));
    assert_eq!(offset, [1.0, 2.0, 3.0]);

    // Re-grabbing the same transform accumulates the delta instead of creating a second node.
    dispatch(&mut app, Generation3dCommand::TranslateSelection(TranslateSelection { node_ids: vec![transform_id.into()], dx: 1.0, dy: 0.0, dz: 0.0 })).await;
    let projection2 = testkit::snapshot(&app);
    assert_eq!(projection2.fixture.widgets.iter().filter(|widget| widget_id(widget) == transform_id).count(), 1);
    assert_eq!(with_host(&projection2.fixture, |host| gumball_widget_offset(host, transform_id)), [2.0, 2.0, 3.0]);
}
