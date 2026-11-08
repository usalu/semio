import sys
D="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧳️original/"
F=D+"🦀️.rs";P=D+"🪞️projection/🦀️.rs"
L=open(F,encoding="utf-8").read().split("\n")
def find(prefix):
    for i,l in enumerate(L):
        if l.startswith(prefix): return i
    sys.exit("missing "+prefix)
def sub(old,new):
    for i,l in enumerate(L):
        if old in l:
            assert l.count(old)==1,(old,l.count(old));L[i]=l.replace(old,new);return
    sys.exit("nomatch "+old[:60])
sub("use projection::OriginalInput;","use projection::OriginalInput;\nuse semio_framework_value_native_receiver_protocol::{NativeAdvanceSelection,NativeSlotOperation,NativeSlotProducer};\nuse crate::native_decoding::OriginalEffectAuthority;\ntype Authority<I,T>=OriginalEffectAuthority<NativeSlotProducer<I,T,<T as NativeReceive<I>>::Receiver>>;")
sub("action:semio_framework_value_native_receiver_protocol::NativeReceivingChild,closing:bool}","action:semio_framework_value_native_receiver_protocol::NativeReceivingChild<OriginalReceivingPrimary>,authority:ManuallyDrop<Option<Authority<I,T>>>,closing:bool}\nfn original_seed<I:Sync+'static,T:NativeReceive<I>>(_:bool)->Result<T::Receiver,ValueError>{Ok(T::receiver())}")
sub("retirement:Default::default(),action:Default::default(),closing:false}}\n" if False else "retirement:Default::default(),action:Default::default(),closing:false}}","retirement:Default::default(),action:Default::default(),authority:ManuallyDrop::new(Some(OriginalEffectAuthority::new(NativeSlotProducer::new()))),closing:false}}")
a=find(" pub fn advance(&mut self,control")
L[a]=''' pub fn advance(&mut self,control:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>{
  let done=|result:Result<RetainedCloneStep,OriginalReceivingPrimary>|->RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>{RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work:0})};
  if self.action.request.is_some_and(|request|request!=(body,maximum_work_units)){return done(Err(ValueError::invariant("original local receiver request changed").into()))}
  if maximum_work_units==0{return if self.action.demand.is_some(){RetainedWorkPoll::Pending{work:0}}else{done(Ok(RetainedCloneStep::Progress(Default::default())))}}
  let result:Result<RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>,OriginalReceivingPrimary>=(||{
   let d=self.demands(body,maximum_work_units)?;if !retained_clone_grant_admits(g,d){return Ok(if self.action.demand.is_some(){RetainedWorkPoll::Pending{work:0}}else{done(Ok(RetainedCloneStep::Progress(Default::default())))})}
   if self.action.demand.is_none(){control.checkpoint()?;crate::owned::receiving_capacity(control,d.capacity_bytes)?}
   let input=self.original.as_ref().ok_or_else(invalid)?.try_borrow()?;let authority=self.authority.as_mut().ok_or_else(invalid)?;
   if !self.receiver.reserved()||self.action.tag==3{self.action.tag=3;let units=self.action.work.unwrap_or_else(||self.receiver.reserve_work_demand());return Ok(self.action.poll(control,d,g,units,maximum_work_units,authority,|_,effect,authority,action|authority.drive(&mut NativeSlotOperation::Reserve(input,&mut self.receiver),effect,action)))}
   if !self.receiver.initialized()||self.action.tag==4{self.action.tag=4;let units=self.action.work.unwrap_or_else(||self.receiver.seed_work_demand());return Ok(self.action.poll(control,d,g,units,maximum_work_units,authority,|_,effect,authority,action|authority.drive(&mut NativeSlotOperation::Seed(input,&mut self.receiver,false,original_seed::<I,T>),effect,action)))}
   let receiver=self.receiver.get_mut().ok_or_else(||ValueError::invariant("get_mut target absent"))?;
   if receiver.output_ready()||self.action.tag==5{self.action.tag=5;let units=match self.action.work{Some(work)=>work,None=>receiver.take_work_demand()?};return Ok(self.action.poll(control,d,g,units,maximum_work_units,authority,|_,effect,authority,action|authority.drive(&mut NativeSlotOperation::Publish(input,receiver,&mut self.output),effect,action)))}
   let selection=if self.action.tag==6{NativeAdvanceSelection::Local{work:self.action.work.ok_or_else(||ValueError::invariant("original local receiver action lost its work quote"))?}}else{receiver.select_advance(input,body,maximum_work_units)?};
   match selection{
    NativeAdvanceSelection::Nested=>Ok(receiver.advance_nested(input,control,body,maximum_work_units,RetainedCloneGrant{maximum_depth:g.maximum_depth-1,..g}).map(|step|RetainedCloneStep::Progress(step.progress()))),
    NativeAdvanceSelection::Local{work}=>{if self.action.demand.is_none()&&work>maximum_work_units{return Ok(done(Ok(RetainedCloneStep::Progress(Default::default()))))}self.action.tag=6;Ok(self.action.poll_local(control,d,g,work,body,maximum_work_units,authority,|_,effect,authority,action|authority.drive(&mut NativeSlotOperation::Advance(input,receiver,body),effect,action)))}
   }
  })();
  match result{Ok(turn)=>turn,Err(error)=>done(Err(error))}
 }'''
