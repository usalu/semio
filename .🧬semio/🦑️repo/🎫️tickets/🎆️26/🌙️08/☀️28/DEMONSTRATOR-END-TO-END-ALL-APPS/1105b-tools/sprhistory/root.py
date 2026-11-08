P="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs"
s=open(P,encoding="utf-8").read()
def once(old,new,count=1):
    global s
    n=s.count(old)
    assert n==count,(n,old[:100])
    s=s.replace(old,new)
once("use semio_framework_value_native_owned::native_decoding::allocation::{NativeForwardedDecodeReceipt,NativeDecodeAllocationIssuer};","use semio_framework_value_native_owned::native_decoding::allocation::NativeForwardedDecodeReceipt;\nuse semio_framework_value_native_owned::native_decoding::{NativeAllocationAuthority,NativeAllocationAuthorityPort,NativeAllocationRefusal,NativeAllocationReply,NativeAllocationReplyComplete,OriginalSourceTurnIdentity,OriginalWorkLoan};")
once("use semio_framework_value::{admit_owned_retirement, owned_retirement_birth_bytes, ErasedSnapshotRetirement, RetainedCloneGrant,","use semio_framework_value::{admit_owned_retirement, close_factory_ticket_controlled, owned_retirement_birth_bytes, ErasedSnapshotRetirement, OriginalRetirementFailure, RetainedAllocationEffect, RetainedWorkPoll, RetainedCloneGrant,")
once("    last_work: u64,\n    last_native_work:u64,","    last_work: u128,\n    last_native_work:u128,")
once("    allocation_issuer:NativeDecodeAllocationIssuer,\n    allocation_budget:NativeAllocationBudget,\n","    allocation:ManuallyDrop<Option<NativeAllocationAuthority<HistoryAllocationPort>>>,\n")
once("            allocation_issuer:NativeDecodeAllocationIssuer::new(),\n            allocation_budget:NativeAllocationBudget::default(),\n","            allocation:ManuallyDrop::new(Some(NativeAllocationAuthority::new(HistoryAllocationPort{budget:NativeAllocationBudget::default()}))),\n")
once("pub fn last_turn_work(&self) -> u64 { self.last_work }","pub fn last_turn_work(&self) -> u128 { self.last_work }")
once("pub fn last_turn_native_work(&self)->u64{self.last_native_work}","pub fn last_turn_native_work(&self)->u128{self.last_native_work}")
once("from_causes(Some(primary),None,receipt).with_retained_progress(progress)","from_causes(Some(primary),None,receipt,None).with_retained_progress(progress)")
lines=s.split("\n")
def idx(prefix):
    r=[i for i,l in enumerate(lines) if l.startswith(prefix)]
    assert len(r)==1,(prefix,len(r))
    return r[0]
