import sys
F="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/⚠️refusal/🦀️.rs"
L=open(F,encoding="utf-8").read().split("\n")
def idx(prefix):
    for i,l in enumerate(L):
        if l.startswith(prefix): return i
    sys.exit("missing "+prefix)
def sub(old,new,count=1):
    for i,l in enumerate(L):
        if old in l:
            assert l.count(old)==count,(old,l.count(old))
            L[i]=l.replace(old,new);return
    sys.exit("nomatch "+old)
sub("fn begin_close(&mut self){self.closing=true;self.fields.begin_close()}","fn begin_close(&mut self){self.closing=true;self.fields.begin_close();}")
sub("1=>if self.fields.ready(){Ok(fixed(std::mem::size_of::<NativeRetirementNodeFields>()))}else","1=>if self.fields.ready(){self.fields.take_demand()}else")
i=idx("fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementOutcome")
L[i]='''fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{use crate::RetainedWorkPoll::Complete;use crate::retained_clone::retained_clone_grant_admits;let g=effect.grant();
  let fail=|error:ValueError|->crate::OriginalRetirementPoll{Complete(crate::RetainedWorkOutcome{result:Err(error.into()),work:0})};
  if self.requires_original_control(){return fail(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"retirement receiver requires its original close loan"))}
  if self.terminal_is_empty(){return Complete(crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Complete(Default::default())),work:0})}
  let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return fail(error)};if !retained_clone_grant_admits(g,d){return Complete(crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Progress(Default::default())),work:0})}
  if !self.fields.terminal_is_empty(){let mut child=match effect.reborrow(crate::owned::tree::child_grant(g,0),effect.maximum_work_units()){Ok(child)=>child,Err(error)=>return fail(error)};return self.fields.close_step(&mut child).map(|step|crate::RetainedCloneStep::Progress(step.progress()))}
  if !self.retirement.is_empty(){return self.retirement.step_granted(effect).map(|step|crate::RetainedCloneStep::Progress(step.progress()))}
  if self.pending.is_some(){let turn=self.retirement.begin_granted(&mut self.pending,effect);return Complete(crate::RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(crate::RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work})}
  if self.storage.is_some(){let turn=self.retirement.begin_granted(&mut self.storage,effect);return Complete(crate::RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(crate::RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work})}
  if self.output.is_some(){let turn=self.retirement.begin_granted(&mut self.output,effect);return Complete(crate::RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(crate::RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work})}
  crate::retained_clone::RetainedCloneBinding::close_one(&mut self.binding,effect)}'''
i=idx("fn close_original_birth_step(")
L[i]='''fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:crate::RetainedCloneGrant)->crate::OriginalRetirementPoll{use crate::RetainedWorkPoll::Complete;let fail=|error:ValueError|->crate::OriginalRetirementPoll{Complete(crate::RetainedWorkOutcome{result:Err(error.into()),work:0})};let idle=||->crate::OriginalRetirementPoll{Complete(crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Progress(Default::default())),work:0})};
  if let Some(g)=self.birth{if self.identity!=Some((original.ledger_identity(),original.retirement_recipient_identity())){return fail(ValueError::invariant("retirement close recipient changed"))}if g.restricted_by(grant)!=g{return idle()}let turn=crate::refill_original_failure_storage(original,g);if original.failure_storage_birth().terminal_is_empty(){self.birth=None}return turn}
  if self.fields.requires_original_birth(){if grant.maximum_depth==0{return idle()}return self.fields.close_original_birth_step(original,crate::owned::tree::child_grant(grant,0))}
  if self.retirement.requires_original_birth(){return self.retirement.step_original_birth(original,grant)}
  fail(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"retirement receiver has no original birth to settle"))}'''
i=idx("fn close_original_step(")
L[i]='''fn close_original_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{
  if self.requires_original_birth(){return crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"settle original birth before opening a close effect").into()),work:0})}
  if self.fields.requires_original_control(){let g=effect.grant();let mut child=match effect.reborrow(crate::owned::tree::child_grant(g,0),effect.maximum_work_units()){Ok(child)=>child,Err(error)=>return crate::RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(error.into()),work:0})};return self.fields.close_original_step(original,&mut child)}
  if self.retirement.requires_original_control(){return self.retirement.step_original(original,effect)}self.close_step(effect)}'''
open(F,"w",encoding="utf-8").write("\n".join(L))
