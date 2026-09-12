use crate::editor::sequence::unit_tests::context::{dispatch, new_app};
use crate::editor::sequence::SequenceCommand;

use super::connect_steps::ConnectSteps;
use super::disconnect_steps::DisconnectSteps;

#[semio_framework_async_macros::async_test]
async fn disconnect_then_reconnect_round_trips_the_edge() {
    let mut app = new_app().await;
    dispatch(&mut app, SequenceCommand::DisconnectSteps(DisconnectSteps { from_id: "step-1".into(), to_id: "step-2".into() })).await;
    assert!(app.snapshot().expect("projection").to_fixture().edges.is_empty());
    dispatch(&mut app, SequenceCommand::ConnectSteps(ConnectSteps { source_node_id: "step-1".into(), target_node_id: "step-2".into() })).await;
    assert_eq!(app.snapshot().expect("projection").to_fixture().edges.len(), 1);
}
