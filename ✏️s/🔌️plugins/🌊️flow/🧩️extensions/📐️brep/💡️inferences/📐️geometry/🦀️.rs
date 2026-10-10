//! 📐️ Geometry computation owned by the named flow artifact inference provider.

use super::{module_registry, SessionCapture};
use semio_framework_plugin::{ArtifactInferenceExecution, ArtifactInferenceExecutionStep, ArtifactInferenceExecutionError, ArtifactInferenceExecutionRequest, ArtifactInferencePayloadContract, ArtifactInferenceService, ArtifactInferenceServiceMetadata, WireArtifactInferenceCacheMode};

pub const GEOMETRY_INFERENCE_SCHEMA: &str = "s.flow-extension-brep.geometry";
pub const GEOMETRY_ARTIFACT_KIND: &str = "s.flow.flow";

/// 🎟️ Keeps one original geometry execution source and its exact registry/session close authority.
pub struct GeometryInferenceContext {session:SessionCapture,evaluation:flow_extension_sdk::ExtensionEvaluationResources,normal:std::sync::Mutex<Option<GeometryInferenceTurn>>}
impl GeometryInferenceContext {
    pub fn new(session:SessionCapture)->Self {let evaluation=flow_extension_sdk::ExtensionEvaluationResources::new(module_registry(&session));Self {session,evaluation,normal:std::sync::Mutex::new(None)}}
    pub fn registry(&self)->&neural_engine::SharedRegistry {self.evaluation.registry()}
    pub fn session(&self)->&SessionCapture {&self.session}
    pub fn evaluate_raw(&self,source:&[u8],cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_plugin::ExtensionInvokeStep,semio_framework::Fault>{flow_extension_sdk::evaluate_invoke_json(&self.evaluation,source,cx)}
    pub fn retirement_demands(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError> {
        use semio_framework_plugin::ExtensionResourceOwner;
        if let Some(owner)=self.normal.lock().expect("original geometry cursor").as_ref(){return owner.retirement_demands(self.evaluation.registry(),copy);}
        if !self.evaluation.terminal_is_empty(){return self.evaluation.retirement_demands(copy);}
        Ok(semio_framework_value::RetirementDemand {copy_bytes:self.session.next_close_copy_byte_demand()?,capacity_bytes:self.session.next_close_capacity_byte_demand(copy)?,release_bytes:self.session.next_close_release_byte_demand()?,depth:self.session.next_close_depth_demand()?})
    }
    pub fn begin_close(&mut self) {if let Some(owner)=self.normal.get_mut().expect("exclusive geometry cursor").as_mut(){owner.begin_close();}semio_framework_plugin::ExtensionResourceOwner::begin_close(&mut self.evaluation);}
    pub fn close_step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_plugin::PluginLifecycleStep,semio_framework::Fault> {
        use semio_framework_plugin::{ExtensionResourceOwner,PluginLifecycleStep};
        if let Some(owner)=self.normal.get_mut().expect("exclusive geometry cursor").as_mut(){let step=owner.close_step(self.evaluation.registry(),grant).map_err(|error|semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin,semio_framework::FaultCode::new("geometry-inference.close"),error.into_message()))?;if owner.terminal_is_empty(){*self.normal.get_mut().expect("exclusive geometry cursor")=None;}return Ok(PluginLifecycleStep::Progress(step.progress()));}
        if !self.evaluation.terminal_is_empty(){return self.evaluation.close_step(grant).map(|step|match step {PluginLifecycleStep::Complete(progress)=>PluginLifecycleStep::Progress(progress),step=>step});}
        let step=self.session.close_step(grant).and_then(|step|semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,self.session.terminal_is_empty(),"original geometry inference session"));
        step.map(|step|PluginLifecycleStep::retained(step,self.terminal_is_empty())).map_err(|error|semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin,semio_framework::FaultCode::new("extension.brep-close"),error.into_message()))
    }
    pub fn terminal_is_empty(&self)->bool {self.normal.lock().is_ok_and(|owner|owner.is_none())&&semio_framework_plugin::ExtensionResourceOwner::terminal_is_empty(&self.evaluation)&&self.session.terminal_is_empty()}
}

