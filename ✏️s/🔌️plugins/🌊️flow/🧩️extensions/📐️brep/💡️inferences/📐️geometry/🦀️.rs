//! 📐️ Geometry computation owned by the named flow artifact inference provider.

use super::{module_registry, SessionCapture};
use semio_framework_plugin::{ArtifactInferenceExecution, ArtifactInferenceExecutionStep, ArtifactInferenceExecutionError, ArtifactInferenceExecutionRequest, ArtifactInferencePayloadContract, ArtifactInferenceService, ArtifactInferenceServiceMetadata, WireArtifactInferenceCacheMode};

pub const GEOMETRY_INFERENCE_SCHEMA: &str = "s.flow-extension-brep.geometry";
pub const GEOMETRY_ARTIFACT_KIND: &str = "s.flow.flow";

/// 🎟️ Keeps one original geometry execution source and its exact registry/session close authority.
pub struct GeometryInferenceContext {session:SessionCapture,evaluation:std::sync::Mutex<flow_extension_sdk::ExtensionEvaluationResources>,normal:std::sync::Mutex<Option<GeometryInferenceTurn>>,tessellation:super::tessellation::TessellationResources}
impl GeometryInferenceContext {
    pub fn new(session:SessionCapture)->Self {let evaluation=flow_extension_sdk::ExtensionEvaluationResources::new(module_registry(&session));Self {session,evaluation:std::sync::Mutex::new(evaluation),normal:std::sync::Mutex::new(None),tessellation:super::tessellation::TessellationResources::new()}}
    pub(crate) fn evaluation(&self)->std::sync::MutexGuard<'_,flow_extension_sdk::ExtensionEvaluationResources> {self.evaluation.lock().unwrap_or_else(std::sync::PoisonError::into_inner)}
    pub fn session(&self)->&SessionCapture {&self.session}
    pub fn evaluate_raw(&self,source:&[u8],cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_plugin::ExtensionInvokeStep,semio_framework::Fault>{flow_extension_sdk::evaluate_invoke_json(&mut self.evaluation(),source,cx)}
    pub fn tessellate_raw(&self,source:&[u8],cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_plugin::ExtensionInvokeStep,semio_framework::Fault>{self.tessellation.invoke(&self.session,source,cx)}
    pub fn retirement_demands(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError> {
        use semio_framework_plugin::ExtensionResourceOwner;
        if !self.tessellation.terminal_is_empty(){return self.tessellation.retirement_demands(copy);}
        let evaluation=self.evaluation();
        if let Some(owner)=self.normal.lock().expect("original geometry cursor").as_ref(){return owner.retirement_demands(evaluation.registry(),copy);}
        if !evaluation.terminal_is_empty(){return evaluation.retirement_demands(copy);}
        Ok(semio_framework_value::RetirementDemand {copy_bytes:self.session.next_close_copy_byte_demand()?,capacity_bytes:self.session.next_close_capacity_byte_demand(copy)?,release_bytes:self.session.next_close_release_byte_demand()?,depth:self.session.next_close_depth_demand()?})
    }
    pub fn begin_close(&mut self) {self.tessellation.begin_close();if let Some(owner)=self.normal.get_mut().expect("exclusive geometry cursor").as_mut(){owner.begin_close();}semio_framework_plugin::ExtensionResourceOwner::begin_close(self.evaluation.get_mut().unwrap_or_else(std::sync::PoisonError::into_inner));}
    pub fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_plugin::PluginLifecycleStep,semio_framework_job::OriginalTurnFailure>{
        use semio_framework_plugin::{ExtensionResourceOwner,PluginLifecycleStep};
        if !self.tessellation.terminal_is_empty(){let step=self.tessellation.close_step(cx)?;return Ok(PluginLifecycleStep::Progress(step.progress()));}
        let evaluation=self.evaluation.get_mut().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(owner)=self.normal.get_mut().expect("exclusive geometry cursor").as_mut(){let step=owner.close_step(evaluation.registry(),cx)?;if owner.terminal_is_empty(){*self.normal.get_mut().expect("exclusive geometry cursor")=None;}return Ok(PluginLifecycleStep::Progress(step.progress()));}
        if !evaluation.terminal_is_empty(){return evaluation.close_step(cx).map(|step|match step{PluginLifecycleStep::Complete(progress)=>PluginLifecycleStep::Progress(progress),step=>step});}
        let grant=cx.retained_grant();let demand=semio_framework_value::RetirementDemand{copy_bytes:self.session.next_close_copy_byte_demand()?,capacity_bytes:self.session.next_close_capacity_byte_demand(grant.maximum_copy_bytes)?,release_bytes:self.session.next_close_release_byte_demand()?,depth:self.session.next_close_depth_demand()?};
        let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|self.session.close_step(grant).and_then(|step|semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,self.session.terminal_is_empty(),"original geometry inference session")))?;
        Ok(PluginLifecycleStep::retained(semio_framework_value::retained_clone::RetainedCloneStep::Progress(step.progress()),self.terminal_is_empty()))
    }
    pub fn terminal_is_empty(&self)->bool {let evaluation=self.evaluation();self.tessellation.terminal_is_empty()&&self.normal.lock().is_ok_and(|owner|owner.is_none())&&semio_framework_plugin::ExtensionResourceOwner::terminal_is_empty(&*evaluation)&&self.session.terminal_is_empty()}
}

use semio_framework_value::retirement::RetireOwned;
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,ErasedSnapshotRetirement,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use semio_framework_job::{OriginalDomainProjection,OriginalDomainFailure,OriginalDomainView as GeometryView};

#[derive(Debug,semio_framework_value::RetireOwned)]
struct GeometryInferenceFailure{primary:flow_extension_sdk::EvaluationFailure,receipt_refusal:Option<ValueError>}
impl OriginalDomainProjection for GeometryInferenceFailure{
 fn projection_identity(&self)->usize{self.primary.projection_identity()}
 fn projection_view(&self,path:&[usize])->Result<GeometryView<'_>,ValueError>{match path{[]=>Ok(GeometryView::IntrinsicObject(1+usize::from(self.receipt_refusal.is_some()))),[0,rest @ ..]=>self.primary.projection_view(rest),[1,rest @ ..]=>self.receipt_refusal.as_ref().ok_or_else(geometry_projection_refusal).and_then(|original|semio_framework_job::project_original_value_error(original,rest)),_=>Err(geometry_projection_refusal())}}
 fn projection_key(&self,path:&[usize],ordinal:usize)->Result<&str,ValueError>{match path{[]=>["primary","receiptRefusal"].get(ordinal).copied().filter(|_|ordinal==0||self.receipt_refusal.is_some()).ok_or_else(geometry_projection_refusal),[0,rest @ ..]=>self.primary.projection_key(rest,ordinal),[1,rest @ ..]=>self.receipt_refusal.as_ref().ok_or_else(geometry_projection_refusal).and_then(|original|semio_framework_job::key_original_value_error(original,rest,ordinal)),_=>Err(geometry_projection_refusal())}}
}
impl OriginalDomainFailure for GeometryInferenceFailure{
 fn domain_code(&self)->&str{self.primary.domain_code()}
 fn domain_message(&self)->&str{self.primary.domain_message()}
 fn domain_retained_progress(&self)->RetainedCloneProgress{self.primary.retained_progress()}
}
fn geometry_projection_refusal()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original geometry inference failure ordinal changed")}
fn geometry_admit_failure(source:&mut Option<GeometryInferenceFailure>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::OriginalTurnFailure>,semio_framework_job::OriginalTurnFailure>{
 if source.as_ref().is_some_and(|failure|matches!(failure.primary,flow_extension_sdk::EvaluationFailure::Original(_))&&failure.receipt_refusal.is_none()){
  let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}cx.consume_retained(geometry_item())?;cx.consume_fuel(1);let original=source.take().unwrap();if let flow_extension_sdk::EvaluationFailure::Original(original)=original.primary{return Ok(Some(original))}unreachable!()
 }
 semio_framework_job::admit_original_domain_failure(source,cx)
}
fn geometry_delta(before:RetainedCloneProgress,after:RetainedCloneProgress)->RetainedCloneProgress{RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes}}

