import sys
P="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job/👷️worker/🎟️authority/🦀️.rs"
L=open(P,encoding='utf-8').read().split("\n")
def find(prefix,nth=0):
    idx=[i for i,l in enumerate(L) if l.lstrip().startswith(prefix)]
    if len(idx)<=nth: sys.exit(f"missing {prefix[:70]} nth={nth} have={len(idx)}")
    return idx[nth]
dele=set()
def delete(prefix,nth=0):
    i=find(prefix,nth)
    if i in dele: sys.exit(f"dup delete {prefix[:60]}")
    dele.add(i)
def setline(prefix,new,nth=0): L[find(prefix,nth)]=new
def sub(prefix,old,new,nth=0):
    i=find(prefix,nth)
    if L[i].count(old)!=1: sys.exit(f"sub count {L[i].count(old)} for {old[:80]}")
    L[i]=L[i].replace(old,new)

delete("fn turn(demand:RetirementDemand")
delete("preview_sequence: u64,")
delete("quarantined_outcome: JobOutcomeDescriptorSlot,")
delete("std::ptr::addr_of_mut!((*target).preview_sequence)")
delete("JobOutcomeDescriptorSlot::initialize_empty(std::ptr::addr_of_mut!((*target).quarantined_outcome))")
sub("pub enum OriginalWorkerAuthorityPhase","{QuarantinedOutcome,Outcome,OriginalFailure,AdmissionPage,BeginClose,Fault,Job,JobRelease,Parameters,Ledger,Empty}","{Outcome,OriginalFailure,BeginClose,Payload,Job,JobRelease,Parameters,Empty}")
# Authority impl
delete("pub fn initialization_copy_bytes()->usize{NativeWorkerAuthorityState")
delete("pub fn admission_page_publication_copy_bytes()->usize{0}",0)
delete("pub fn try_admit_owned(job:&mut Option<J>")
delete("pub fn initialization_demands(&self)->RetirementDemand{self.original")
delete("pub fn advance_initialization(&mut self,grant:RetainedCloneGrant)->RetainedCloneProgress{self.original")
sub("pub fn can_admit_prepared","original.outcome.is_empty()&&original.quarantined_outcome.is_empty()","original.outcome.is_empty()&&original.payload.outcome().is_none()&&original.payload.continuation().is_none()")
setline("pub fn acknowledge_outcome(&mut self,grant:RetainedCloneGrant)->RetainedCloneStep{self.original",
 " pub fn continuation(&self)->Option<&OriginalJobContinuation>{self.original.payload.continuation()}\n pub fn acknowledge_outcome(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.original.acknowledge_outcome(cx)}")
delete("pub fn drive_callback(&mut self,issued:RetainedCloneGrant)->bool{self.original")
delete("pub fn stage_callback(")
delete("pub fn run_staged_callback(")
# State impl
setline("pub fn admission_demand()->RetirementDemand{RetirementDemand{copy_bytes",
 " pub fn admission_demand()->RetirementDemand{RetirementDemand{capacity_bytes:size_of::<Self>(),depth:1,..Default::default()}}")
delete("pub fn initialization_copy_bytes()->usize{OriginalWorkerPayload")
delete("pub fn admission_page_publication_copy_bytes()->usize{0}",1)
delete("pub fn initialization_demands(&self)->RetirementDemand{self.payload")
delete("pub fn advance_initialization(&mut self,grant:RetainedCloneGrant)->RetainedCloneProgress{self.payload")
setline("pub fn outcome_descriptor(&self)->Option<&JobOutcomeDescriptor>{if self.retained_receipt_pending",
 " pub fn outcome_descriptor(&self)->Option<&JobOutcomeDescriptor>{if self.retained_receipt_pending{None}else if self.worker_fault{self.outcome.original()}else{self.payload.outcome()}}")