# admit_owned + close_box
a=idx("fn admit_owned<T: RetireOwned>")
b=idx("fn recipient_demands(")
new_admit='''fn admit_owned<T: RetireOwned>(pending: &mut Option<T>, active: &mut Option<Box<dyn ErasedSnapshotRetirement>>, effect: &mut RetainedAllocationEffect<'_>, work: &mut u128) -> Result<RetainedCloneStep, ValueError> {
    let grant = effect.grant();
    if active.is_some() {
        return Err(invariant("original History retirement admission requires an empty child slot"));
    }
    let demand = birth_demands(pending)?;
    if pending.is_none() {
        return Err(invariant("original History retirement admission lost its owner"));
    }
    let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth.saturating_sub(1), ..grant };
    let turn = admit_owned_retirement(pending, &mut effect.nested(child));
    *work = turn.work;
    match turn.result {
        Ok(Some((owner, progress))) => {
            *active = Some(owner);
            admit_retained_clone_progress(child, progress, "original History child birth")?;
            if progress.retained_capacity_bytes != demand.capacity_bytes {
                return Err(invariant("original History child birth receipt differs from its quote"));
            }
            Ok(RetainedCloneStep::Progress(progress))
        }
        Ok(None) => Ok(RetainedCloneStep::Progress(Default::default())),
        Err(error) => Err(error),
    }
}

fn polled(poll:RetainedWorkPoll<RetainedCloneStep,OriginalRetirementFailure>,work:&mut u128)->Result<RetainedCloneStep,OriginalRetirementFailure>{
 *work=poll.work();
 match poll{RetainedWorkPoll::Pending{..}=>Ok(RetainedCloneStep::Progress(Default::default())),RetainedWorkPoll::Complete(turn)=>turn.result}
}

/// 🧰️ Runs one funded decoder turn: the closure's children record their performed work in the turn effect and only the owed remainder of the claimed work is added, once.
fn funded<T>(control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant,last_work:&mut u128,run:impl FnOnce(&mut NativeDecodeControl<'_>,&mut RetainedAllocationEffect<'_>,&mut u128)->T)->Result<Option<T>,ValueError>{
 control.with_effect(grant,RetainedHistorySourceDecode::work_demand(grant),|control,effect|{
  let start=effect.performed_work();
  let output=run(control,effect,&mut *last_work);
  *last_work=settle_since(effect,start,*last_work)?;
  Ok::<_,ValueError>(output)
 })?.transpose()
}

fn close_recipient(control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant,last_work:&mut u128)->Result<RetainedCloneStep,OriginalRetirementFailure>{
 match control.with_effect(grant,RetainedHistorySourceDecode::work_demand(grant),|control,effect|control.close_retirement_recipient(effect)){
  Ok(Some(poll))=>polled(poll,last_work),
  Ok(None)=>Ok(RetainedCloneStep::Progress(Default::default())),
  Err(error)=>Err(OriginalRetirementFailure::from_work_refusal(error)),
 }
}

fn close_box(control:&mut NativeDecodeControl<'_>,slot:&mut Option<Box<dyn ErasedSnapshotRetirement>>,grant:RetainedCloneGrant,last_work:&mut u128)->Result<RetainedCloneStep,OriginalRetirementFailure>{
 let child=if slot.as_ref().is_some_and(|owner|!owner.terminal_is_empty()){RetainedCloneGrant{maximum_depth:grant.maximum_depth.saturating_sub(1),..grant}}else{grant};
 match control.with_effect(grant,RetainedHistorySourceDecode::work_demand(grant),|control,effect|control.with_retirement(child,|original|Ok::<_,ValueError>(close_factory_ticket_controlled(slot,&mut effect.nested(child),original)))){
  Ok(Some(Ok(poll)))=>polled(poll,last_work),
  Ok(Some(Err(error)))=>Err(OriginalRetirementFailure::from_work_refusal(error)),
  Ok(None)=>Ok(RetainedCloneStep::Progress(Default::default())),
  Err(error)=>Err(OriginalRetirementFailure::from_work_refusal(error)),
 }
}
'''.split("\n")
lines[a:b]=new_admit
# with_control
a=idx("fn with_control<T>(")
b=idx("impl RetainedHistoryDecode {")
new_wc='''/// 🪙️ The allocation port admits through the cumulative budget and the caller's turn without owning either.
struct HistoryAllocationPort{budget:NativeAllocationBudget}
struct HistoryAllocationContext<'turn>{turn:&'turn dyn RetainedHistoryTurn,grant:RetainedCloneGrant,closing:bool,hop:bool}
impl NativeAllocationAuthorityPort for HistoryAllocationPort{
    type Context<'turn,'source>=HistoryAllocationContext<'turn> where 'source:'turn;
    fn validate_context<'turn,'source>(&self,_:&Self::Context<'turn,'source>)->Result<(),ValueError> where 'source:'turn{Ok(())}
    fn source_turn<'turn,'source>(&self,_:&Self::Context<'turn,'source>)->Option<OriginalSourceTurnIdentity> where 'source:'turn{None}
    fn receive<'reply,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,event:NativeAllocationEvent,reply:NativeAllocationReply<'reply>)->Result<NativeAllocationReplyComplete<'reply>,ValueError> where 'source:'turn{
        if !context.hop{self.budget.begin_hop();context.hop=true}
        let may_quote=!matches!(event,NativeAllocationEvent::Quote(_)|NativeAllocationEvent::Admit(_))||!context.turn.should_yield()&&(context.closing||!context.turn.is_cancelled());
        let verdict=self.budget.receive(event,context.grant,may_quote).map_err(NativeAllocationRefusal::from);
        Ok(reply.complete(verdict))
    }
    fn recover<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,_:NativeAllocationEvent,reply:NativeAllocationReply<'reply>)->Result<NativeAllocationReplyComplete<'reply>,ValueError> where 'source:'turn{Ok(reply.pending())}
}

fn with_control<T>(
    receipt: &mut Option<NativeForwardedDecodeReceipt>,
    recipient: &mut NativeDecodeRetirementRecipient,
    allocation:&mut Option<NativeAllocationAuthority<HistoryAllocationPort>>,
    maximum: usize,
    turn: &dyn RetainedHistoryTurn,
    grant:RetainedCloneGrant,
    closing: bool,
    work: &mut OriginalWorkLoan<'_>,
    observed:&mut u128,
    run: impl FnOnce(&mut NativeDecodeControl<'_>) -> T,
) -> Result<T, ValueError> {
    let mut progress = |_| !turn.should_yield() && (closing || !turn.is_cancelled());
    let authority=allocation.as_mut().ok_or_else(||invariant("original History allocation authority was returned"))?;
    let mut context=HistoryAllocationContext{turn,grant,closing,hop:false};
    authority.with_send_loan(&mut context,|allocation_loan|{
        let mut control = if let Some(original) = receipt.take() {
            match NativeForwardedDecodeControl::rebind(original, &mut progress, allocation_loan, recipient) {
                Ok(control) => control,
                Err((error, original)) => {
                    *receipt = Some(original);
                    return Err(error);
                }
            }
        } else {
            let mut control = NativeDecodeControl::new_forwarded(maximum, &mut progress, allocation_loan);
            control.install_retirement_recipient(recipient)?;
            control
        };
        let before=control.actual_work();
        let result = control.with_work_recipient(work,|original|original.scoped_stage(|native|Ok::<_,ValueError>(run(native))))?;
        *observed=control.actual_work().saturating_sub(before);
        *receipt = Some(control.detach().unwrap_or_else(|_| panic!("original History decode continuation violated canonical control invariants")));
        Ok(result)
    })?
}
'''.split("\n")
lines[a:b]=new_wc
s="\n".join(lines)
once("with_control(&mut self.receipt,&mut self.recipient,&mut self.allocation_issuer,&mut self.allocation_budget,self.maximum,","with_control(&mut self.receipt,&mut self.recipient,&mut self.allocation,self.maximum,")
once("with_control(&mut self.receipt, &mut self.recipient, &mut self.allocation_issuer, &mut self.allocation_budget, self.maximum,","with_control(&mut self.receipt, &mut self.recipient, &mut self.allocation, self.maximum,",4)
once("        self.closed && self.allocation_budget.terminal_is_empty() &&","        self.closed && self.allocation_idle() &&")
once("    pub fn terminal_is_empty(&self) -> bool {","    fn allocation_idle(&self)->bool{self.allocation.as_ref().is_none_or(|authority|!authority.has_pending_transaction()&&authority.inspect_port(|port|port.budget.terminal_is_empty()))}\n\n    pub fn terminal_is_empty(&self) -> bool {")
once("        if self.terminal_is_empty() {\n            unsafe {\n                ManuallyDrop::drop(&mut self.decoder);","        if self.terminal_is_empty() {\n            if let Some(authority)=self.allocation.take(){if let Err(authority)=authority.into_port(){std::mem::forget(authority)}}\n            unsafe {\n                ManuallyDrop::drop(&mut self.decoder);")
# refill arms
once("Ok(Ok(outcome))=>outcome,Ok(Err(error))=>semio_framework_value::RetainedWorkOutcome{result:Err(error),work:0},Err(error)=>semio_framework_value::RetainedWorkOutcome{result:Err(error.into()),work:0}};","Ok(Ok(RetainedWorkPoll::Complete(outcome)))=>outcome,Ok(Ok(RetainedWorkPoll::Pending{work}))=>semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work},Ok(Err(error))=>semio_framework_value::RetainedWorkOutcome{result:Err(error),work:0},Err(error)=>semio_framework_value::RetainedWorkOutcome{result:Err(error.into()),work:0}};")
lines=s.split("\n")
def idx(prefix):
    r=[i for i,l in enumerate(lines) if l.startswith(prefix)]
    assert len(r)==1,(prefix,len(r))
    return r[0]