struct GeometryInferenceTurn{operation:u64,generation:u64,pointer:usize,length:usize,cursor:Option<flow_extension_sdk::EvaluationInvokeCursor>,wire:Option<Vec<u8>>,discarded_execution:Option<ArtifactInferenceExecution>,failure:Option<GeometryInferenceFailure>,normal_posted:bool,quality:Option<&'static str>,faulted:bool,complete:bool,metadata:[Option<Vec<u8>>;2],field:usize,position:usize,active:Option<Box<dyn ErasedSnapshotRetirement>>,closing:bool}
impl GeometryInferenceTurn{
 fn new(request:&ArtifactInferenceExecutionRequest<'_>)->Self{Self{operation:request.operation,generation:request.generation,pointer:request.canonical_payload.as_ptr()as usize,length:request.canonical_payload.len(),cursor:Some(flow_extension_sdk::EvaluationInvokeCursor::new(request.operation,request.generation,request.canonical_payload)),wire:None,discarded_execution:None,failure:None,normal_posted:false,quality:None,faulted:false,complete:false,metadata:[None,None],field:0,position:0,active:None,closing:false}}
 fn matches(&self,request:&ArtifactInferenceExecutionRequest<'_>)->bool{self.operation==request.operation&&self.generation==request.generation&&self.pointer==request.canonical_payload.as_ptr()as usize&&self.length==request.canonical_payload.len()}
 fn begin_close(&mut self){self.closing=true;if let Some(cursor)=&mut self.cursor{cursor.begin_close();}}
 fn terminal_is_empty(&self)->bool{self.cursor.is_none()&&self.wire.is_none()&&self.discarded_execution.is_none()&&self.failure.is_none()&&self.metadata.iter().all(Option::is_none)&&self.active.is_none()}
 fn retirement_demands(&self,registry:Option<&neural_engine::SharedRegistry>,copy:usize)->Result<RetirementDemand,ValueError>{if self.failure.is_some(){return Ok(semio_framework_job::original_domain_failure_demand::<GeometryInferenceFailure>());}if let Some(cursor)=&self.cursor{return match registry{Some(registry)=>cursor.retirement_demands_with_registry(registry,copy),None=>cursor.retirement_demands(copy)}}if let Some(active)=&self.active{return semio_framework_value::factory_ticket_demands(active,copy)}if self.discarded_execution.is_some(){return Ok(geometry_birth::<ArtifactInferenceExecution>())}if self.wire.is_some(){return Ok(geometry_birth::<Vec<u8>>())}if self.metadata.iter().any(Option::is_some){return Ok(geometry_birth::<Vec<u8>>())}Ok(Default::default())}
 fn close_raw(&mut self,registry:Option<&neural_engine::SharedRegistry>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,flow_extension_sdk::EvaluationFailure>{if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}if let Some(cursor)=&mut self.cursor{let step=match registry{Some(registry)=>cursor.close_step_with_registry(registry,grant)?,None=>cursor.close_step(grant)?};if cursor.terminal_is_empty(){self.cursor=None;}return Ok(RetainedCloneStep::Progress(step.progress()))}if self.active.is_some(){return semio_framework_value::close_factory_ticket(&mut self.active,grant).map_err(Into::into)}if self.discarded_execution.is_some(){return geometry_admit(&mut self.discarded_execution,&mut self.active,grant).map_err(Into::into)}if self.wire.is_some(){return geometry_admit(&mut self.wire,&mut self.active,grant).map_err(Into::into)}if let Some(source)=self.metadata.iter_mut().find(|source|source.is_some()){return geometry_admit(source,&mut self.active,grant).map_err(Into::into)}Ok(RetainedCloneStep::Complete(Default::default()))}
 fn close_step(&mut self,registry:Option<&neural_engine::SharedRegistry>,cx:&mut semio_framework_job::StepContext<'_>)->Result<RetainedCloneStep,semio_framework_job::OriginalTurnFailure>{
  if self.failure.is_some(){return match geometry_admit_failure(&mut self.failure,cx)?{Some(original)=>Err(original),None=>Ok(RetainedCloneStep::Progress(Default::default()))}}
  let available=cx.retained_grant();let demand=self.retirement_demands(registry,available.maximum_copy_bytes)?;if cx.should_yield()||available.maximum_items==0||available.maximum_copy_bytes<demand.copy_bytes||available.maximum_capacity_bytes<demand.capacity_bytes||available.maximum_release_bytes<demand.release_bytes||available.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()))}cx.consume_fuel(1);
  let result=self.close_raw(registry,RetainedCloneGrant{maximum_items:1,..available});let receipt=match &result{Ok(step)=>step.progress(),Err(primary)=>primary.retained_progress()};let receipt_refusal=cx.consume_retained(receipt).err();match result{Ok(step)=>{if let Some(original)=receipt_refusal{self.failure=Some(GeometryInferenceFailure{primary:original.into(),receipt_refusal:None});}Ok(step)},Err(primary)=>{self.failure=Some(GeometryInferenceFailure{primary,receipt_refusal});Ok(RetainedCloneStep::Progress(receipt))}}
 }
 fn step(&mut self,registry:Option<&neural_engine::SharedRegistry>,request:&ArtifactInferenceExecutionRequest<'_>,cx:&mut semio_framework_plugin::ArtifactInferenceStepContext<'_>)->Result<ArtifactInferenceExecutionStep,ArtifactInferenceExecutionError>{
  self.normal_posted=false;let before=cx.retained_progress();let mut result=self.step_original(registry,request,cx);if !self.normal_posted{let receipt=match &result{Ok(step)=>step.retained_progress,Err(original)=>original.retained_progress};cx.consume_fuel(receipt.copied_items as u64);if let Err(refusal)=cx.consume_retained(receipt){if let Some(failure)=&mut self.failure{failure.receipt_refusal=Some(refusal)}else{self.failure=Some(GeometryInferenceFailure{primary:refusal.into(),receipt_refusal:None});self.begin_close();}if let Ok(step)=&mut result{self.discarded_execution=step.execution.take();step.terminal=false;}}}
  let receipt=geometry_delta(before,cx.retained_progress());match &mut result{Ok(step)=>step.retained_progress=receipt,Err(original)=>original.retained_progress=receipt}result
 }
 fn step_original(&mut self,registry:Option<&neural_engine::SharedRegistry>,request:&ArtifactInferenceExecutionRequest<'_>, cx:&mut semio_framework_plugin::ArtifactInferenceStepContext<'_>)->Result<ArtifactInferenceExecutionStep,ArtifactInferenceExecutionError>{
  let available=cx.retained_grant();let grant=RetainedCloneGrant{maximum_items:available.maximum_items.min(usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX)),..available};let pending=|retained_progress|ArtifactInferenceExecutionStep{execution:None,retained_progress,terminal:false};
  if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(pending(Default::default()))}
  if self.failure.is_some(){self.normal_posted=true;let admitted=geometry_admit_failure(&mut self.failure,cx).map_err(|error|ArtifactInferenceExecutionError::original(error,false))?;return match admitted{Some(error)=>Err(ArtifactInferenceExecutionError::original(error,false)),None=>Ok(pending(Default::default()))}}
  if !self.matches(request){return Err(ArtifactInferenceExecutionError::native(ValueError::literal(ValueRefusalKind::OwnershipLimit,"geometry inference retains another original caller"),false))}
  if request.cancelled||cx.is_cancelled(){self.begin_close();}if self.closing{self.normal_posted=true;let step=self.close_step(registry,cx).map_err(|error|ArtifactInferenceExecutionError::original(error,false))?;return Ok(ArtifactInferenceExecutionStep{execution:None,retained_progress:step.progress(),terminal:self.terminal_is_empty()})}
  let Some(registry)=registry else{return Ok(pending(Default::default()))};
  if let Some(cursor)=&mut self.cursor{
   if !cursor.matches(request.operation,request.generation,request.canonical_payload){return Err(ArtifactInferenceExecutionError::native(ValueError::literal(ValueRefusalKind::OwnershipLimit,"geometry inference retains another original caller"),false))}
   if self.quality.is_none(){if let Some(original)=cursor.original_request(){if !original.operator_id.is_empty(){self.quality=geometry_operator_quality_ref(registry,&original.operator_id);}}}
   if cursor.terminal_is_empty()&&self.wire.is_some(){self.cursor=None;return Ok(pending(geometry_item()))}
   let input=flow_extension_sdk::EvaluationRequestContextInput{cancellation_id:request.cancellation_id,policy:request.policy,dependencies:request.dependencies,work_units:request.budgets.work_units,wait_terminal:true};
   let result=cursor.step_with_context(registry,request.canonical_payload,cx,input);self.normal_posted=cursor.normal_context_posted();let receipt=cursor.normal_step_progress();
   let reply=match result{Ok((reply,_))=>reply,Err(primary)=>{cursor.begin_close();self.closing=true;self.failure=Some(GeometryInferenceFailure{primary,receipt_refusal:None});return Ok(pending(receipt))}};
   if let Some(reply)=reply{assert!(reply.returned_request.is_none(),"original callback keeps denied source in its cursor");self.faulted=reply.faulted;self.complete=reply.complete;self.wire=reply.wire;if let Some(primary)=reply.refusal{cursor.begin_close();self.closing=true;self.failure=Some(GeometryInferenceFailure{primary,receipt_refusal:None});}}
   return Ok(pending(receipt));
  }
  if self.field<2{
   let text=if self.field==0{if self.faulted{"invalid"}else{"valid"}}else{match self.quality{Some(quality)=>quality,None if self.faulted=>"InvalidRequest",None=>{self.closing=true;return Err(ArtifactInferenceExecutionError::native(ValueError::literal(ValueRefusalKind::InvariantViolated,"completed geometry has no declared operator quality"),false))}}};
   if self.metadata[self.field].is_none(){if grant.maximum_capacity_bytes<text.len(){return Ok(pending(Default::default()))}let mut destination=Vec::new();destination.try_reserve_exact(text.len()).map_err(|_|ArtifactInferenceExecutionError::native(ValueError::literal(ValueRefusalKind::AllocationFailed,"original geometry metadata allocation failed"),false))?;let capacity=destination.capacity();self.metadata[self.field]=Some(destination);return Ok(pending(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:capacity,..Default::default()}))}
   let count=grant.maximum_copy_bytes.min(text.len().saturating_sub(self.position));if count==0&&self.position!=text.len(){return Ok(pending(Default::default()))}self.metadata[self.field].as_mut().unwrap().extend_from_slice(&text.as_bytes()[self.position..self.position+count]);self.position+=count;if self.position==text.len(){self.field+=1;self.position=0;}return Ok(pending(RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()}))
  }
  let validity=String::from_utf8(self.metadata[0].take().unwrap()).expect("declared validity is UTF8");let quality=String::from_utf8(self.metadata[1].take().unwrap()).expect("declared geometry quality is UTF8");let receipt=geometry_item();Ok(ArtifactInferenceExecution{canonical_payload:self.wire.take(),retirement_progress:receipt,diagnostics:Vec::new(),validity,quality,complete:self.complete,actual_cache_mode:request.requested_cache_mode.clone()}.into_step(true))
 }
}
fn geometry_item()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
fn geometry_birth<T:RetireOwned>()->RetirementDemand{RetirementDemand{capacity_bytes:semio_framework_value::retirement::owned_retirement_birth_bytes::<T>(),depth:1,..Default::default()}}
fn geometry_admit<T:RetireOwned>(source:&mut Option<T>,active:&mut Option<Box<dyn ErasedSnapshotRetirement>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if grant.maximum_capacity_bytes<geometry_birth::<T>().capacity_bytes{return Ok(RetainedCloneStep::Progress(Default::default()))}if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"geometry original handoff requires depth"))}let original=source.take().unwrap();match semio_framework_value::retirement::admit_owned_retirement(original,grant){Ok((owner,receipt))=>{*active=Some(owner);Ok(RetainedCloneStep::Progress(receipt))},Err((error,original))=>{*source=Some(original);Err(error)}}}
fn geometry_operator_quality_ref(registry:&neural_engine::Registry,operator:&str)->Option<&'static str>{
 let info=registry.operator_info(operator)?;if let Some(operation)=operator.strip_prefix("brep.mesh."){return Some(super::mesh::operation_quality(operation))}
 if !super::NODE_KERNEL_METHOD.iter().any(|(id,_)|*id==operator)&&info.group.iter().any(|group|group=="Schemas")&&operator.rsplit('.').next().is_some_and(|schema|registry.schema(schema).is_some()){return Some("SchemaValue")}
 let (_,method)=super::NODE_KERNEL_METHOD.iter().find(|(id,_)|*id==operator)?;use semio_framework_3d_brep_representation::contract::OpQuality;Some(match super::operation_quality(method){OpQuality::ExactAnalytic=>"ExactAnalytic",OpQuality::ExactNumericalWithinTolerance=>"ExactNumericalWithinTolerance",OpQuality::ApproximateBRep=>"ApproximateBRep",OpQuality::MeshDerivedBRep=>"MeshDerivedBRep",OpQuality::PreviewOnly=>"PreviewOnly",OpQuality::Unsupported=>"Unsupported"})
}

