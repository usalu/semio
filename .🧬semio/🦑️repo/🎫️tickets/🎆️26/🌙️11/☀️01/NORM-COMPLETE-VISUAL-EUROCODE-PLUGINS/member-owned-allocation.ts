import{readFileSync,writeFileSync}from"node:fs";const p="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🛂️receiving/🦀️.rs";let s=readFileSync(p,"utf8");
s=s.replace("use semio_framework_value_native_owned::{NativeAllocationRequest,NativeAllocationEvent,NativeAllocationBudget};","use semio_framework_value_native_owned::{NativeAllocationRequest,NativeAllocationEvent,NativeAllocationBudget,NativeAllocationAuthority,NativeAllocationAuthorityPort,NativeAllocationRefusal};").replace("::{NativeDecodeAllocationIssuer,NativeForwardedDecodeControl}","::{NativeForwardedDecodeControl}");
const a=s.indexOf("#[derive(Default)]\nstruct MemberHistoryAllocation"),b=s.indexOf("\npub struct MemberHistoryReceiving",a);if(a<0||b<a)throw Error("allocation owner boundaries");
s=s.slice(0,a)+`trait MemberAllocationCaller:Sync{
 fn validate_source(&self,source:&semio_framework_job::OriginalCallerSourceLoan,binding:semio_framework_job::OriginalSourceBinding)->Result<(),ValueError>;
 fn allocation_view(&self,event:NativeAllocationEvent,closing:bool)->Result<(RetainedCloneGrant,bool),ValueError>;
}
impl MemberAllocationCaller for BorrowedOriginalTurn<'_,'_>{
 fn validate_source(&self,source:&semio_framework_job::OriginalCallerSourceLoan,binding:semio_framework_job::OriginalSourceBinding)->Result<(),ValueError>{self.with(|cx|{source.validate_retirement_context(cx)?;if !binding.matches(cx){return Err(invariant("Member decoding requires its original source and clock"))}Ok(())})?}
 fn allocation_view(&self,event:NativeAllocationEvent,closing:bool)->Result<(RetainedCloneGrant,bool),ValueError>{self.with(|cx|{let may_quote=!matches!(event,NativeAllocationEvent::Quote(_)|NativeAllocationEvent::Admit(_))||!paused(cx,closing)&&(closing||!cx.is_cancelled());(cx.retained_grant(),may_quote)})}
}
struct MemberAllocationContext<'a>{caller:&'a dyn MemberAllocationCaller,closing:bool,hop_started:bool}
struct MemberHistoryAllocationPort{source:semio_framework_job::OriginalCallerSourceLoan,binding:semio_framework_job::OriginalSourceBinding,state:NativeAllocationBudget,pending_event:Option<NativeAllocationEvent>}
impl MemberHistoryAllocationPort{
 fn receive_original(&mut self,context:&mut MemberAllocationContext<'_>,event:NativeAllocationEvent)->Result<(),NativeAllocationRefusal>{
  if !context.hop_started{self.state.begin_hop();context.hop_started=true}
  let result=context.caller.allocation_view(event,context.closing).and_then(|(grant,may_quote)|self.state.receive(event,grant,may_quote));self.pending_event=None;result.map_err(Into::into)
 }
}
impl NativeAllocationAuthorityPort for MemberHistoryAllocationPort{
 type Context<'turn,'source>=MemberAllocationContext<'turn> where 'source:'turn;
 fn validate_context<'turn,'source>(&self,context:&Self::Context<'turn,'source>)->Result<(),ValueError> where 'source:'turn{context.caller.validate_source(&self.source,self.binding)}
 fn receive<'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,event:NativeAllocationEvent)->Result<(),NativeAllocationRefusal> where 'source:'turn{if self.pending_event.is_some(){return Err(invariant("Member allocation retains an unfinished original event").into())}self.pending_event=Some(event);self.receive_original(context,event)}
 fn recover<'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,event:NativeAllocationEvent)->Result<Option<Result<(),NativeAllocationRefusal>>,ValueError> where 'source:'turn{if self.pending_event!=Some(event){return Ok(None)}self.validate_context(context)?;Ok(Some(self.receive_original(context,event)))}
}
#[derive(Default)]
struct MemberHistoryAllocation{source:ManuallyDrop<Option<semio_framework_job::OriginalCallerSourceLoan>>,authority:ManuallyDrop<Option<NativeAllocationAuthority<MemberHistoryAllocationPort>>>,work_issuer:semio_framework_value_native_owned::native_decoding::NativeDecodeWorkIssuer}
impl MemberHistoryAllocation{
 fn has_source(&self)->bool{self.source.is_some()||self.authority.is_some()}
 fn has_pending_allocation(&self)->bool{self.authority.as_ref().is_some_and(NativeAllocationAuthority::has_pending_transaction)}
 fn transition_demand()->RetirementDemand{RetirementDemand{copy_bytes:2*std::mem::size_of::<MemberHistoryAllocationPort>()+std::mem::size_of::<NativeAllocationAuthority<MemberHistoryAllocationPort>>()+2*std::mem::size_of::<Option<semio_framework_job::OriginalCallerSourceLoan>>(),depth:1,..Default::default()}}
 fn transition(cx:&mut StepContext<'_>,grant:RetainedCloneGrant,closing:bool,run:impl FnOnce())->Result<bool,OriginalTurnFailure>{let demand=Self::transition_demand();if paused(cx,closing)||(!closing&&cx.is_cancelled())||cx.fuel_remaining()<2||!semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(false)}run();let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};cx.consume_fuel(2);cx.consume_retained(progress).map_err(|error|OriginalTurnFailure::from(error.with_retained_progress(progress)))?;Ok(true)}
 fn admit_authority(&mut self,cx:&mut StepContext<'_>,grant:RetainedCloneGrant)->Result<bool,OriginalTurnFailure>{if self.authority.is_some(){return Ok(true)}let source=self.source.as_ref().ok_or_else(||invariant("Member allocation authority requires its paid original source"))?;source.validate_context(cx)?;let binding=semio_framework_job::OriginalSourceBinding::borrow(cx);Self::transition(cx,grant,false,||{let source=self.source.take().expect("observed original source loan");*self.authority=Some(NativeAllocationAuthority::new(MemberHistoryAllocationPort{source,binding,state:Default::default(),pending_event:None}));})}
 fn return_authority(&mut self,cx:&mut StepContext<'_>,grant:RetainedCloneGrant)->Result<bool,OriginalTurnFailure>{if self.authority.is_none(){return Ok(true)}if self.has_pending_allocation(){return Ok(false)}if self.source.is_some(){return Err(invariant("Member original source return slot is occupied").into())}let mut refusal=false;let moved=Self::transition(cx,grant,true,||{let authority=self.authority.take().expect("observed original allocation authority");match authority.into_port(){Ok(port)=>{let MemberHistoryAllocationPort{source,state,pending_event,..}=port;assert!(pending_event.is_none()&&state.terminal_is_empty(),"returned Member allocation port retains an original event");*self.source=Some(source)},Err(authority)=>{*self.authority=Some(authority);refusal=true}}})?;if refusal{return Err(invariant("Member allocation authority refused its selected owner transfer").into())}Ok(moved)}
}
impl Drop for MemberHistoryAllocation{fn drop(&mut self){assert!(std::thread::panicking()||!self.has_source()&&!self.work_issuer.has_pending_work(),"Member allocation abandoned its original source");if !self.has_source(){unsafe{ManuallyDrop::drop(&mut self.source);ManuallyDrop::drop(&mut self.authority)}}}}
`+s.slice(b);
const x=s.indexOf("fn with_original_control<T>("),y=s.indexOf("\nfn controlled_retirement",x);if(x<0||y<x)throw Error("control boundaries");s=s.slice(0,x)+`fn with_original_control<T>(receipt:&mut Option<NativeForwardedDecodeReceipt>,recipient:&mut NativeDecodeRetirementRecipient,allocation:&mut MemberHistoryAllocation,maximum:usize,cx:&mut StepContext<'_>,closing:bool,run:impl FnOnce(&mut NativeDecodeControl<'_>)->T)->Result<(T,u64),OriginalTurnFailure>{
 let borrowed=BorrowedOriginalTurn::new(&mut *cx);let mut context=MemberAllocationContext{caller:&borrowed,closing,hop_started:false};let mut progress=|_|borrowed.with(|actual|!paused(actual,closing)&&(closing||!actual.is_cancelled())).unwrap_or(false);
 let authority=allocation.authority.as_mut().ok_or_else(||invariant("Member decode requires its admitted original allocation port"))?;let work_issuer=&mut allocation.work_issuer;
 authority.with_send_loan(&mut context,|allocation_loan|{
  let mut control=if let Some(original)=receipt.take(){match NativeForwardedDecodeControl::rebind(original,&mut progress,allocation_loan,recipient){Ok(control)=>control,Err((error,original))=>{*receipt=Some(original);return Err(error.into())}}}else{let mut control=NativeDecodeControl::new_forwarded(maximum,&mut progress,allocation_loan);control.install_retirement_recipient(recipient)?;control};
  let before=control.actual_work();let mut work=OriginalMemberWork{turn:&borrowed,closing};let mut work_loan=work_issuer.loan(&mut work);let result=control.with_work_recipient(&mut work_loan,|native|Ok::<_,ValueError>(run(native)));let observed=control.actual_work().saturating_sub(before);
  *receipt=Some(control.detach().map_err(|_|invariant("original Member decode continuation violated canonical control invariants"))?);result.map(|value|(value,observed)).map_err(Into::into)
 })?
}
`+s.slice(y);
s=s.replace("self.closed&&self.allocation.source.is_none()&&!self.allocation.work_issuer.has_pending_work()&&self.allocation.state.terminal_is_empty()","self.closed&&!self.allocation.has_source()&&!self.allocation.work_issuer.has_pending_work()&&!self.allocation.has_pending_allocation()");
s=s.replace("if !self.closing&&!self.admitted&&self.allocation.source.is_none(){","if !self.closing&&!self.admitted&&!self.allocation.has_source(){");
s=s.replace("  if self.native_custody_empty()&&(self.closing||self.admitted){","  if !self.closing&&!self.admitted&&self.allocation.source.is_some()&&self.allocation.authority.is_none(){return Ok(MemberHistoryAllocation::transition_demand())}\n  if self.native_custody_empty()&&(self.closing||self.admitted)&&self.allocation.authority.is_some(){return Ok(MemberHistoryAllocation::transition_demand())}\n  if self.native_custody_empty()&&(self.closing||self.admitted){");
s=s.replaceAll("&&self.allocation.state.terminal_is_empty()","&&!self.allocation.has_pending_allocation()");
s=s.replace(" fn close_original_source(&mut self,cx:&mut StepContext<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,OriginalTurnFailure>{"," fn close_original_source(&mut self,cx:&mut StepContext<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,OriginalTurnFailure>{\n  if self.allocation.authority.is_some(){let returned=self.allocation.return_authority(cx,grant)?;return Ok(RetainedCloneStep::Progress(if returned{RetainedCloneProgress{copied_items:1,copied_bytes:MemberHistoryAllocation::transition_demand().copy_bytes,..Default::default()}}else{Default::default()}))}");
s=s.replace("if !self.admitted&&self.allocation.source.is_none(){","if !self.admitted&&!self.allocation.has_source(){");
s=s.replace("  if self.admitted&&self.native_custody_empty()&&self.allocation.source.is_some(){","  if !self.admitted&&self.allocation.authority.is_none(){return Ok(if self.allocation.admit_authority(cx,grant)?{RetainedHistorySourceStep::Progress}else{RetainedHistorySourceStep::Blocked})}\n  if self.admitted&&self.native_custody_empty()&&self.allocation.has_source(){");
s=s.replace("if self.admitted&&self.allocation.source.is_none()&&","if self.admitted&&!self.allocation.has_source()&&").replace("if self.native_custody_empty()&&self.allocation.source.is_some(){","if self.native_custody_empty()&&self.allocation.has_source(){").replace(";ManuallyDrop::drop(&mut self.allocation.source)","");
writeFileSync(p,s);
