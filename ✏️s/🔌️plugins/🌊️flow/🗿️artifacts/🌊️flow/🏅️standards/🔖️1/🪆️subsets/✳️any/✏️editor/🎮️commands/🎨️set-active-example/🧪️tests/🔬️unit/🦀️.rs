use super::*;
use crate::editor::flow::unit_tests::context::{dispatch, flow_app, settle};
use crate::editor::flow::FlowCommand;
use crate::examples::demo;

/// ⚖️ LAW: `setActiveExample demo` turns the live document into exactly the published demo graph —
/// widgets, synapses and layout — not merely into a different widget count (an empty demo satisfied
/// that, and painted an empty canvas in play).
#[semio_framework_async_macros::async_test]
async fn set_active_example_demo_loads_the_published_demo_graph() {
    let mut app = flow_app().await;
    let expected = <FlowSnapshot as store::ArtifactDsl>::parse_dsl(demo::PRIMARY_TEXT).expect("demo parses").to_host_snapshot();
    let before = app.snapshot().expect("snapshot").to_host_snapshot();
    assert_ne!(before, expected, "the live default document must differ from the demo so a demo load is observable");
    before.retire_cold();
    dispatch(&mut app, FlowCommand::SetActiveExample(SetActiveExample { example_id: demo::ID.into() })).await;
    settle(&mut app).await;
    let after = app.snapshot().expect("snapshot").to_host_snapshot();
    assert_eq!(after, expected, "setActiveExample demo must load the published demo graph");
    after.retire_cold();
    expected.retire_cold();
}
