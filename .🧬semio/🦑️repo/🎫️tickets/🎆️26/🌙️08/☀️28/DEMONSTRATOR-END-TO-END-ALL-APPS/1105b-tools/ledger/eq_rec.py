p="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🌳️tree/🟰️equal/🎚️record/🦀️.rs"
s=open(p,encoding='utf8').read()
def rep(a,b,n=1):
    global s
    assert s.count(a)==n,(s.count(a),a[:100])
    s=s.replace(a,b)
rep("maximum_work_units:u64,g:RetainedCloneGrant,actual:&mut Option<usize>)->crate::RetainedWorkOutcome<RetainedEqualityStep,crate::OriginalRetirementFailure>{let mut work=0;let result:Result<_,crate::OriginalRetirementFailure>=(||{\n  if actual.is_some(){",
    "maximum_work_units:u64,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedEqualityStep,crate::OriginalRetirementFailure>{let g=effect.grant();let mut work:u128=0;let result:Result<_,crate::OriginalRetirementFailure>=(||{\n  if effect.actual().is_some(){")
rep("work=T::SHAPE_WORK;","work=u128::from(T::SHAPE_WORK);")
rep("work+=T::FIELD_METADATA_WORK;","work+=u128::from(T::FIELD_METADATA_WORK);",3)
rep("work=h.inline_work.ok_or_else(||invalid(\"typed inline work absent\"))?+T::FIELD_PROJECTION_WORK*2+3;","work=u128::from(h.inline_work.ok_or_else(||invalid(\"typed inline work absent\"))?+T::FIELD_PROJECTION_WORK*2+3);")
rep("maximum_work_units-T::FIELD_PROJECTION_WORK-2,g,actual);work=T::FIELD_PROJECTION_WORK+turn.work;","maximum_work_units-T::FIELD_PROJECTION_WORK-2,g,effect.actual());work=u128::from(T::FIELD_PROJECTION_WORK)+turn.work;")
rep("self.bytes=actual.unwrap_or(0);","self.bytes=effect.actual().unwrap_or(0);")
rep("if *actual!=expected||reported","if *effect.actual()!=expected||reported")
rep("body,maximum_work_units-3,RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-extra,maximum_depth:g.maximum_depth-1,..g},actual);work=1+turn.work;",
    "body,maximum_work_units-3,&mut effect.reborrow(RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-extra,maximum_depth:g.maximum_depth-1,..g},maximum_work_units-3)?);work=1+turn.work;")
rep(".close_step(RetainedCloneGrant{maximum_depth:g.maximum_depth-1,..g});work=1+turn.work;p=turn.result?.progress()}",
    ".close_step(&mut effect.reborrow(RetainedCloneGrant{maximum_depth:g.maximum_depth-1,..g},effect.maximum_work_units())?);let turn=complete(turn)?;work=1+turn.work;p=turn.result?.progress()}")
i=s.index(" fn close_step(&mut self,g:RetainedCloneGrant)->crate::RetainedWorkOutcome{let mut work=0;")
j=s.index(" fn terminal_is_empty(&self)->bool{self.active.is_none()&&self.header.is_none()")
new=""" fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{let g=effect.grant();let mut work:u128=0;let result=(||->Result<RetainedCloneStep,RetirementPollExit>{if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.close_demand(g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,d){return Ok(RetainedCloneStep::Progress(Default::default()))}if let Some(active)=self.active.as_mut(){if !active.terminal_is_empty(){let turn=RetirementPollExit::settled(active.close_step(&mut effect.reborrow(RetainedCloneGrant{maximum_depth:g.maximum_depth-1,..g},effect.maximum_work_units())?))?;work=turn.work+1;return Ok(RetainedCloneStep::Progress(turn.result?.progress()))}drop(self.active.take());self.bytes=0;work=2;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))}if self.header.is_some(){self.header=None;work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}self.index=0;self.bytes=0;self.pending=None;self.result=None;work=4;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})();RetirementPollExit::finish(result,work)}
"""
s=s[:i]+new+s[j:]
open(p,'w',encoding='utf8').write(s);print("ok")