pub const GEOMETRY_INFERENCE_CONTRACT: ArtifactInferencePayloadContract = ArtifactInferencePayloadContract {
    payload_schema_id: "s.flow-extension-brep.geometry.payload",
    input_schema: include_str!("📥️request.json"),
    output_schema: include_str!("📤️result.json"),
    progress_unit: "operator-step",
    artifact_binding: None,
    commit: None,
};

/// 🪪️ Registers the computation owner without capturing or duplicating instance resources.
pub const fn geometry_inference_service() -> ArtifactInferenceService {
    ArtifactInferenceService::new_contextual(ArtifactInferenceServiceMetadata {
        owner: "flow-extension-brep",
        artifact_kind: GEOMETRY_ARTIFACT_KIND,
        artifact_schema: "s.flow.flow",
        artifact_schema_version: 1,
        inference_schema: GEOMETRY_INFERENCE_SCHEMA,
        inference_schema_version: 1,
        algorithm_version: 1,
        policy_version: 1,
        payload: Some(GEOMETRY_INFERENCE_CONTRACT),
    }, infer_geometry,geometry_inference_demands)
}

#[cfg(feature = "testkit")]
pub static GEOMETRY_INFERENCE_CALLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// 🎯️ Shares each registered operator's owned fidelity contract with named inference results.
pub(crate) fn geometry_operator_quality(registry: &neural_engine::Registry, operator: &str) -> String {
    let Some(info) = registry.operator_info(operator) else { return "Unsupported".into(); };
    if let Some(operation) = operator.strip_prefix("brep.mesh.") { return super::mesh::operation_quality(operation).into(); }
    if !super::NODE_KERNEL_METHOD.iter().any(|(id, _)| *id == operator) && info.group.iter().any(|group| group == "Schemas") && operator.rsplit('.').next().is_some_and(|schema| registry.schema(schema).is_some()) { return "SchemaValue".into(); }
    super::NODE_KERNEL_METHOD.iter().find(|(id, _)| *id == operator).map(|(_, method)| format!("{:?}", super::operation_quality(method))).unwrap_or_else(|| "Unsupported".into())
}