use semio_framework_value::{RetireOwned,ValueError,ValueRefusalKind,RetirementDemand,ErasedSnapshotRetirement,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};

struct GeometryInferenceTurn{operation:u64,generation:u64,pointer:usize,length:usize,cursor:Option<flow_extension_sdk::EvaluationInvokeCursor>,wire:Option<String>,quality:Option<&'static str>,faulted:bool,complete:bool,metadata:[Option<Vec<u8>>;2],field:usize,position:usize,active:Option<Box<dyn ErasedSnapshotRetirement>>,closing:bool}
impl GeometryInferenceTurn{
 fn new(request:&ArtifactInferenceExecutionRequest<'_>)->Self{Self{operation:request.operation,generation:request.generation,pointer:request.canonical_payload.as_ptr()as usize,length:request.canonical_payload.len(),cursor:Some(flow_extension_sdk::EvaluationInvokeCursor::new(request.operation,request.generation,request.canonical_payload)),wire:None,quality:None,faulted:false,complete:false,metadata:[None,None],field:0,position:0,active:None,closing:false}}
 fn matches(&self,request:&ArtifactInferenceExecutionRequest<'_>)->bool{self.operation==request.operation&&self.generation==request.generation&&self.pointer==request.canonical_payload.as_ptr()as usize&&self.length==request.canonical_payload.len()}
 fn begin_close(&mut self){self.closing=true;if let Some(cursor)=&mut self.cursor{cursor.begin_close();}}
 fn terminal_is_empty(&self)->bool{self.cursor.is_none()&&self.wire.is_none()&&self.metadata.iter().all(Option::is_none)&&self.active.is_none()}
 fn retirement_demands(&self,registry:&neural_engine::SharedRegistry,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(cursor)=&self.cursor{return cursor.retirement_demands_with_registry(registry,copy)}if let Some(active)=&self.active{return semio_framework_value::retirement::factory::factory_ticket_demands(active,copy)}if self.wire.is_some(){return Ok(geometry_birth::<String>())}if self.metadata.iter().any(Option::is_some){return Ok(geometry_birth::<Vec<u8>>())}Ok(Default::default())}
 fn close_step(&mut self,registry:&neural_engine::SharedRegistry,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}if let Some(cursor)=&mut self.cursor{let step=cursor.close_step_with_registry(registry,grant)?;if cursor.terminal_is_empty(){self.cursor=None;}return Ok(RetainedCloneStep::Progress(step.progress()))}if self.active.is_some(){return semio_framework_value::retirement::factory::close_factory_ticket(&mut self.active,grant)}if self.wire.is_some(){return geometry_admit(&mut self.wire,&mut self.active,grant)}if let Some(source)=self.metadata.iter_mut().find(|source|source.is_some()){return geometry_admit(source,&mut self.active,grant)}Ok(RetainedCloneStep::Complete(Default::default()))}
 fn step(&mut self,registry:&neural_engine::SharedRegistry,request:&ArtifactInferenceExecutionRequest<'_>)->Result<ArtifactInferenceExecutionStep,ArtifactInferenceExecutionError>{
  let grant=request.retained;let pending=|retained_progress|ArtifactInferenceExecutionStep{execution:None,retained_progress,terminal:false};
  if grant.maximum_items==0||grant.maximum_depth==0{return Ok(pending(Default::default()))}
  if !self.matches(request){return Err(ArtifactInferenceExecutionError::native(ValueError::literal(ValueRefusalKind::OwnershipLimit,"geometry inference retains another original caller"),false))}
  if request.cancelled{self.begin_close();}if self.closing{let step=self.close_step(registry,grant).map_err(|error|ArtifactInferenceExecutionError::native(error,false))?;return Ok(ArtifactInferenceExecutionStep{execution:None,retained_progress:step.progress(),terminal:self.terminal_is_empty()})}
  if let Some(cursor)=&mut self.cursor{
   if !cursor.matches(request.operation,request.generation,request.canonical_payload){return Err(ArtifactInferenceExecutionError::native(ValueError::literal(ValueRefusalKind::OwnershipLimit,"geometry inference retains another original caller"),false))}
   if self.quality.is_none(){if let Some(original)=cursor.original_request(){if !original.operator_id.is_empty(){self.quality=geometry_operator_quality_ref(registry,&original.operator_id);}}}
   if cursor.terminal_is_empty()&&self.wire.is_some(){self.cursor=None;return Ok(pending(geometry_item()))}
   let input=flow_extension_sdk::EvaluationRequestContextInput{cancellation_id:request.cancellation_id,policy:request.policy,dependencies:request.dependencies,work_units:request.budgets.work_units,wait_terminal:true};
   let (reply,receipt)=cursor.step_with_context(registry,request.canonical_payload,grant,request.cancelled,input).map_err(|error|{cursor.begin_close();self.closing=true;ArtifactInferenceExecutionError::native(error,false)})?;
   if let Some(reply)=reply{assert!(reply.returned_request.is_none()&&reply.refusal.is_none(),"original callback keeps denied source in its cursor");self.faulted=reply.faulted;self.complete=reply.complete;self.wire=reply.wire;}
   return Ok(pending(receipt));
  }
  if self.field<2{
   let text=if self.field==0{if self.faulted{"invalid"}else{"valid"}}else{match self.quality{Some(quality)=>quality,None if self.faulted=>"InvalidRequest",None=>{self.closing=true;return Err(ArtifactInferenceExecutionError::native(ValueError::literal(ValueRefusalKind::InvariantViolated,"completed geometry has no declared operator quality"),false))}}};
   if self.metadata[self.field].is_none(){if grant.maximum_capacity_bytes<text.len(){return Ok(pending(Default::default()))}let mut destination=Vec::new();destination.try_reserve_exact(text.len()).map_err(|_|ArtifactInferenceExecutionError::native(ValueError::literal(ValueRefusalKind::AllocationFailed,"original geometry metadata allocation failed"),false))?;let capacity=destination.capacity();self.metadata[self.field]=Some(destination);return Ok(pending(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:capacity,..Default::default()}))}
   let count=grant.maximum_copy_bytes.min(text.len().saturating_sub(self.position));if count==0&&self.position!=text.len(){return Ok(pending(Default::default()))}self.metadata[self.field].as_mut().unwrap().extend_from_slice(&text.as_bytes()[self.position..self.position+count]);self.position+=count;if self.position==text.len(){self.field+=1;self.position=0;}return Ok(pending(RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()}))
  }
  let validity=String::from_utf8(self.metadata[0].take().unwrap()).expect("declared validity is UTF8");let quality=String::from_utf8(self.metadata[1].take().unwrap()).expect("declared geometry quality is UTF8");let receipt=geometry_item();Ok(ArtifactInferenceExecution{canonical_payload:self.wire.take().map(String::into_bytes),retirement_progress:receipt,diagnostics:Vec::new(),validity,quality,complete:self.complete,actual_cache_mode:request.requested_cache_mode.clone()}.into_step(true))
 }
}
fn geometry_item()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
fn geometry_birth<T:RetireOwned>()->RetirementDemand{RetirementDemand{capacity_bytes:semio_framework_value::retirement::owned_retirement_birth_bytes::<T>(),depth:1,..Default::default()}}
fn geometry_admit<T:RetireOwned>(source:&mut Option<T>,active:&mut Option<Box<dyn ErasedSnapshotRetirement>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if grant.maximum_capacity_bytes<geometry_birth::<T>().capacity_bytes{return Ok(RetainedCloneStep::Progress(Default::default()))}if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"geometry original handoff requires depth"))}let original=source.take().unwrap();match semio_framework_value::retirement::admit_owned_retirement(original,grant){Ok((owner,receipt))=>{*active=Some(owner);Ok(RetainedCloneStep::Progress(receipt))},Err((error,original))=>{*source=Some(original);Err(error)}}}
fn geometry_operator_quality_ref(registry:&neural_engine::Registry,operator:&str)->Option<&'static str>{
 let info=registry.operator_info(operator)?;if let Some(operation)=operator.strip_prefix("brep.mesh."){return Some(super::mesh::operation_quality(operation))}
 if !super::NODE_KERNEL_METHOD.iter().any(|(id,_)|*id==operator)&&info.group.iter().any(|group|group=="Schemas")&&operator.rsplit('.').next().is_some_and(|schema|registry.schema(schema).is_some()){return Some("SchemaValue")}
 let (_,method)=super::NODE_KERNEL_METHOD.iter().find(|(id,_)|*id==operator)?;use semio_framework_3d::brep::engine::OpQuality;Some(match super::operation_quality(method){OpQuality::ExactAnalytic=>"ExactAnalytic",OpQuality::ExactNumericalWithinTolerance=>"ExactNumericalWithinTolerance",OpQuality::ApproximateBRep=>"ApproximateBRep",OpQuality::MeshDerivedBRep=>"MeshDerivedBRep",OpQuality::PreviewOnly=>"PreviewOnly",OpQuality::Unsupported=>"Unsupported"})
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