setline("pub fn acknowledge_outcome(&mut self,grant:RetainedCloneGrant)->RetainedCloneStep{if self.retained_receipt_pending",
""" pub fn acknowledge_outcome(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.retained_receipt_pending{return Ok(false)}
  if !self.outcome.is_empty(){let demand=self.outcome.retirement_demands();close_original_retained_step(cx,demand,|grant|self.outcome.close_step(grant))?;if self.outcome.is_empty(){self.worker_fault=false}return Ok(self.outcome.is_empty())}
  let(_,receiver)=self.payload.context_parts()?;receiver.acknowledge_outcome(cx)
 }""")
setline("pub fn is_resumable(&self)->bool{!self.retained_receipt_pending",
 " pub fn is_resumable(&self)->bool{!self.retained_receipt_pending&&self.outcome.is_empty()&&self.payload.outcome().is_none()&&self.payload.continuation().is_none()}")
sub("pub fn job_mut(&mut self)","||!self.quarantined_outcome.is_empty()","||self.payload.outcome().is_some()||self.payload.continuation().is_some()",1)
sub("pub fn child_custody_is_empty","&&!self.payload.has_unpublished_page()&&self.payload.fault_is_empty()&&!self.payload.ledger_is_present()&&self.outcome.is_empty()&&self.quarantined_outcome.is_empty()","&&self.payload.terminal_is_empty()&&self.outcome.is_empty()")
setline("pub fn close_phase(&self)->OriginalWorkerAuthorityPhase{if !self.quarantined_outcome",
 " pub fn close_phase(&self)->OriginalWorkerAuthorityPhase{if !self.outcome.is_empty(){return OriginalWorkerAuthorityPhase::Outcome}if self.step_fault.is_some()||self.step_fault_retirement.is_some()||self.panic_fault.is_some(){return OriginalWorkerAuthorityPhase::OriginalFailure}if self.close_stage==0{return OriginalWorkerAuthorityPhase::BeginClose}if !self.payload.terminal_is_empty(){return OriginalWorkerAuthorityPhase::Payload}match self.close_stage{1=>OriginalWorkerAuthorityPhase::Job,2=>OriginalWorkerAuthorityPhase::JobRelease,3=>OriginalWorkerAuthorityPhase::Parameters,_=>OriginalWorkerAuthorityPhase::Empty}}")
delete("fn terminal_authority_demands(&self)->Result<RetirementDemand,ValueError>{Ok(self.payload.ledger_demands())}")
delete("fn original_admission_page_demands(&self)->RetirementDemand{self.payload.unpublished_demands()}",0)
delete("fn close_original_admission_page(",0)
i=find("pub fn return_original_cancel_alias_step",1)
assert "original.return_alias_step(witness,grant).map(Some)" in L[i]
L[i]=L[i].replace("original.return_alias_step(witness,grant).map(Some)","original.return_alias_step(witness,grant).result.map(Some)")
# close_partial_source_step closure
sub("pub fn close_partial_source_step(&mut self,initializer:&mut impl WorkerJobInitializer<J>,cx:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{self.bind_initializer",
 '|remaining|{let step=unsafe{initializer.close_partial_step(target,remaining)}?;semio_framework_value::retained_clone::admit_retained_clone_close(remaining,step,initializer.partial_terminal_is_empty(),"native partial initializer closure")?;Ok(step)}',
 '|remaining|{let turn=unsafe{initializer.close_partial_step(target,remaining)}.and_then(|step|semio_framework_value::retained_clone::admit_retained_clone_close(remaining,step,initializer.partial_terminal_is_empty(),"native partial initializer closure"));let work=u64::from(turn.as_ref().is_ok_and(|step|step.progress().copied_items>0));semio_framework_value::RetainedWorkOutcome{result:turn,work}}')
