//! 📥️ Original command custody, canonical receiving and ledger closure share one initiating context.
use super::*;
use semio_framework_os_flow::{FlowPreviewCompletedReceiver,FlowPreviewCompletedIntakeSource,FlowPreviewCompletedAddress,FlowPreviewLedgerFold};
use semio_framework_job::{OriginalTurnFailure,InteractiveJobCloseStep};
use semio_framework_value::{DslValue,RetirementDemand,ValueError,ValueRefusalKind};
/// 🧵️ The actual Editor and Viewer command work own the exact moved reply until cache commit or full close.
pub struct PreviewTessellateReceive{receiver:Option<FlowPreviewCompletedReceiver>,ledger:Option<FlowPreviewLedgerFold>,folded:bool,closing:bool,scope:PreviewScopeCopy}
impl PreviewTessellateReceive{
 pub fn new()->Self{Self{receiver:None,ledger:None,folded:false,closing:false,scope:PreviewScopeCopy::new()}}
 pub fn is_folded(&self)->bool{self.folded}
 pub fn configure_scope(&mut self,windows:[Option<&'static str>;2],panels:[Option<&'static str>;2]){self.scope.keys=[windows[0],windows[1],panels[0],panels[1]];}
 pub fn publish_scope(&mut self,cx:&mut StepContext<'_>)->Result<Option<semio_framework::kernel::UiDirtyScope>,OriginalTurnFailure>{self.scope.step(cx)}
 pub fn receive(&mut self,payload:&mut FlowTessellateResolve,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.receiver.is_some(){return Ok(true)}let grant=cx.retained_grant();if cx.is_cancelled()||cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
  if let Some(fault)=payload.fault.take(){cx.consume_fuel(1);let original=semio_framework_job::OriginalTurnFailure::from(fault);let receipt_refusal=cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()}).err();return Err(semio_framework_job::OriginalTurnFailure{receipt_refusal,..original})}
  if !payload.input_valid{cx.consume_fuel(1);return Err(OriginalTurnFailure::debit_original(ValueError::literal(ValueRefusalKind::InvalidValue,"original reply routing input is malformed").with_retained_progress(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()}),cx))}
  self.scope.origin=Some((cx.operation().0,cx.generation().0));let output=std::mem::replace(&mut payload.output,DslValue::Null);match FlowPreviewCompletedReceiver::new(output,cx){Ok(receiver)=>{self.receiver=Some(receiver);Ok(true)},Err((failure,source))=>{match source{FlowPreviewCompletedIntakeSource::Value(output)=>payload.output=output,FlowPreviewCompletedIntakeSource::Receiver(receiver)=>self.receiver=Some(receiver)}Err(failure)}}
 }
 pub fn demands(&self,payload:&FlowTessellateResolve,session:&FlowEvalSession,copy:usize)->Result<RetirementDemand,ValueError>{
  if let Some(receiver)=self.receiver.as_ref().filter(|receiver|!receiver.terminal_is_empty()){return session.next_preview_tessellate_demands(receiver,copy)}
  if let Some(ledger)=self.ledger.as_ref(){return ledger.next_demands(session,&payload.window_id,copy)}
  if !self.folded{return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<u64>()+std::mem::size_of::<semio_framework_os_flow::PreviewTessellateProgress>(),depth:1,..Default::default()})}self.scope.demands(copy)
 }
 pub fn step(&mut self,payload:&FlowTessellateResolve,session:&mut FlowEvalSession,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.folded{return Ok(true)}if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original preview receiving is closing").into())}
  let receiver=self.receiver.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original preview reply was not donated"))?;
  if !receiver.terminal_is_empty(){let address=FlowPreviewCompletedAddress{window_id:&payload.window_id,window_kind_id:&payload.window_kind_id,handle:&payload.handle,node_hash:payload.node_hash,tolerance:payload.tolerance};session.resolve_preview_tessellate(receiver,&address,cx)?;return Ok(false)}
  if self.ledger.is_none(){let progress=receiver.received_progress().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original preview receiving completed without a result"))?;self.ledger=FlowPreviewLedgerFold::new(payload.node_hash,progress,cx)?;return Ok(false)}
  if self.ledger.as_mut().unwrap().step(session,&payload.window_id,cx)?{self.ledger=None;self.receiver=None;self.folded=true;}Ok(self.folded)
 }
 pub fn begin_close(&mut self){self.closing=true;if let Some(receiver)=self.receiver.as_mut(){receiver.begin_close()}if let Some(ledger)=self.ledger.as_mut(){ledger.begin_close()}self.scope.begin_close()}
 pub fn close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(receiver)=self.receiver.as_ref().filter(|receiver|!receiver.terminal_is_empty()){return receiver.next_close_demands(copy)}if let Some(ledger)=self.ledger.as_ref().filter(|ledger|!ledger.terminal_is_empty()){return ledger.next_close_demands(copy)}self.scope.close_demands(copy)}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  self.begin_close();if let Some(receiver)=self.receiver.as_mut(){if !receiver.terminal_is_empty(){let step=receiver.close_step(cx)?;if receiver.terminal_is_empty(){self.receiver=None;}return Ok(InteractiveJobCloseStep::Pending{progress:step.progress()})}self.receiver=None;}
  if let Some(ledger)=self.ledger.as_mut(){if !ledger.terminal_is_empty(){let step=ledger.close_step(cx)?;if ledger.terminal_is_empty(){self.ledger=None;}return Ok(InteractiveJobCloseStep::Pending{progress:step.progress()})}self.ledger=None;}
  self.scope.close_step(cx)
 }
 pub fn terminal_is_empty(&self)->bool{self.receiver.as_ref().is_none_or(FlowPreviewCompletedReceiver::terminal_is_empty)&&self.ledger.as_ref().is_none_or(FlowPreviewLedgerFold::terminal_is_empty)&&self.scope.terminal_is_empty()}
}

