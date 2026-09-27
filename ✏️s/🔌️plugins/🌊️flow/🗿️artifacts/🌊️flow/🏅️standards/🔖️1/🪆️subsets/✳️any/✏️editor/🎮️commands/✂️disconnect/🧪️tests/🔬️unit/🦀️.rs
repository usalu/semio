use super::*;
use crate::editor::flow::unit_tests::context::{dispatch, flow_app, settle_refusal};
use crate::editor::flow::FlowCommand;

#[semio_framework_async_macros::async_test]
async fn disconnecting_an_unknown_synapse_is_refused_by_name() {
    let mut app = flow_app().await;
    let result = dispatch(&mut app, FlowCommand::Disconnect(Disconnect { synapse_id: "nope".into() })).await;
    assert!(result.mutations.is_empty());
    let refusal = settle_refusal(&mut app).await;
    assert!(refusal.contains("mutation.target-missing") && refusal.contains("\"nope\""), "{refusal}");
}