# imports
sub("use crate::RetainedWorkOutcome;","use crate::{RetainedWorkOutcome,RetainedWorkPoll};")
# close_demands / close_work_demand
sub("if self.output.is_some(){return nested(self.retirement.next_owner_demands::<T>(true,&None,body)?,0)}if let Some(original)","if self.output.is_some(){return nested(self.retirement.next_owner_demands::<T>(true,&None,body)?,0)}if self.authority.is_some(){return nested(self.retirement.next_owner_demands::<Authority<I,T>>(true,&None,body)?,0)}if let Some(original)")
sub("if self.output.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}match self.original.as_ref(){Some(original)if !original.terminal_is_empty()=>original.close_work_demand(body)","if self.output.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}if self.authority.is_some(){return self.retirement.next_owner_work::<Authority<I,T>>(true,&None,body)}match self.original.as_ref(){Some(original)if !original.terminal_is_empty()=>original.next_close_work(body)")
# close_step
a=find(" pub fn close_step(&mut self,effect")
L[a]=''' pub fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{
  use crate::RetainedWorkPoll::Complete;let fail=|error:ValueError|->crate::OriginalRetirementPoll{Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};
  if self.requires_original_control(){return fail(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native original receiver requires its original close loan"))}
  let g=effect.grant();if self.terminal_is_empty(){return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})}
  let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return fail(error)};if !retained_clone_grant_admits(g,d){return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}
  let child=RetainedCloneGrant{maximum_depth:g.maximum_depth.saturating_sub(1),..g};
  if self.receiver.reserved()||self.retired_header.is_some(){if let Some(receiver)=self.receiver.get_mut().filter(|receiver|!receiver.terminal_is_empty()){return receiver.close_step(&mut effect.nested(child)).map(|s|RetainedCloneStep::Progress(s.progress()))}return crate::owned::close_native_receiver_backing::<I,T,_>(&mut self.receiver,&mut self.retired_header,effect).map(|s|RetainedCloneStep::Progress(s.progress()))}
  if !self.retirement.is_empty(){return self.retirement.step_granted(&mut effect.nested(child)).map(|s|RetainedCloneStep::Progress(s.progress()))}
  if self.action.failure.is_some(){return Complete(self.action.close_failure(&mut self.retirement,&mut effect.nested(child)))}
  if self.output.is_some(){let turn=self.retirement.begin_granted(&mut self.output,&mut effect.nested(child));return Complete(RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work})}
  if self.authority.is_some(){let turn=self.retirement.begin_granted(&mut self.authority,&mut effect.nested(child));return Complete(RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work})}
  if let Some(original)=self.original.as_mut(){if !original.terminal_is_empty(){return original.close_step(&mut effect.nested(child)).map(|s|RetainedCloneStep::Progress(s.progress()))}match effect.performed(1){Ok(true)=>{},Ok(false)=>return fail(ValueError::literal(ValueRefusalKind::WorkLimit,"native original receiver release exceeded its work authority")),Err(error)=>return fail(error)}self.original.take();return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(receipt(0))),work:1})}
  Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})
 }'''
