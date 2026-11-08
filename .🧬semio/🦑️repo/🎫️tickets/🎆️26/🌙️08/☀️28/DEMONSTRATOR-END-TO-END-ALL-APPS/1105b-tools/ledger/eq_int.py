p="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🌳️tree/🟰️equal/🌱️intrinsic/🦀️.rs"
s=open(p,encoding='utf8').read()
def rep(a,b,n=1):
    global s
    assert s.count(a)==n,(s.count(a),a[:100])
    s=s.replace(a,b)
rep("body:usize,work:u64,g:RetainedCloneGrant,actual:&mut Option<usize>)->RetainedWorkOutcome<crate::RetainedEqualityStep,crate::OriginalRetirementFailure>{if actual.is_some(){","body:usize,work:u64,effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<crate::RetainedEqualityStep,crate::OriginalRetirementFailure>{let g=effect.grant();if effect.actual().is_some(){")
rep("let turn=self.cursor.advance(left,right,body,work-2,g,actual);RetainedWorkOutcome{result:turn.result,work:turn.work+2}}","let turn=self.cursor.advance(left,right,body,work-2,effect);RetainedWorkOutcome{result:turn.result,work:turn.work+2}}")
rep(" fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome{let g=effect.grant();self.cursor.close_step(&mut effect.reborrow(g,effect.maximum_work_units()).expect(\"retirement child exceeds original effect\"))}",
    " fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{self.cursor.close_step(effect)}")
rep("maximum_work_units:u64,g:RetainedCloneGrant,actual:&mut Option<usize>)->RetainedWorkOutcome<ValueEqualityStep,crate::OriginalRetirementFailure>{\n  if actual.is_some(){","maximum_work_units:u64,effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome<ValueEqualityStep,crate::OriginalRetirementFailure>{let g=effect.grant();\n  if effect.actual().is_some(){")
rep("self.cursor.advance(left,right,body,maximum_work_units-1,g,actual);","self.cursor.advance(left,right,body,maximum_work_units-1,effect);")
i=s.index(" pub fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetainedWorkOutcome{let g=effect.grant();")
j=s.index(" pub fn terminal_is_empty(&self)->bool{self.closing&&self.binding.is_none()")
new=""" pub fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{let g=effect.grant();let done=|result:Result<RetainedCloneStep,ValueError>,work:u128|crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:result.map_err(Into::into),work});
  if !self.closing{return done(Err(invalid()),0)}if self.terminal_is_empty(){return done(Ok(RetainedCloneStep::Complete(Default::default())),0)}
  let d=match self.next_close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return done(Err(error),0)};if !permits(g,d){return done(Ok(RetainedCloneStep::Progress(Default::default())),0)}
  if !self.cursor.terminal_is_empty(){return self.cursor.close_step(effect)}self.binding=None;self.result=None;done(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),1)
 }
"""
s=s[:i]+new+s[j:]
rep("let g=effect.grant();use crate::retirement::RetirementStep;let turn=self.0.close_step(&mut effect.reborrow(g,effect.maximum_work_units()).expect(\"retirement child exceeds original effect\"));match turn.result{Err(error)=>RetirementStep::Failure(error.into(),turn.work),Ok(RetainedCloneStep::Complete(p))if p==RetainedCloneProgress::default()=>RetirementStep::Complete(turn.work),Ok(step)if step.progress()!=RetainedCloneProgress::default()||turn.work!=0=>RetirementStep::Progress(step.progress(),turn.work),Ok(_)=>RetirementStep::BudgetExhausted}}",
    "use crate::retirement::RetirementStep;let turn=match self.0.close_step(effect){crate::RetainedWorkPoll::Complete(turn)=>turn,crate::RetainedWorkPoll::Pending{work}=>return RetirementStep::Failure(invalid().into(),u64::try_from(work).unwrap_or(u64::MAX))};let work=u64::try_from(turn.work).unwrap_or(u64::MAX);match turn.result{Err(error)=>RetirementStep::Failure(error.into(),work),Ok(RetainedCloneStep::Complete(p))if p==RetainedCloneProgress::default()=>RetirementStep::Complete(work),Ok(step)if step.progress()!=RetainedCloneProgress::default()||work!=0=>RetirementStep::Progress(step.progress(),work),Ok(_)=>RetirementStep::BudgetExhausted}}")
open(p,'w',encoding='utf8').write(s);print("ok")
