p="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🌳️tree/🟰️equal/🦀️.rs"
s=open(p,encoding='utf8').read()
def rep(a,b,n=1):
    global s
    assert s.count(a)==n,(s.count(a),a[:90])
    s=s.replace(a,b)
rep("fn advance(&mut self,left:&T,right:&T,body:usize,maximum_work_units:u64,grant:RetainedCloneGrant,actual:&mut Option<usize>)->crate::RetainedWorkOutcome<RetainedEqualityStep,crate::OriginalRetirementFailure>;",
    "fn advance(&mut self,left:&T,right:&T,body:usize,maximum_work_units:u64,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedEqualityStep,crate::OriginalRetirementFailure>;")
rep(" fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome;\n"," fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll;\n")
# inline
rep("fn advance(&mut self,left:&T,right:&T,body:usize,maximum_work_units:u64,g:RetainedCloneGrant,actual:&mut Option<usize>)->crate::RetainedWorkOutcome<RetainedEqualityStep,crate::OriginalRetirementFailure>{let mut work=0;let result:Result<_,crate::OriginalRetirementFailure>=(||{if actual.is_some(){",
    "fn advance(&mut self,left:&T,right:&T,body:usize,maximum_work_units:u64,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedEqualityStep,crate::OriginalRetirementFailure>{let g=effect.grant();let mut work:u128=0;let result:Result<_,crate::OriginalRetirementFailure>=(||{if effect.actual().is_some(){")
rep("work=T::WORK;Ok(RetainedEqualityStep","work=u128::from(T::WORK);Ok(RetainedEqualityStep")
rep("fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{let _=effect.grant();crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}",
    "fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{let _=effect.grant();crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})}")
# string
rep("fn advance(&mut self,left:&String,right:&String,body:usize,maximum_work_units:u64,g:RetainedCloneGrant,actual:&mut Option<usize>)->crate::RetainedWorkOutcome<RetainedEqualityStep,crate::OriginalRetirementFailure>{\n  if actual.is_some(){",
    "fn advance(&mut self,left:&String,right:&String,body:usize,maximum_work_units:u64,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedEqualityStep,crate::OriginalRetirementFailure>{let g=effect.grant();\n  if effect.actual().is_some(){")
rep("fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome{self.cursor.close_step(effect.maximum_work_units(),effect.grant())}",
    "fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{let turn=self.cursor.close_step(effect.maximum_work_units(),effect.grant());crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work})}")
# owner advance
rep("maximum_work_units:u64,g:RetainedCloneGrant,actual:&mut Option<usize>)->crate::RetainedWorkOutcome<RetainedEqualityStep,crate::OriginalRetirementFailure>{let mut work=0;let result:Result<_,crate::OriginalRetirementFailure>=(||{\n  if actual.is_some(){",
    "maximum_work_units:u64,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedEqualityStep,crate::OriginalRetirementFailure>{let g=effect.grant();let mut work:u128=0;let result:Result<_,crate::OriginalRetirementFailure>=(||{\n  if effect.actual().is_some(){")
rep("let turn=left.bind(&mut self.left,g);","let turn=left.bind(&mut self.left,effect);")
rep("let turn=right.bind(&mut self.right,g);","let turn=right.bind(&mut self.right,effect);")
rep("maximum_work_units-work,g);work+=checked.work;","maximum_work_units.saturating_sub(u64::try_from(work).unwrap_or(u64::MAX)),g);work+=checked.work;")
rep("let turn=self.cursor.advance(left.get(),right.get(),body.saturating_sub(Self::fixed()),maximum_work_units-checks,RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-extra,maximum_depth:g.maximum_depth-1,..g},actual);",
    "let turn=self.cursor.advance(left.get(),right.get(),body.saturating_sub(Self::fixed()),maximum_work_units-checks,&mut effect.reborrow(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-extra,maximum_depth:g.maximum_depth-1,..g},maximum_work_units-checks)?);")
# owner close
i=s.index(" pub fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome{let g=effect.grant();")
j=s.index(" pub fn terminal_is_empty(&self)->bool{self.closing&&")
new=""" pub fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{let g=effect.grant();let mut work:u128=0;let result=(|| -> Result<RetainedCloneStep,RetirementPollExit>{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  let d=self.close_demand(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if !self.cursor.terminal_is_empty(){let turn=RetirementPollExit::settled(self.cursor.close_step(&mut effect.reborrow(g,effect.maximum_work_units())?))?;work=turn.work;return Ok(RetainedCloneStep::Progress(turn.result?.progress()))}
  let turn=RetirementPollExit::settled(RetainedCloneBinding::close_one(if self.right.is_some(){&mut self.right}else{&mut self.left},effect))?;work=turn.work;let step=turn.result?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
 })();RetirementPollExit::finish(result,work)}

"""
s=s[:i]+new+s[j:]
# equality_box work
rep("Ok(Some((Box::new(B::birth()),RetainedCloneProgress{copied_items:1,..Default::default()}))),work}}","Ok(Some((Box::new(B::birth()),RetainedCloneProgress{copied_items:1,..Default::default()}))),work:u128::from(work)}}")
rep("retained_capacity_bytes:layout.size(),..Default::default()}))),work}}","retained_capacity_bytes:layout.size(),..Default::default()}))),work:u128::from(work)}}")
rep("use std::mem::size_of;","use std::mem::size_of;\nuse semio_framework_value_retirement::retirement_contract::RetirementPollExit;")
open(p,'w',encoding='utf8').write(s)
print("ok")