# step_result block
a=idx("    fn step_result<")
b=idx("    pub fn close_step<")
assert lines[b-1]=="" and lines[b-2]=="    }",(lines[b-2][:40])
new_step='''    fn step_result<S: RetainedWireByteSource + ?Sized>(&mut self, source: &S, turn: &dyn RetainedHistoryTurn, grant: RetainedCloneGrant, history: &mut Option<HistoryLog>, work: &mut OriginalWorkLoan<'_>) -> Result<RetainedHistorySourceStep, OriginalRetirementFailure> {
        self.last_work=0;
        if self.closed {
            return Ok(RetainedHistorySourceStep::Closed);
        }
        if self.closing || self.fault.is_some() {
            return Ok(RetainedHistorySourceStep::Fault);
        }
        if turn.should_yield() || turn.is_cancelled() || grant.maximum_items == 0 {
            return Ok(RetainedHistorySourceStep::Blocked);
        }
        if !permits(grant, self.demands(grant.maximum_copy_bytes)?) {
            return Ok(RetainedHistorySourceStep::Blocked);
        }
        if self.failure_storage_needed(){return self.refill_failure_storage(turn,grant,work,false).result.map(|_|RetainedHistorySourceStep::Progress)}
        let finish_storage=self.admitted&&self.decoder.is_none()&&!self.recipient.terminal_is_empty();
        let last_work=&mut self.last_work;
        let decoder = &mut self.decoder;
        let admitted = &mut self.admitted;
        let quote = &mut self.demand;
        let (range, limits, persisted_document) = (self.range, self.limits, self.persisted_document);
        let (result, issued) = with_control(&mut self.receipt, &mut self.recipient, &mut self.allocation, self.maximum, turn, grant, false, work, &mut self.last_native_work, |control| {
            if control.has_retirement_owner()||finish_storage {
                let raw = close_recipient(control,grant,last_work);
                let progress = match &raw {
                    Ok(step) => step.progress(),
                    Err(error) => error.retained_progress(),
                };
                return (raw.map(|_| RetainedHistorySourceStep::Progress).map_err(RetainedHistoryOpMetaFault::from), Some((grant, progress)));
            }
            let funded = funded(control, grant, last_work, |control, effect, last_work| {
                let result = if !*admitted {
                    *last_work=1;
                    match RetainedHistorySourceDecode::admit(source, range, limits, persisted_document, control, effect) {
                        Ok(Some(owner)) => {
                            **decoder = Some(owner);
                            *admitted = true;
                            Ok(RetainedHistorySourceStep::Progress)
                        }
                        Ok(None) => Ok(RetainedHistorySourceStep::Blocked),
                        Err(error) => Err(error),
                    }
                } else if let Some(owner) = decoder.as_mut() {
                    if owner.terminal_is_empty() {
                        decoder.take();
                        *last_work=1;
                        Ok(RetainedHistorySourceStep::Closed)
                    } else if owner.is_ready() {
                        let ready=owner.take_ready_into(history, effect);
                        *last_work=owner.last_turn_work();
                        ready.map(|ready| if ready { RetainedHistorySourceStep::Ready } else { RetainedHistorySourceStep::Blocked })
                    } else {
                        let step=owner.step(source, control, effect);
                        *last_work=owner.last_turn_work();
                        step
                    }
                } else {
                    *last_work=1;
                    Ok(RetainedHistorySourceStep::Ready)
                };
                let issued = decoder.as_mut().and_then(|owner| owner.take_receipt());
                if let Some(owner) = decoder.as_ref() {
                    if let Ok(next) = owner.demands(source, control, grant.maximum_copy_bytes) {
                        *quote = next;
                    }
                }
                (result, issued)
            });
            match funded {
                Ok(Some(pair)) => pair,
                Ok(None) => (Ok(RetainedHistorySourceStep::Blocked), None),
                Err(error) => (Err(error.into()), None),
            }
        })?;
        let step = match result {
            Ok(step) => step,
            Err(RetainedHistoryOpMetaFault::Value(primary)) => {
                self.closing = true;
                return Err(receive_failure(issued, grant, primary).into());
            }
            Err(error) => {
                *self.fault = Some(error);
                self.closing = true;
                RetainedHistorySourceStep::Fault
            }
        };
        let progress = receive(issued, grant)?;
        self.issued = issued;
        if self.fault.is_some() {
            return Ok(RetainedHistorySourceStep::Fault);
        }
        if self.admitted && self.decoder.is_none() && self.recipient.terminal_is_empty() && issued.is_none() {
            self.receipt.take();
            self.closed = true;
            self.issued = Some((grant, RetainedCloneProgress { copied_items: 1, ..Default::default() }));
            return Ok(RetainedHistorySourceStep::Ready);
        }
        Ok(if progress.copied_items > 0 { RetainedHistorySourceStep::Progress } else { step })
    }'''.split("\n")
