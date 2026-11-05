//! 📤️ The mounted host boundary retains every typed effect while settling exact operation pages.
use super::context;
use semio_framework::kernel::Effect;
use semio_framework_plugin::{PluginApp, artifact_app_laws, app::TypedOperationResultLane};

pub(super) async fn collect(app:&mut context::LowpolyApp)->Vec<Effect>{
 let(effects,faults)=collect_with_faults(app).await;
 assert!(faults.is_empty(),"mounted operation faults: {faults:?}");
 effects
}

pub(super) async fn collect_with_faults(app:&mut context::LowpolyApp)->(Vec<Effect>,Vec<String>){
 let deadline=std::time::Instant::now()+std::time::Duration::from_secs(30);let mut effects=Vec::new();let mut faults=Vec::new();
 loop{
  while let Some(effect)=app.take_typed_operation_effect(){effects.push(effect);}
  while let Some(mut page)=artifact_app_laws::take_fixture_result_page(&mut app.0,context::INSTANCE,semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).unwrap(){
   if page.lane==TypedOperationResultLane::Fault{
    let framed:serde_json::Value=serde_json::from_slice(page.bytes()).expect("independent typed fault frame parser");
    faults.push(framed["code"].as_str().expect("typed fault code").to_string());
   }
   let token=artifact_app_laws::close_fixture_result_page(&mut page,semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close);
   assert!(app.acknowledge_typed_operation_result(token).unwrap());
  }
  while app.take_typed_operation_event().is_some(){}while app.take_typed_operation_ui_scope().is_some(){}
  loop{match app.take_typed_operation_completion().await{Ok(Some(_))=>{},Ok(None)=>break,Err(fault)=>{faults.push(fault.code.0);break;}}}
  while let Some(effect)=app.take_typed_operation_effect(){effects.push(effect);}
  if !app.has_pending_typed_operations(){break;}assert!(std::time::Instant::now()<deadline,"mounted operation did not settle");semio_framework_async::yield_once().await;
  app.0.maintenance_step(app.1.cx(),semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).unwrap();
  app.0.advance_typed_operation_publication(&mut artifact_app_laws::fixture_identity(),app.1.cx()).await.unwrap();
 }(effects,faults)
}
