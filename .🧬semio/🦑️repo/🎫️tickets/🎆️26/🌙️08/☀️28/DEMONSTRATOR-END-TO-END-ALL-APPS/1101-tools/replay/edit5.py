P='/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🧬️supersession/🦀️.rs'
s=open(P).read()
def sub(a,b,n=1):
    global s
    assert s.count(a)==n,(a[:70],s.count(a));s=s.replace(a,b)
sub('RetainedCloneStep,retirement::controlled::ControlledRetirement};','RetainedCloneStep,RetainedWorkOutcome,retirement::controlled::ControlledRetirement};')
sub('use semio_framework_job::{OriginalSourceBinding,','use semio_framework_job::{OriginalDomainReservation,OriginalSourceBinding,')
sub('enum Residue{Input(Input),Output(Output),Supersession(protocol::EffectiveSupersession),Failure(TransitionFailure)}','enum Residue{Input(Input),Output(Output),Supersession(protocol::EffectiveSupersession),Failure(TransitionFailure),Place(OriginalDomainReservation<TransitionFailure>)}')
sub('failure:Option<TransitionFailure>,binding:','failure:Option<TransitionFailure>,place:OriginalDomainReservation<TransitionFailure>,binding:')
sub('failure:None,binding:None,offset:0','failure:None,place:Default::default(),binding:None,offset:0')
sub('&&self.failure.is_none()&&self.binding.is_none()}','&&self.failure.is_none()&&self.place.terminal_is_empty()&&self.binding.is_none()}')
sub('NativeConstructionStepOutcome{result:outcome.result.map(|result|result.map(|_|RetainedCloneStep::Progress(p))),receipt_refusal:outcome.receipt_refusal,progress:p}.into_step_result()','NativeConstructionStepOutcome{result:outcome.result.map(|result|result.map(|_|RetainedCloneStep::Progress(p))),receipt_refusal:outcome.receipt_refusal,progress:p,work:outcome.work}.into_step_outcome().result')
sub('fn receive<T>(','''fn build<O>(cx:&mut StepContext<'_>,output:&mut Option<O>,d:RetirementDemand,construct:impl FnOnce()->Result<(O,RetainedCloneProgress),ValueError>)->Result<bool,OriginalTurnFailure>{admit_original_construction(cx,output,physical(d),|_|RetainedWorkOutcome{result:construct(),work:1})}
fn retire(cx:&mut StepContext<'_>,d:RetirementDemand,advance:impl FnOnce(RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>)->Result<(),OriginalTurnFailure>{close_original_retained_step(cx,d,|grant|RetainedWorkOutcome{result:advance(grant),work:1}).map(|_|())}
fn receive<T>(''')
n=s.count('admit_original_construction(cx,')
s=s.replace('admit_original_construction(cx,','build(cx,').replace('physical(d),|_|','d,||')
sub('fn build<O>(cx:&mut StepContext<\'_>,output:&mut Option<O>,d:RetirementDemand,construct:impl FnOnce()->Result<(O,RetainedCloneProgress),ValueError>)->Result<bool,OriginalTurnFailure>{build(cx,output,d,||RetainedWorkOutcome{result:construct(),work:1})}','fn build<O>(cx:&mut StepContext<\'_>,output:&mut Option<O>,d:RetirementDemand,construct:impl FnOnce()->Result<(O,RetainedCloneProgress),ValueError>)->Result<bool,OriginalTurnFailure>{admit_original_construction(cx,output,physical(d),|_|RetainedWorkOutcome{result:construct(),work:1})}')
# failure path
sub('  if self.failure.is_some(){\n   if self.failure.as_ref().unwrap().text.is_none(){','  if self.failure.is_some()||!self.place.terminal_is_empty(){\n   if self.failure.as_ref().is_some_and(|failure|failure.text.is_none()){')
sub('   if let Some(failure)=semio_framework_job::admit_original_domain_failure(&mut self.failure,cx)?{return Err(failure)}return Ok(None)','   if !self.place.ready(){self.place.prepare(cx)?;return Ok(None)}if !self.place.pending(){let original=&mut self.failure;self.place.with_body(cx,|_,slot|{*slot=original.take();Ok(())})?;}if let Some(failure)=semio_framework_job::admit_original_domain_failure(&mut self.place,cx)?{return Err(failure)}return Ok(None)')
# close
sub('close_original_retained_step(cx,d,|_|{self.retirement=None;','retire(cx,d,|_|{self.retirement=None;')
sub('close_original_retained_step(cx,d,|grant|self.retirement.as_mut().unwrap().step(grant))?;','close_original_retained_step(cx,d,|grant|{let owner=self.retirement.as_mut().unwrap();let result=owner.step(grant);RetainedWorkOutcome{result,work:owner.last_turn_work()}})?;')
sub('close_original_retained_step(cx,d,|_|{self.job=None;','retire(cx,d,|_|{self.job=None;')
sub('if self.input.is_some()||self.output.is_some()||self.failure.is_some(){let d=header::<Self>();close_original_retained_step(cx,d,|_|{self.retirement=Some(ControlledRetirement::retain(if let Some(failure)=self.failure.take(){Residue::Failure(failure)}','if self.input.is_some()||self.output.is_some()||self.failure.is_some()||!self.place.terminal_is_empty(){let d=header::<Self>();retire(cx,d,|_|{self.retirement=Some(ControlledRetirement::retain(if let Some(failure)=self.failure.take(){Residue::Failure(failure)}else if !self.place.terminal_is_empty(){Residue::Place(std::mem::take(&mut self.place))}')
sub('let d=header::<Self>();close_original_retained_step(cx,d,|_|{self.binding=None;self.done=false;Ok(RetainedCloneStep::Complete','let d=header::<Self>();retire(cx,d,|_|{self.binding=None;self.done=false;Ok(RetainedCloneStep::Complete')
open(P,'w').write(s)
print(n)