lines[a:b-1]=new_step
a=idx("    fn close_step_result<")
b=idx("    fn allocation_idle(")
assert lines[b-1]=="" and lines[b-2]=="    }",(lines[b-2][:40])
new_close='''    fn close_step_result<S: RetainedWireByteSource + ?Sized>(&mut self, source: &S, turn: &dyn RetainedHistoryTurn, grant: RetainedCloneGrant, work: &mut OriginalWorkLoan<'_>) -> Result<RetainedCloneStep, OriginalRetirementFailure> {
        self.last_work=0;
        if self.terminal_is_empty() {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        self.closing = true;
        if turn.should_yield() || grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if !permits(grant, demand) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        if self.failure_storage_needed(){return self.refill_failure_storage(turn,grant,work,true).result}
        let last_work=&mut self.last_work;
        if self.active.is_some() || self.fault.is_some() || self.inner_fault.is_some() || self.decoder.is_none() && self.partial.is_some() {
            let active = &mut self.active;
            let fault = &mut self.fault;
            let inner_fault = &mut self.inner_fault;
            let partial = &mut self.partial;
            let raw = with_control(&mut self.receipt, &mut self.recipient, &mut self.allocation, self.maximum, turn, grant, true, work, &mut self.last_native_work, |control| {
                if active.is_some(){return close_box(control,active,grant,last_work)}
                let funded = funded(control, grant, last_work, |control, effect, last_work| {
                    control.charge(demand.capacity_bytes).map_err(OriginalRetirementFailure::from_work_refusal)?;
                    let admitted = if fault.is_some(){admit_owned(fault,active,effect,last_work)}else if inner_fault.is_some(){admit_owned(inner_fault,active,effect,last_work)}else{admit_owned(partial,active,effect,last_work)};
                    admitted.map_err(OriginalRetirementFailure::from)
                });
                match funded {
                    Ok(Some(raw)) => raw,
                    Ok(None) => Ok(RetainedCloneStep::Progress(Default::default())),
                    Err(error) => Err(OriginalRetirementFailure::from_work_refusal(error)),
                }
            })?;
            let progress=raw.as_ref().map(|step|step.progress()).unwrap_or_else(|error|error.retained_progress());self.issued=Some((grant,progress));return raw.map_err(Into::into);
        }
        if self.decoder.is_some() {
            let decoder = &mut self.decoder;
            let partial = &mut self.partial;
            let fault = &mut self.inner_fault;
            let quote = &mut self.demand;
            let (result, issued) = with_control(&mut self.receipt, &mut self.recipient, &mut self.allocation, self.maximum, turn, grant, true, work, &mut self.last_native_work, |control| {
                let funded = funded(control, grant, last_work, |control, effect, last_work| {
                    let owner = decoder.as_mut().unwrap();
                    owner.cancel();
                    let result = if owner.terminal_is_empty() {
                        *last_work=1;
                        Ok(RetainedHistorySourceStep::Closed)
                    } else if owner.fault().is_some() {
                        let taken=owner.take_fault_into(fault, effect);
                        *last_work=owner.last_turn_work();
                        taken.map(|_| RetainedHistorySourceStep::Progress)
                    } else if owner.partial_is_receivable() {
                        let taken=owner.take_partial_into(partial, effect);
                        *last_work=owner.last_turn_work();
                        taken.map(|_| RetainedHistorySourceStep::Progress)
                    } else {
                        let step=owner.close_step(source, control, effect);
                        *last_work=owner.last_turn_work();
                        step
                    };
                    let issued = owner.take_receipt();
                    if let Ok(next) = owner.demands(source, control, grant.maximum_copy_bytes) {
                        *quote = next;
                    }
                    (result, issued)
                });
                match funded {
                    Ok(Some(pair)) => pair,
                    Ok(None) => (Ok(RetainedHistorySourceStep::Blocked), None),
                    Err(error) => (Err(error.into()), None),
                }
            })?;
            match result {
                Err(RetainedHistoryOpMetaFault::Value(primary)) => return Err(receive_failure(issued, grant, primary).into()),
                Err(error) => { *self.fault = Some(error); },
                Ok(_) => {}
            }
            let progress = receive(issued, grant)?;
            if self.decoder.as_ref().is_some_and(RetainedHistorySourceDecode::terminal_is_empty) {
                self.decoder.take();
                if issued.is_none() {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
                }
            }
            return Ok(RetainedCloneStep::Progress(progress));
        }
        if !self.recipient.terminal_is_empty() {
            let raw = with_control(&mut self.receipt, &mut self.recipient, &mut self.allocation, self.maximum, turn, grant, true, work, &mut self.last_native_work, |control| close_recipient(control,grant,last_work))?;
            let progress=raw.as_ref().map(|step|step.progress()).unwrap_or_else(|error|error.retained_progress());self.issued=Some((grant,progress));return raw.map_err(Into::into);
        }
        let paid = with_control(&mut self.receipt, &mut self.recipient, &mut self.allocation, self.maximum, turn, grant, true, work, &mut self.last_native_work, |control| funded(control, grant, last_work, |_, _, last_work| { *last_work=1; }))?;
        match paid {
            Ok(Some(())) => {}
            Ok(None) => return Ok(RetainedCloneStep::Progress(Default::default())),
            Err(error) => return Err(OriginalRetirementFailure::from_work_refusal(error)),
        }
        self.receipt.take();
        self.closed = true;
        let progress=RetainedCloneProgress{copied_items:1,..Default::default()};self.issued=Some((grant,progress));Ok(RetainedCloneStep::Complete(progress))
    }'''.split("\n")
lines[a:b-1]=new_close
s="\n".join(lines)
open(P,"w",encoding="utf-8").write(s)
print("ok")
