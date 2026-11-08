p="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🌳️tree/🟰️equal/📋️sequence/🦀️.rs"
s=open(p,encoding='utf8').read()
def rep(a,b,n=1):
    global s
    assert s.count(a)==n,(s.count(a),a[:100])
    s=s.replace(a,b)
rep("work:u64,g:RetainedCloneGrant,actual:&mut Option<usize>)->crate::RetainedWorkOutcome<RetainedEqualityStep,crate::OriginalRetirementFailure>{if actual.is_some(){","work:u64,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedEqualityStep,crate::OriginalRetirementFailure>{let g=effect.grant();if effect.actual().is_some(){",4)
rep(",work-2,g,actual);",",work-2,effect);",4)
rep("fn close_step(&mut self,g:RetainedCloneGrant)->crate::RetainedWorkOutcome{self.close_step(g)}","fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{self.close_step(effect)}",2)
rep("fn close_step(&mut self,g:RetainedCloneGrant)->crate::RetainedWorkOutcome{RetainedSequenceEquality::close_step(self,g)}","fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{RetainedSequenceEquality::close_step(self,effect)}",2)
# inherent advance
rep("body:usize,maximum_work_units:u64,g:RetainedCloneGrant,actual:&mut Option<usize>)->crate::RetainedWorkOutcome<RetainedEqualityStep,crate::OriginalRetirementFailure>{let mut work:u64=0;let result:Result<_,crate::OriginalRetirementFailure>=(||{if actual.is_some(){",
    "body:usize,maximum_work_units:u64,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedEqualityStep,crate::OriginalRetirementFailure>{let g=effect.grant();let mut work:u128=0;let result:Result<_,crate::OriginalRetirementFailure>=(||{if effect.actual().is_some(){")
rep("work=work.checked_add(unit).ok_or_else","work=work.checked_add(u128::from(unit)).ok_or_else")
rep("equality_box::<SequenceChildBirth<T>>(maximum_work_units-1,g,actual);","equality_box::<SequenceChildBirth<T>>(maximum_work_units-1,g,effect.actual());")
rep("if *actual!=expected||","if *effect.actual()!=expected||")
rep("maximum_work_units-5,RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-fixed-size_of::<Option<bool>>()*2,maximum_depth:g.maximum_depth-1,..g},actual);work+=3+turn.work;",
    "maximum_work_units-5,&mut effect.reborrow(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-fixed-size_of::<Option<bool>>()*2,maximum_depth:g.maximum_depth-1,..g},maximum_work_units-5)?);work+=3+turn.work;")
rep(".close_step(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-fixed,maximum_depth:g.maximum_depth-1,..g});work+=1+turn.work;p=turn.result?.progress();",
    ".close_step(&mut effect.reborrow(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-fixed,maximum_depth:g.maximum_depth-1,..g},effect.maximum_work_units())?);let turn=complete(turn)?;work+=1+turn.work;p=turn.result?.progress();")
# inherent close_step
i=s.index(" pub fn close_step(&mut self,g:RetainedCloneGrant)->crate::RetainedWorkOutcome{let mut work=0;")
j=s.index(" pub fn terminal_is_empty(&self)->bool{self.active.is_none()")
new=""" pub fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{let g=effect.grant();let mut work:u128=0;let result=(||->Result<RetainedCloneStep,RetirementPollExit>{if !self.closing{return Err(invalid("typed sequence close has not begun").into())}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.close_demand(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if let Some(active)=self.active.as_mut(){if !active.terminal_is_empty(){let turn=RetirementPollExit::settled(active.close_step(&mut effect.reborrow(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-Self::fixed(),maximum_depth:g.maximum_depth-1,..g},effect.maximum_work_units())?))?;work=turn.work;let mut p=turn.result?.progress();p.copied_bytes+=Self::fixed();return Ok(RetainedCloneStep::Progress(p))}drop(self.active.take());work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))}
  if !self.inspection.is_empty(){let turn=self.inspection.close_step(1,g);work=turn.work;return Ok(RetainedCloneStep::Progress(turn.result?.progress()))}
  self.index=0;self.pending=None;self.result=None;work=3;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})();RetirementPollExit::finish(result,work)}
"""
s=s[:i]+new+s[j:]
rep("use super::*;\nstruct SequenceChildBirth","use super::*;\nuse semio_framework_value_retirement::retirement_contract::RetirementPollExit;\nstruct SequenceChildBirth")
open(p,'w',encoding='utf8').write(s);print("ok")
