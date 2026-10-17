use crate::editor::flow::base::FLOW_PLAY_WINDOW_MAIN;
use flow::FlowEvalSession;
#[allow(unused_imports)]
use crate::editor::flow::commands::flow_eval_tick::{eval_tick_effect, may_rearm};
#[allow(unused_imports)]
use crate::editor::flow::host::host_from_snapshot;
#[allow(unused_imports)]
use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
#[allow(unused_imports)]
use crate::{FlowMutation, FlowSnapshot};
#[allow(unused_imports)]
use semio_framework_plugin::NoConfig;
#[allow(unused_imports)]
use semio_framework_plugin::NoConfigMutation;
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
#[allow(unused_imports)]
use semio_framework_value_derive::{FromValue, ToValue};
use crate::editor::flow::commands::evaluate::*;
use crate::editor::flow::unit_tests::context::{dispatch, flow_app, settle};
use crate::editor::flow::FlowCommand;

#[semio_framework_async_macros::async_test]
async fn evaluate_updates_preview_state_without_operations() {
    let mut app = flow_app().await;
    let result = dispatch(&mut app, FlowCommand::Evaluate(Evaluate {})).await;
    assert!(result.mutations.is_empty(), "evaluate is a view action");
    settle(&mut app).await;
}

#[semio_framework_async_macros::async_test]
async fn resolving_a_node_output_re_arms_the_tick_chain() {
    let mut app = flow_app().await;
    let result = dispatch(&mut app, FlowCommand::FlowEvalResolve(crate::editor::flow::commands::flow_eval_resolve::FlowEvalResolve { window_id: FLOW_PLAY_WINDOW_MAIN.into(), node_hash: 42, output_json: Some(r#"{"done":true,"phase":"complete","unitsDone":1,"unitsTotal":1,"outputJson":"{}"}"#.into()) })).await;
    assert!(result.mutations.is_empty(), "resolving is not a document edit");
    settle(&mut app).await;
}

#[semio_framework_async_macros::async_test]
async fn flow_eval_session_neural_cache_is_per_instance_not_process_wide() {
    let a = FlowEvalSession::new();
    let b = FlowEvalSession::new();
    assert!(!std::sync::Arc::ptr_eq(&a.neural_cache(), &b.neural_cache()));
    a.retire_cold();
    b.retire_cold();
}