# outcome() view
sub("pub fn outcome(&self)->Result<Option<JobOutcomeView<'_>>,ValueError>{if self.retained_receipt_pending","let Some(original)=self.outcome.original()else{return Ok(None)};","let Some(original)=(if self.worker_fault{self.outcome.original()}else{self.payload.outcome()})else{return Ok(None)};")
delete("pub fn drive_callback(&mut self,issued:RetainedCloneGrant)->bool{self.issued_retained")
# step_on_original_caller (State) -> line after header
i=find("pub fn step_on_original_caller(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{",1)
j=i+1
t=L[j]
a=t.index("let result=drive_step(");b=t.index("let after=cx.retained_progress();")
t=t[:a]+"let result=self.drive_original_caller(cx,config.site,config.stage);"+t[b:]
t=t.replace("let outcome=match result{Ok(outcome)=>outcome,Err(failure)=>","let terminal=match result{Ok(terminal)=>terminal,Err(failure)=>")
a=t.index("let terminal=outcome.as_ref()");b=t.index("self.retained_progress=match cumulative{Ok(total)=>total,Err(error)=>{self.retained_receipt_pending=false;")
t=t[:a]+t[b:]
L[j]=t
# new methods inserted before State close_step (InteractiveJob impl)
k=find("pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<WorkerJobCloseStep,OriginalTurnFailure>{",0)
newm=''' fn publish_original_fault(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  let detail=self.payload.fault().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original worker fault requires its prepared payload"))?;
  let Some(outcome)=JobOutcomeBorrow::admit_fault(cx,detail)?else{return Ok(false)};
  if self.outcome.retain(outcome.into_descriptor()).is_err(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original worker fault requires an empty outcome place").into())}
  self.worker_fault=true;self.fault_pending=false;Ok(true)
 }
 fn drive_original_caller(&mut self,cx:&mut StepContext<'_>,site:&'static str,stage:InteractiveStage)->Result<bool,OriginalTurnFailure>{
  if self.step_sequence==u64::MAX{self.fault_pending=true}
  if self.fault_pending{return self.publish_original_fault(cx)}
  let(_,receiver)=self.payload.context_parts()?;
  let job=self.job.original_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original callback requires its exact producer"))?;
  let terminal=match drive_step(job,cx,receiver,site,stage,&mut self.callback_verdict)?{
   OriginalJobReturn::Preparing|OriginalJobReturn::Pending=>false,
   OriginalJobReturn::Outcome(reply)=>{let mut input=Some(reply.into_descriptor());if !receiver.receive_outcome(cx,&mut input)?{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original receiver refused its genuine semantic result").into())}receiver.outcome().is_some_and(JobOutcomeDescriptor::is_terminal)},
   OriginalJobReturn::Continuation(value)=>{let mut input=Some(value);if !receiver.receive_continuation(cx,&mut input)?||input.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original receiver refused its genuine continuation").into())}true},
  };
  if !terminal&&self.callback_verdict.as_ref().is_some_and(|verdict|self.overruns.admit(verdict).is_terminal()){self.fault_pending=true}
  Ok(terminal)
 }'''
L[k]=newm+"\n"+L[k]
# close_step body lines
delete("if !self.quarantined_outcome.is_empty(){let step=worker_payload_step(self.quarantined_outcome")
setline("if !self.outcome.is_empty(){let step=worker_payload_step(self.outcome.close_step(grant))",
 "  if !self.outcome.is_empty(){let step=worker_work_step(cx,self.outcome.close_step(grant))?;return receive_original_worker_close_effects(cx,before,step)}")
setline("if self.payload.has_unpublished_page(){let step=self.close_original_admission_page(grant)",
 "  if self.close_stage!=0&&!self.payload.terminal_is_empty(){let step=match self.payload.close_step(cx)?{InteractiveJobCloseStep::Pending{progress}|InteractiveJobCloseStep::Complete{progress}=>WorkerJobCloseStep::Pending{progress},InteractiveJobCloseStep::Blocked=>WorkerJobCloseStep::Blocked};return receive_original_worker_close_effects(cx,before,step)}")

