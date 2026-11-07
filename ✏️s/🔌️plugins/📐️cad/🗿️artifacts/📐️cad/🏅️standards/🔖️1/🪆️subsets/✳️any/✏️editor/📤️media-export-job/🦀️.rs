//! 🎞️ CAD reserved import borrows original media, snapshot and completion until funded close.
use super::*;
use semio_plugin::EphemeralEmit;
#[derive(Default)]
pub(super) struct CadMediaWork{
 priced_bytes:usize,price_checksum:u64,pricing_complete:bool,validated:bool,completed:bool,closing:bool,
 rejection:std::mem::ManuallyDrop<Option<semio_plugin::app::ArtifactToolCompletionRejection<EditorApp<CadPlayApp>>>>,
 retirement:semio_framework_value::retained_clone::RetainedCloneClose,
}
impl CadMediaWork{
 fn borrowed_source(media:&Media)->Result<&[u8],Fault>{match &media.payload{
  MediaPayload::Structured{json,..}=>Ok(json.as_bytes()),
  MediaPayload::Intrinsic{value,..}=>value.as_bytes().or_else(||value.as_str().map(str::as_bytes)).ok_or_else(||Fault::new(semio_framework_diagnostic::FaultOrigin::App,semio_framework_diagnostic::FaultCode::new("cad.media-input-shape"),"Cad file media requires borrowed intrinsic text or bytes")),
  MediaPayload::Binary{..}=>Err(Fault::new(semio_framework_diagnostic::FaultOrigin::App,semio_framework_diagnostic::FaultCode::new("cad.media-input-shape"),"Cad file media requires a retained source rather than a binary address")),
 }}
 fn price_source(&mut self,media:&Media,maximum_bytes:usize)->Result<usize,Fault>{
  if maximum_bytes==0||self.pricing_complete{return Ok(0)}let source=Self::borrowed_source(media)?;if source.len()>semio_plugin::CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES{return Err(Fault::new(semio_framework_diagnostic::FaultOrigin::App,semio_framework_diagnostic::FaultCode::new("cad.media-input-capacity"),"Cad media source exceeds its declared retained raw-byte authority"))}let end=self.priced_bytes.saturating_add(maximum_bytes.min(64)).min(source.len());let mut checksum=self.price_checksum;for byte in &source[self.priced_bytes..end]{checksum=checksum.wrapping_mul(31).wrapping_add(u64::from(*byte));}let charged=end-self.priced_bytes;self.price_checksum=checksum;self.priced_bytes=end;self.pricing_complete=end==source.len();Ok(charged)
 }
 fn price_step(&mut self,media:&Media,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,Fault>{
  if cx.is_cancelled(){return Err(Fault::new(semio_framework_diagnostic::FaultOrigin::Framework,semio_framework_diagnostic::FaultCode::new("interactive-job.cancelled"),"Cad media pricing was cancelled before conversion"))}if cx.should_yield(){return Ok(false)}if !self.pricing_complete{let result=self.price_source(media,64);cx.consume_fuel(1);result?;return Ok(false)}Ok(true)
 }
 fn step_value<'a>(&'a mut self,request:&semio_plugin::ArtifactReservedToolJobRequest<EditorApp<CadPlayApp>>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>,semio_framework_job::OriginalTurnFailure>{
  use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneProgress};
  if self.closing||cx.is_cancelled(){return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into)}if cx.should_yield(){return Ok(None)}if self.completed{return semio_framework_job::JobOutcomeBorrow::admit_complete(cx,None,None).map_err(Into::into)}
  let Some(semio_plugin::ArtifactReservedToolInput::Media{port,media})=request.input.as_ref()else{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Cad media requires its declared input owner").into())};
  if !self.validated{let valid=request.tool_id=="import-media"&&request.raw_wire.is_empty()&&!request.authoring_seed.is_empty()&&matches!(port.as_str(),"geometry:in"|"artifact:in");cx.consume_fuel(1);if !valid{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Cad media requires exact decoded port and authoring authority").into())}self.validated=true;return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(Into::into)}
  cx.set_stage("cad-media-pricing");if !self.price_step(media,cx)?{return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(Into::into)}
  let bytes=request.parent_document_id.len()+request.authoring_seed.len();let copied=2*size_of::<AppOperationContext>()+2*size_of::<ChildContentView>()+bytes;let demand=RetirementDemand{copy_bytes:copied,capacity_bytes:bytes,release_bytes:bytes,depth:1};if !cx.retained_work_can_enter(demand){return Ok(None)}
  let operation=AppOperationContext{retained:request.retained,app_instance_id:request.app_instance_id,parent_document_id:request.parent_document_id.clone(),operation_id:cx.operation().0,generation:cx.generation().0,canonical_base_revision:request.canonical_base_revision,authoring_seed:request.authoring_seed.clone()};
  let capacity=operation.parent_document_id.capacity()+operation.authoring_seed.capacity();let doc=ArtifactView::with_children(request.snapshot.as_ref(),request.history.as_ref(),request.children.clone()).bound_to_operation(operation);cx.set_stage("cad-media-decode");let emit=CadPlayApp::import_media(port,media,&doc);drop(doc);cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:copied,retained_capacity_bytes:capacity,released_bytes:capacity})?;
  let emit=emit.map_err(|error|Fault::from(error.to_string()))?;if let Err(rejected)=request.completion.complete(Ok(emit),EphemeralEmit::default()){*self.rejection=Some(rejected);return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Cad media completion was rejected").into())}self.completed=true;semio_framework_job::JobOutcomeBorrow::admit_complete(cx,None,None).map_err(Into::into)
 }
 fn demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.retirement.next_owner_demands::<semio_plugin::app::ArtifactToolCompletionRejection<EditorApp<CadPlayApp>>>(self.rejection.is_some(),&None,body)}
}
impl semio_plugin::plugin_app_close_prelude::ArtifactReservedWork<EditorApp<CadPlayApp>> for CadMediaWork{
 fn step<'a>(&'a mut self,request:&'a mut semio_plugin::ArtifactReservedToolJobRequest<EditorApp<CadPlayApp>>,cx:&mut semio_framework_job::StepContext<'_>,_:&mut semio_framework_job::OriginalJobReturnClaim<'_>)->semio_framework_job::InteractiveJobStep<'a>{semio_framework_job::InteractiveJobStep::from_callback(self.step_value(request,cx))}
 fn borrow_outcome<'a>(&'a self,descriptor:&'a semio_framework_job::JobOutcomeDescriptor)->Result<semio_framework_job::JobOutcomeView<'a>,semio_framework_value::ValueError>{use semio_framework_job::JobOutcomeKind;match descriptor.kind(){JobOutcomeKind::Yield=>descriptor.yielded(),JobOutcomeKind::Cancelled=>descriptor.cancelled(),JobOutcomeKind::Complete if self.completed=>descriptor.complete(None,None),_=>Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"Cad reserved outcome is not owned"))}}
 fn begin_close(&mut self){self.closing=true}
 fn close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{self.demands(body)}
 fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
  self.closing=true;let demand=self.demands(cx.retained_grant().maximum_copy_bytes)?;let step=semio_framework_job::close_original_retained_step(cx,demand,|grant|if self.retirement.is_empty(){let turn=self.retirement.begin_granted(&mut self.rejection,grant);semio_framework_value::RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(semio_framework_value::RetainedCloneStep::Complete(Default::default()))),work:turn.work}}else{self.retirement.step_granted(grant)})?;Ok(if self.terminal_is_empty(){semio_framework_job::InteractiveJobCloseStep::Complete{progress:step.progress()}}else{step})
 }
 fn terminal_is_empty(&self)->bool{self.closing&&self.rejection.is_none()&&self.retirement.is_empty()}
}