#[derive(semio_framework_value::RetireOwned)]
struct ScopeFields{windows:Vec<String>,panels:Vec<String>,text:Option<String>}
struct PreviewScopeCopy{keys:[Option<&'static str>;4],fields:std::mem::ManuallyDrop<Option<ScopeFields>>,close:Option<semio_framework_value::retirement::controlled::ControlledRetirement<ScopeFields>>,origin:Option<(u64,u64)>,phase:u8,slot:usize,offset:usize,closing:bool}
impl PreviewScopeCopy{
 fn new()->Self{Self{keys:[None;4],fields:std::mem::ManuallyDrop::new(None),close:None,origin:None,phase:0,slot:0,offset:0,closing:false}}
 fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  let d=match self.phase{0=>RetirementDemand{depth:1,..Default::default()},1|2=>RetirementDemand{capacity_bytes:self.keys[if self.phase==1{0..2}else{2..4}].iter().filter(|key|key.is_some()).count()*std::mem::size_of::<String>(),depth:1,..Default::default()},3=>RetirementDemand{capacity_bytes:self.keys.get(self.slot).and_then(|key|*key).map_or(0,str::len),depth:1,..Default::default()},4=>RetirementDemand{copy_bytes:self.keys[self.slot].unwrap().len().saturating_sub(self.offset).min(body),depth:1,..Default::default()},5=>RetirementDemand{copy_bytes:std::mem::size_of::<String>(),depth:1,..Default::default()},6=>RetirementDemand{copy_bytes:std::mem::size_of::<semio_framework::kernel::UiDirtyScope>(),depth:1,..Default::default()},_=>Default::default()};Ok(d)
 }
 fn step(&mut self,cx:&mut StepContext<'_>)->Result<Option<semio_framework::kernel::UiDirtyScope>,OriginalTurnFailure>{
  if self.phase==7{return Ok(None)}let g=cx.retained_grant();let d=self.demands(g.maximum_copy_bytes)?;
  if cx.is_cancelled()||cx.should_yield()||g.maximum_items==0||g.maximum_copy_bytes<d.copy_bytes||g.maximum_capacity_bytes<d.capacity_bytes||g.maximum_release_bytes<d.release_bytes||g.maximum_depth<d.depth||(self.phase==4&&d.copy_bytes==0){return Ok(None)}
  if self.origin.is_some_and(|origin|origin!=(cx.operation().0,cx.generation().0)){cx.consume_fuel(1);return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original preview publication initiating source changed").into())}self.origin=Some((cx.operation().0,cx.generation().0));cx.consume_fuel(1);
  let mut p=semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};let mut output=None;
  match self.phase{
   0=>{*self.fields=Some(ScopeFields{windows:Vec::new(),panels:Vec::new(),text:None});self.phase=1;},
   1|2=>{let fields=self.fields.as_mut().unwrap();let target=if self.phase==1{&mut fields.windows}else{&mut fields.panels};let count=self.keys[if self.phase==1{0..2}else{2..4}].iter().filter(|key|key.is_some()).count();target.reserve_exact(count);p.retained_capacity_bytes=target.capacity()*std::mem::size_of::<String>();self.phase+=1;},
   3=>{if self.slot==4{self.phase=6;}else if let Some(key)=self.keys[self.slot]{let value=String::with_capacity(key.len());p.retained_capacity_bytes=value.capacity();self.fields.as_mut().unwrap().text=Some(value);self.phase=4;}else{self.slot+=1;}},
   4=>{let source=self.keys[self.slot].unwrap();let mut end=self.offset+d.copy_bytes;while !source.is_char_boundary(end){end-=1;}if end==self.offset{return Err(OriginalTurnFailure::debit_original(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original preview publication text unit exceeds copy body").with_retained_progress(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()}),cx))}self.fields.as_mut().unwrap().text.as_mut().unwrap().push_str(&source[self.offset..end]);p.copied_bytes=end-self.offset;self.offset=end;if end==source.len(){self.phase=5;}},
   5=>{let fields=self.fields.as_mut().unwrap();let value=fields.text.take().unwrap();if self.slot<2{fields.windows.push(value);}else{fields.panels.push(value);}self.offset=0;self.slot+=1;self.phase=3;},
   6=>{let fields=self.fields.take().unwrap();output=Some(semio_framework::kernel::UiDirtyScope::Partial{window_bodies:fields.windows,panel_bodies:fields.panels,utilities:false,tools:false,engagements:false,measures:false,labels:false});self.phase=7;},_=>unreachable!(),
  }if let Err(error)=cx.consume_retained(p){if let Some(semio_framework::kernel::UiDirtyScope::Partial{window_bodies,panel_bodies,..})=output.take(){*self.fields=Some(ScopeFields{windows:window_bodies,panels:panel_bodies,text:None});self.phase=6;}return Err(error.into())}Ok(output)
 }
 fn begin_close(&mut self){self.closing=true;}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{self.close.as_ref().map_or(Ok(RetirementDemand{depth:usize::from(self.fields.is_some()),..Default::default()}),|close|close.selected_demand(body))}
 fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
  self.begin_close();if self.terminal_is_empty(){return Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}if self.origin.is_some_and(|origin|origin!=(cx.operation().0,cx.generation().0)){if cx.should_yield(){return Ok(InteractiveJobCloseStep::Blocked)}cx.consume_fuel(1);return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original preview publication close source changed").into())}
  let d=self.close_demands(cx.retained_grant().maximum_copy_bytes)?;let step=semio_framework_job::close_original_retained_step(cx,d,|grant|if let Some(close)=self.close.as_mut(){close.step(grant)}else{self.close=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(self.fields.take().unwrap()).unwrap_or_else(|_|unreachable!()));Ok(semio_framework_value::RetainedCloneStep::Progress(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()}))})?;
  if self.close.as_ref().is_some_and(|close|close.terminal_is_empty()){self.close=None;}Ok(if self.terminal_is_empty(){InteractiveJobCloseStep::Complete{progress:step.progress()}}else{InteractiveJobCloseStep::Pending{progress:step.progress()}})
 }
 fn terminal_is_empty(&self)->bool{self.fields.is_none()&&self.close.is_none()}
}
impl Drop for PreviewScopeCopy{fn drop(&mut self){assert!(self.terminal_is_empty(),"original preview publication requires paid full close");}}

/// 🧠️ The actual evaluation command owns its donated reply through Dictionary/cache receiving and addressed publication.
pub struct PreviewEvalReceive{receiver:Option<semio_framework_os_flow::FlowEvaluationReplyReceiver>,scope:PreviewScopeCopy,folded:bool,closing:bool}
impl PreviewEvalReceive{
 pub fn new()->Self{Self{receiver:None,scope:PreviewScopeCopy::new(),folded:false,closing:false}}
 pub fn configure_scope(&mut self,windows:[Option<&'static str>;2],panels:[Option<&'static str>;2]){self.scope.keys=[windows[0],windows[1],panels[0],panels[1]];}
 pub fn is_folded(&self)->bool{self.folded}
 pub fn receive(&mut self,payload:&mut FlowEvalResolve,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.receiver.is_some(){return Ok(true)}let g=cx.retained_grant();if cx.is_cancelled()||cx.should_yield()||g.maximum_items==0||g.maximum_depth==0{return Ok(false)}
  if let Some(fault)=payload.fault.take(){cx.consume_fuel(1);let original=OriginalTurnFailure::from(fault);let receipt_refusal=cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()}).err();return Err(OriginalTurnFailure{receipt_refusal,..original})}
  if !payload.input_valid{cx.consume_fuel(1);return Err(OriginalTurnFailure::debit_original(ValueError::literal(ValueRefusalKind::InvalidValue,"original reply routing input is malformed").with_retained_progress(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()}),cx))}
  self.scope.origin=Some((cx.operation().0,cx.generation().0));let output=std::mem::replace(&mut payload.output,DslValue::Null);match semio_framework_os_flow::FlowEvaluationReplyReceiver::new(output,cx){Ok(receiver)=>{self.receiver=Some(receiver);Ok(true)},Err((failure,source))=>{match source{semio_framework_os_flow::FlowEvaluationReplyIntakeSource::Value(output)=>payload.output=output,semio_framework_os_flow::FlowEvaluationReplyIntakeSource::Receiver(receiver)=>self.receiver=Some(receiver)}Err(failure)}}
 }
 pub fn demands(&self,payload:&FlowEvalResolve,session:&FlowEvalSession,body:usize)->Result<RetirementDemand,ValueError>{if let Some(receiver)=self.receiver.as_ref(){return receiver.next_demands(session,body,&payload.window_id)}self.scope.demands(body)}
 pub fn step(&mut self,payload:&FlowEvalResolve,session:&mut FlowEvalSession,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if self.folded{return Ok(true)}let receiver=self.receiver.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original evaluation reply was not donated"))?;let address=semio_framework_os_flow::FlowEvaluationReplyAddress{node_hash:payload.node_hash,window_id:&payload.window_id,window_kind_id:&payload.window_kind_id};if receiver.step(session,&address,cx)?{self.receiver=None;self.folded=true;}Ok(self.folded)}
 pub fn publish_scope(&mut self,cx:&mut StepContext<'_>)->Result<Option<semio_framework::kernel::UiDirtyScope>,OriginalTurnFailure>{self.scope.step(cx)}
 pub fn begin_close(&mut self){self.closing=true;if let Some(receiver)=self.receiver.as_mut(){receiver.begin_close()}self.scope.begin_close()}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(receiver)=self.receiver.as_ref(){return receiver.next_close_demands(body)}self.scope.close_demands(body)}
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{self.begin_close();if let Some(receiver)=self.receiver.as_mut(){let step=receiver.close_step(cx)?;if receiver.terminal_is_empty(){self.receiver=None;}return Ok(InteractiveJobCloseStep::Pending{progress:step.progress()})}self.scope.close_step(cx)}
 pub fn terminal_is_empty(&self)->bool{self.receiver.as_ref().is_none_or(semio_framework_os_flow::FlowEvaluationReplyReceiver::terminal_is_empty)&&self.scope.terminal_is_empty()}
}