delete("if !self.payload.fault_is_empty(){let result=self.payload.close_fault(grant)")
sub("if self.close_stage==2{let progress=self.job.remove_terminal(grant)?.progress();","remove_terminal(grant)?.progress()","remove_terminal(grant).result?.progress()")
i=find("if self.payload.ledger_is_present(){if !self.payload.ledger_is_empty(){return Ok(WorkerJobCloseStep::Blocked)}let step=self.payload.close_ledger(grant)?;")
t=L[i];a=t.index("assert!(self.child_custody_is_empty())");L[i]="  "+t[a:]
i=find("if let Some(original)=self.step_fault_retirement.as_mut(){if original.terminal_is_empty(){drop(")
assert L[i].count("let step=original.close_step(child)?;")==1;L[i]=L[i].replace("let step=original.close_step(child)?;","let step=original.close_step(child).result?;")
i=find("let original=self.step_fault.take().expect(\"quoted original worker fault\")")
assert L[i].count("admit_owned_retirement(original,child){Ok(")==1;L[i]=L[i].replace("admit_owned_retirement(original,child){Ok(","admit_owned_retirement(original,child).result{Ok(")
i=find("return match cancel.close_step(grant){Ok(step)=>")
L[i]=L[i].replace("cancel.close_step(grant){Ok(step)=>","cancel.close_step(grant).result{Ok(step)=>")
delete("if !authority.quarantined_outcome.is_empty(){return Ok(authority.quarantined_outcome")
delete("if authority.payload.has_unpublished_page(){return Ok(authority.original_admission_page_demands())}")
setline("if !authority.payload.fault_is_empty(){return authority.payload.fault_demands();}","    if !authority.payload.terminal_is_empty(){return authority.payload.retirement_demands(maximum_copy_bytes)}")
setline("authority.terminal_authority_demands()","    Ok(Default::default())")
setline("fn worker_payload_step(step:RetainedCloneStep)->WorkerJobCloseStep{",L[find("fn worker_payload_step(step:RetainedCloneStep)->WorkerJobCloseStep{")]+"\nfn worker_work_step(cx:&mut StepContext<'_>,turn:semio_framework_value::RetainedWorkOutcome)->Result<WorkerJobCloseStep,OriginalTurnFailure>{match turn.result{Ok(step)=>Ok(worker_payload_step(step)),Err(error)=>{cx.consume_fuel(1);Err(OriginalTurnFailure::debit_original(error,cx))}}}")
# delete publish/drive private fns
a=find("fn publish_original_worker_fault<J>(");b=find("fn worker_job_authority_close_demands<J:InteractiveJob>(")
for x in range(a,b): dele.add(x)
# Owner
def delblock(prefix,nth=0):
    i=find(prefix,nth);dele.add(i)
    if not L[i].rstrip().endswith("}") or L[i].count("{")!=L[i].count("}"):
        j=i+1
        while L[j]!="    }": dele.add(j);j+=1
        dele.add(j)
delete("fn original_authority_admission_demand()->RetirementDemand{RetirementDemand{copy_bytes")
delblock("fn try_admit_owned(job:&mut Option<J>")
delete("fn write_storage(")
delete("fn pending_storage_initialization_demand()")
delete("fn admission_page_publication_copy_bytes()->usize{0}",0)
delete("fn original_admission_initialization_demands(")
delete("fn advance_original_admission_initialization(")
delete("fn original_admission_page_demands(&self)->RetirementDemand{self.payload.unpublished_demands()}",1)
delete("fn close_original_admission_page(",1)
delete("if self.payload.ledger_is_present(){return Ok(self.payload.ledger_demands());}")
delete("if authority.payload.ledger_is_present(){if !authority.payload.ledger_is_empty(){return Ok(WorkerJobCloseStep::Blocked)}let step=authority.payload.close_ledger(grant)?;")
sub("assert!(authority.job.is_empty()&&authority.params.is_none()","&&!authority.payload.has_unpublished_page()&&authority.payload.fault_is_empty()&&authority.outcome.is_empty()&&authority.quarantined_outcome.is_empty()","&&authority.payload.terminal_is_empty()&&authority.outcome.is_empty()")
out=[l for i,l in enumerate(L) if i not in dele]
open(P,"w",encoding="utf-8").write("\n".join(out))
print("written",len(dele))
