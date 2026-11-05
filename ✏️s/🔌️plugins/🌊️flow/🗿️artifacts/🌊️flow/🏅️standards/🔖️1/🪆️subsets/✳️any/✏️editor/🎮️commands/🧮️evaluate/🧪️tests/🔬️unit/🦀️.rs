use crate::editor::flow::editor_vocabulary::FLOW_PLAY_WINDOW_MAIN;
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
use semio_plugin::NoConfig;
#[allow(unused_imports)]
use semio_plugin::NoConfigMutation;
#[allow(unused_imports)]
use semio_plugin::{ArtifactView, ConfigView, Emit, Fault};
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
async fn resolving_an_unregistered_node_preserves_its_typed_origin_refusal(){
 use semio_plugin::ArtifactApp;
 use semio_plugin::artifact_app_laws::{dispatch_original_fixture_command,settle_registered_typed_operation,close_registered_fixture_app,fixture_identity,meta,OriginalFixtureCaller};
 use semio_framework_value::retirement::controlled::ControlledRetirement;
 use semio_framework_job::close_original_retained_step;
 use semio_framework_value::RetainedWorkOutcome;
 let mut app=flow_app().await;let command=FlowCommand::FlowEvalResolve(crate::editor::flow::commands::flow_eval_resolve::FlowEvalResolve{window_id:FLOW_PLAY_WINDOW_MAIN.into(),node_hash:42,output:Some(semio_framework_value::DslValue::Object(Vec::new()))});
 let metadata=crate::editor::flow::unit_tests::context::flow_main_window_meta();let policy=semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
 let dispatch=dispatch_original_fixture_command(&mut app.0,&command,&metadata,policy,&mut fixture_identity(),app.1.cx()).await;let dispatched=dispatch.is_ok();let mut refusal=dispatch.err().map(semio_plugin::PluginLifecycleFailure::from);
 if dispatched{refusal=settle_registered_typed_operation(&mut app.0,meta("local").instance_id,policy,&mut fixture_identity(),app.1.cx()).await.err();}
 let exact=refusal.as_ref().is_some_and(|failure|match &failure.primary{semio_plugin::PluginLifecyclePrimary::Value(error)=>error.message=="original Flow reply has no matching invocation origin",_=>false});
 let mut close=ControlledRetirement::new((command,refusal)).unwrap_or_else(|_|unreachable!("the original reply and typed refusal have retained retirement"));
 for _ in 0..OriginalFixtureCaller::maximum_turns(){if close.terminal_is_empty(){break}let cx=app.1.cx();let d=close.selected_demand(cx.retained_grant().maximum_copy_bytes).unwrap();let deadline=cx.original_absolute_deadline_us();cx.with_original_retirement(|cx|cx.with_original_work_loan(1,deadline,|cx|{close_original_retained_step(cx,d,|g|{let result=close.step(g);RetainedWorkOutcome{result,work:close.last_turn_work()}})?;Ok(())})).unwrap();}
 let retired=close.terminal_is_empty();close_registered_fixture_app(&mut app.0,policy,app.1.cx()).unwrap();assert!(dispatched&&exact&&retired&&app.0.close_terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn flow_eval_session_neural_cache_is_per_instance_not_process_wide() {
    let a = FlowEvalSession::new();
    let b = FlowEvalSession::new();
    assert!(!std::sync::Arc::ptr_eq(&a.neural_cache(), &b.neural_cache()));
    a.retire_cold();
    b.retire_cold();
}