fn infer_geometry(request:&ArtifactInferenceExecutionRequest<'_>,context:&dyn std::any::Any, cx:&mut semio_framework_plugin::ArtifactInferenceStepContext<'_>)->Result<ArtifactInferenceExecutionStep,ArtifactInferenceExecutionError>{
 let context=context.downcast_ref::<GeometryInferenceContext>().ok_or_else(||ArtifactInferenceExecutionError::native(ValueError::literal(ValueRefusalKind::InvalidValue,"geometry inference requires its original execution owner"),true))?;
 if request.canonical_payload.len()as u64>request.budgets.allocation_bytes{return Err(ArtifactInferenceExecutionError::native(ValueError::literal(ValueRefusalKind::OwnershipLimit,"geometry source exceeds original allocation authority"),true))}
 let mut evaluation=context.evaluation();let mut owner=context.normal.lock().expect("original geometry cursor");if owner.is_none(){if cx.should_yield()||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(ArtifactInferenceExecutionStep{execution:None,retained_progress:Default::default(),terminal:false})}if (request.operation,request.generation)!=(cx.operation().0,cx.generation().0){return Err(ArtifactInferenceExecutionError::native(ValueError::literal(ValueRefusalKind::OwnershipLimit,"geometry inference requires its actual initiating context"),true))}cx.original_source_parent().map_err(|error|ArtifactInferenceExecutionError::original(error.into(),true))?;cx.consume_retained(geometry_item()).map_err(|error|ArtifactInferenceExecutionError::original(error.into(),true))?;cx.consume_fuel(1);*owner=Some(GeometryInferenceTurn::new(request));return Ok(ArtifactInferenceExecutionStep{execution:None,retained_progress:geometry_item(),terminal:false})}
 #[cfg(feature = "testkit")] GEOMETRY_INFERENCE_CALLS.fetch_add(1,std::sync::atomic::Ordering::SeqCst);
 if !owner.as_ref().unwrap().matches(request){return Err(ArtifactInferenceExecutionError::native(ValueError::literal(ValueRefusalKind::OwnershipLimit,"geometry registry admission retains another initiating source"),false))}
 if evaluation.registry().is_none()&&!owner.as_ref().unwrap().closing&&!request.cancelled&&!cx.is_cancelled(){let before=cx.retained_progress();match evaluation.admit_registry_on_original(cx){Ok(_)=>{},Err(original)=>{let owner=owner.as_mut().unwrap();owner.failure=Some(GeometryInferenceFailure{primary:original.into(),receipt_refusal:None});owner.begin_close();}}return Ok(ArtifactInferenceExecutionStep{execution:None,retained_progress:geometry_delta(before,cx.retained_progress()),terminal:false})}
 let result=owner.as_mut().unwrap().step(evaluation.registry(),request,cx)?;if result.terminal{assert!(owner.as_ref().unwrap().terminal_is_empty());*owner=None;}Ok(result)
}

/// ♻️ Borrows the actual selected child only under its original callback source.
fn geometry_inference_demands(request:&ArtifactInferenceExecutionRequest<'_>,context:&dyn std::any::Any,copy:usize)->Result<RetirementDemand,ValueError>{
 let context=context.downcast_ref::<GeometryInferenceContext>().ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"geometry demand requires its original execution owner"))?;
 let evaluation=context.evaluation();let owner=context.normal.lock().map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original geometry demand owner is busy"))?;
 let owner=owner.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"live original geometry demand has no retained turn"))?;
 if !owner.matches(request){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"geometry demand retains another original caller"))}
 owner.retirement_demands(evaluation.registry(),copy)
}