a=find(" pub fn close_original_birth_step(&mut self,original")
L[a]=''' pub fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{use crate::RetainedWorkPoll::Complete;if grant.maximum_items==0||grant.maximum_depth==0{return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};if let Some(receiver)=self.receiver.get_mut(){if !receiver.terminal_is_empty()&&receiver.requires_original_birth(){return receiver.close_original_birth_step(original,child)}}if !self.receiver.reserved()&&self.retired_header.is_none()&&self.retirement.requires_original_birth(){return self.retirement.step_original_birth(original,child)}Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native original receiver has no birth to settle").into()),work:0})}'''
a=find(" pub fn close_original_step(&mut self,original")
L[a]=''' pub fn close_original_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{use crate::RetainedWorkPoll::Complete;let fail=|error:ValueError|->crate::OriginalRetirementPoll{Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};
  if self.action.pending(){return fail(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original receiver birth requires settlement"))}
  if self.requires_original_birth(){return fail(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"settle original birth before opening a close effect"))}
  let g=effect.grant();let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return fail(error)};if !retained_clone_grant_admits(g,d){return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}
  let child=RetainedCloneGrant{maximum_depth:g.maximum_depth.saturating_sub(1),..g};
  if let Some(receiver)=self.receiver.get_mut(){if !receiver.terminal_is_empty()&&receiver.requires_original_control(){return receiver.close_original_step(original,&mut effect.nested(child))}}
  if !self.receiver.reserved()&&self.retired_header.is_none()&&self.retirement.requires_original_control(){return self.retirement.step_original(original,&mut effect.nested(child))}
  self.close_step(effect)}'''
sub("&&self.action.terminal_is_empty()&&self.original.is_none()}","&&self.action.terminal_is_empty()&&self.authority.is_none()&&self.original.is_none()}")
# cursor impls
conv=lambda call:"crate::retirement::erased_cursor_turn("+call+")"
sub("let turn=owner.close_original_birth_step(original,grant);match turn.result{Ok(step)=>RetirementStep::Progress(step.progress(),turn.work),Err(error)=>RetirementStep::Failure(error,turn.work)}}","crate::retirement::erased_cursor_turn(owner.close_original_birth_step(original,grant))}")
sub("let turn=owner.close_original_step(original,effect);match turn.result{Ok(step)=>RetirementStep::Progress(step.progress(),turn.work),Err(error)=>RetirementStep::Failure(error,turn.work)}}","crate::retirement::erased_cursor_turn(owner.close_original_step(original,effect))}")
sub("self.owner.as_mut().expect(\"retained original receiver\").begin_close();self.started=true;return RetirementStep::Progress(receipt(0),1)}let turn=self.owner.as_mut().expect(\"retained original receiver\").close_step(effect);match turn.result{Ok(step)=>RetirementStep::Progress(step.progress(),turn.work),Err(error)=>RetirementStep::Failure(error,turn.work)}}","self.owner.as_mut().expect(\"retained original receiver\").begin_close();self.started=true;return RetirementStep::Progress(receipt(0),1)}crate::retirement::erased_cursor_turn(self.owner.as_mut().expect(\"retained original receiver\").close_step(effect))}")
sub("let turn=Self::close_original_birth_step(self,original,grant);match turn.result{Ok(step)=>RetirementStep::Progress(step.progress(),turn.work),Err(error)=>RetirementStep::Failure(error,turn.work)}}","crate::retirement::erased_cursor_turn(Self::close_original_birth_step(self,original,grant))}")
sub("let turn=Self::close_original_step(self,original,effect);match turn.result{Ok(step)=>RetirementStep::Progress(step.progress(),turn.work),Err(error)=>RetirementStep::Failure(error,turn.work)}}","crate::retirement::erased_cursor_turn(Self::close_original_step(self,original,effect))}")
sub("let g=effect.grant();let turn=Self::close_step(self,&mut effect.nested(g));match turn.result{Ok(s)=>RetirementStep::Progress(s.progress(),turn.work),Err(error)=>RetirementStep::Failure(error,turn.work)}}","let g=effect.grant();crate::retirement::erased_cursor_turn(Self::close_step(self,&mut effect.nested(g)))}")
open(F,"w",encoding="utf-8").write("\n".join(L))
S=open(P,encoding="utf-8").read()
def r(a,b):
    global S
    assert S.count(a)==1,a;S=S.replace(a,b)
r("fn close_step(&mut self,grant:RetainedCloneGrant,actual:&mut Option<usize>)->crate::OriginalRetirementOutcome{match self{Self::Source(source)=>source.close_step(grant,actual),Self::Projection(projection)=>projection.close_step(grant,actual)}}","fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{match self{Self::Source(source)=>source.close_step(effect),Self::Projection(projection)=>projection.close_step(effect)}}")
r("retirement:Default::default(),effect:Default::default(),closing:false}}","retirement:Default::default(),action:Default::default(),authority:ManuallyDrop::new(Some(OriginalEffectAuthority::new(NativeSlotProducer::new()))),closing:false}}")
open(P,"w",encoding="utf-8").write(S)
