use crate::editor::sequence::unit_tests::context::{dispatch, live_host_snapshot, new_app};
use crate::editor::sequence::SequenceCommand;

use super::connect_steps::ConnectSteps;
use super::disconnect_steps::DisconnectSteps;

#[semio_framework_async_macros::async_test]
async fn disconnect_then_reconnect_round_trips_the_edge() {
    let mut app = new_app().await;
    dispatch(&mut app, SequenceCommand::DisconnectSteps(DisconnectSteps { from_id: "step-1".into(), to_id: "step-2".into() })).await;
    assert!(live_host_snapshot(&app).await.edges.is_empty());
    dispatch(&mut app, SequenceCommand::ConnectSteps(ConnectSteps { source_node_id: "step-1".into(), target_node_id: "step-2".into() })).await;
    assert_eq!(live_host_snapshot(&app).await.edges.len(), 1);
}
