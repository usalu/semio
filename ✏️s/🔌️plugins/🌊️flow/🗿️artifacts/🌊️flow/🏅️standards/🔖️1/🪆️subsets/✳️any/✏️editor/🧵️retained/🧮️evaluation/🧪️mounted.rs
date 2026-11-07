//! 🧪️ Real mounted Flow commands reach their composed child and preserve the original typed refusal.
use super::*;
use semio_framework_job::close_original_retained_step;
use semio_framework_value::RetainedWorkOutcome;
use semio_plugin::{ArtifactApp,AppActionRegistry,EditorApp,PluginApp,VcsArtifactApp,MountedOwnerPolicyV1,ViewModel,ViewWindowInstance,PluginLifecyclePrimary};
use semio_plugin::artifact_app_laws::{OriginalFixtureScope,fixture_identity,meta,settle_store_constructor,dispatch_original_fixture_command,settle_registered_typed_operation,close_registered_fixture_app,OriginalFixtureCaller};
use semio_s_artifact_stdio_semio::SemioMembers;
use semio_framework_job::{OriginalDomainView,OriginalFailureContext};
use semio_framework_value::retirement::controlled::ControlledRetirement;
static SERIAL:std::sync::Mutex<()>=std::sync::Mutex::new(());
#[semio_framework_async_macros::async_test]
async fn original_registered_flow_evaluate_and_tick_keep_actual_composed_source_and_typed_refusal(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../📥️original/🔣️donation.json")).unwrap();let expected=&fixture["uncontributedEvaluation"];
 let _serial=SERIAL.lock().unwrap_or_else(|poisoned|poisoned.into_inner());
 for tick in [false,true]{
  let mut scope=OriginalFixtureScope::admit();let definition=crate::editor::flow::create_flow_app();let registry=AppActionRegistry::from_definition(&definition);
  let mut app=Box::new(VcsArtifactApp::<EditorApp<FlowPlayApp>,SemioMembers>::with_registry(EditorApp::default(),registry,semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()),MountedOwnerPolicyV1::INTERACTIVE_COMPONENT,&mut fixture_identity()).await.expect("the actual Flow reference and composition are born"));
  let mut metadata=meta("local");let window=crate::editor::flow::editor_vocabulary::FLOW_PLAY_WINDOW_MAIN;metadata.view_state=Some(ViewModel{window_id:Some(window.into()),active_window_kind_id:Some(window.into()),window_instances:vec![ViewWindowInstance{id:window.into(),window_kind_id:window.into()}],..ViewModel::new(semio_ui_locale::Locale::En,semio_ui_locale::Terminology::Native)});
  app.bind_instance_id(metadata.instance_id).await;settle_store_constructor(&mut *app,MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);
  let composed={let snapshot=app.snapshot().expect("original parent snapshot");app.child_store("content",&snapshot.content.child_id).await.is_some()};
  let command=if tick{FlowCommand::FlowEvalTick(crate::editor::flow::flow_eval_tick::FlowEvalTick{window_id:window.into(),window_kind_id:window.into()})}else{FlowCommand::Evaluate(crate::editor::flow::evaluate::Evaluate{})};
  let dispatch=dispatch_original_fixture_command(&mut *app,&command,&metadata,MountedOwnerPolicyV1::INTERACTIVE_COMPONENT,&mut fixture_identity(),scope.cx()).await;
  let dispatch_ok=dispatch.is_ok();let mut failure=dispatch.err().map(semio_plugin::PluginLifecycleFailure::from);let mut exact_domain=false;let mut exact_variant=false;let mut exact_word=false;let mut receipt_ok=false;
  if dispatch_ok{match settle_registered_typed_operation(&mut *app,metadata.instance_id,MountedOwnerPolicyV1::INTERACTIVE_COMPONENT,&mut fixture_identity(),scope.cx()).await{
   Ok(receipt)=>drop(receipt),
   Err(original)=>{receipt_ok=original.receipt_refusal.is_none()&&original.context_refusal.is_none();if let PluginLifecyclePrimary::Domain(domain)=&original.primary{exact_domain=Some(domain.code())==expected["code"].as_str();exact_variant=match domain.projection().projection_view(&[0,0]){Ok(OriginalDomainView::IntrinsicText(value))=>Some(value)==expected["variant"].as_str(),_=>false};exact_word=match domain.projection().projection_view(&[0,1]){Ok(OriginalDomainView::IntrinsicText(value))=>Some(value)==expected["requestedOperator"].as_str(),_=>false};}failure=Some(original)}
  }}
  let mut original=ControlledRetirement::new((command,failure)).unwrap_or_else(|_|unreachable!("actual command and failure define retained closure"));for _ in 0..OriginalFixtureCaller::maximum_turns(){if original.terminal_is_empty(){break}let cx=scope.cx();let d=original.selected_demand(cx.retained_grant().maximum_copy_bytes).expect("original diagnostic closure quote");let deadline=cx.original_absolute_deadline_us();cx.with_original_retirement(|cx|cx.with_original_work_loan(1,deadline,|cx|{close_original_retained_step(cx,d,|g|{let result=original.step(g);RetainedWorkOutcome{result,work:original.last_turn_work()}})?;Ok(())})).expect("same original caller closes the actual diagnostic");}
  let diagnostic_closed=original.terminal_is_empty();close_registered_fixture_app(&mut *app,MountedOwnerPolicyV1::INTERACTIVE_COMPONENT,scope.cx()).expect("same original caller closes the actual mounted child family");let app_closed=app.close_terminal_is_empty();
  assert!(composed&&dispatch_ok&&diagnostic_closed&&app_closed);assert!(exact_domain&&exact_variant&&exact_word&&receipt_ok,"the bare production registry retains the original unknown math.add domain refusal");
 }
}