#[cfg(test)]
pub(crate) static GEOMETRY_INFERENCE_CALLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// 🎯️ Shares each registered operator's owned fidelity contract with named inference results.
pub(crate) fn geometry_operator_quality(registry: &neural_engine::Registry, operator: &str) -> String {
    let Some(info) = registry.operator_info(operator) else { return "Unsupported".into(); };
    if let Some(operation) = operator.strip_prefix("brep.mesh.") { return super::mesh::operation_quality(operation).into(); }
    if !super::NODE_KERNEL_METHOD.iter().any(|(id, _)| *id == operator) && info.group.iter().any(|group| group == "Schemas") && operator.rsplit('.').next().is_some_and(|schema| registry.schema(schema).is_some()) { return "SchemaValue".into(); }
    super::NODE_KERNEL_METHOD.iter().find(|(id, _)| *id == operator).map(|(_, method)| format!("{:?}", super::operation_quality(method))).unwrap_or_else(|| "Unsupported".into())
}

fn infer_geometry(request:&ArtifactInferenceExecutionRequest<'_>,context:&dyn std::any::Any)->Result<ArtifactInferenceExecutionStep,ArtifactInferenceExecutionError>{
 let context=context.downcast_ref::<GeometryInferenceContext>().ok_or_else(||ArtifactInferenceExecutionError::native(ValueError::literal(ValueRefusalKind::InvalidValue,"geometry inference requires its original execution owner"),true))?;
 if request.canonical_payload.len()as u64>request.budgets.allocation_bytes{return Err(ArtifactInferenceExecutionError::native(ValueError::literal(ValueRefusalKind::OwnershipLimit,"geometry source exceeds original allocation authority"),true))}
 let mut owner=context.normal.lock().expect("original geometry cursor");if owner.is_none(){if request.retained.maximum_items==0||request.retained.maximum_depth==0{return Ok(ArtifactInferenceExecutionStep{execution:None,retained_progress:Default::default(),terminal:false})}*owner=Some(GeometryInferenceTurn::new(request));}
 #[cfg(test)] GEOMETRY_INFERENCE_CALLS.fetch_add(1,std::sync::atomic::Ordering::SeqCst);
 let result=owner.as_mut().unwrap().step(context.registry(),request)?;if result.terminal{assert!(owner.as_ref().unwrap().terminal_is_empty());*owner=None;}Ok(result)
}

/// ♻️ Borrows the actual selected child only under its original callback source.
fn geometry_inference_demands(request:&ArtifactInferenceExecutionRequest<'_>,context:&dyn std::any::Any,copy:usize)->Result<RetirementDemand,ValueError>{
 let context=context.downcast_ref::<GeometryInferenceContext>().ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"geometry demand requires its original execution owner"))?;
 let owner=context.normal.lock().map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"original geometry demand owner is busy"))?;
 let owner=owner.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"live original geometry demand has no retained turn"))?;
 if !owner.matches(request){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"geometry demand retains another original caller"))}
 owner.retirement_demands(context.registry(),copy)
}
