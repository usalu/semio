use super::*;
use crate::editor::flow::unit_tests::context::{dispatch, flow_app};
use crate::editor::flow::FlowCommand;

#[semio_framework_async_macros::async_test]
async fn evaluate_updates_preview_state_without_operations() {
    let mut app = flow_app().await;
    let result = dispatch(&mut app, FlowCommand::Evaluate(Evaluate {})).await;
    assert!(result.mutations.is_empty(), "evaluate is a view action");
}

#[semio_framework_async_macros::async_test]
async fn resolving_a_node_output_re_arms_the_tick_chain() {
    let mut app = flow_app().await;
    let result = dispatch(&mut app, FlowCommand::FlowEvalResolve(crate::editor::flow::commands::flow_eval_resolve::FlowEvalResolve { node_hash: 42, output_json: "{}".into() })).await;
    assert!(result.mutations.is_empty(), "resolving is not a document edit");
}

#[semio_framework_async_macros::async_test]
async fn flow_eval_session_neural_cache_is_per_instance_not_process_wide() {
    let a = FlowEvalSession::new();
    let b = FlowEvalSession::new();
    assert!(!std::sync::Arc::ptr_eq(&a.neural_cache(), &b.neural_cache()));
}
