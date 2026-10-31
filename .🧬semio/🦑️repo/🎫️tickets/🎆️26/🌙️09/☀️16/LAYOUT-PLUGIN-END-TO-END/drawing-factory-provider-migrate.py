from pathlib import Path
b=Path("🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory")
paths=[b/"📦️owned/🧪️tests/🦀️.rs",b/"📦️boxed/🧪️tests/🦀️.rs",b/"🚼️construction/🧪️tests/🦀️.rs",b/"🧪️testing/🦀️.rs",b.parent/"🔗️arc-custody/🏭️factory/🧪️tests/🔐️custody/🦀️.rs"]
for p in paths:
 s=p.read_text()
 for old in [
 "fn prepare_close_state_step(&self,_:&mut Self::CloseState,_:crate::RetainedCloneGrant)->Result<crate::RetainedCloneStep,crate::ValueError>{Ok(crate::RetainedCloneStep::Complete(Default::default()))}",
 "fn prepare_close_state_step(&self,_:&mut Self::CloseState,_:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{Ok(RetainedCloneStep::Complete(Default::default()))}"]:
  if old in s:s=s.replace(old,old[:old.index("->")]+"->crate::RetainedWorkOutcome{crate::RetainedWorkOutcome{result:Ok(crate::RetainedCloneStep::Complete(Default::default())),work:0}}")
 if p==paths[0]:
  old="fn close_state_step(state:&mut bool,_:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{*state=true;Ok(RetainedCloneStep::Complete(Default::default()))}"
  new="fn close_state_step(state:&mut bool,_:RetainedCloneGrant)->crate::RetainedWorkOutcome{*state=true;crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:1}}"
 elif p==paths[1]:
  old="fn close_state_step(state:&mut bool,_:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{assert!(*state);Ok(RetainedCloneStep::Complete(Default::default()))}"
  new="fn close_state_step(state:&mut bool,_:RetainedCloneGrant)->crate::RetainedWorkOutcome{assert!(*state);crate::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}"
 else:
  start=s.index("fn close_state_step(");brace=s.index("{",start);end=brace+1;depth=1
  while depth:
   if s[end]=="{":depth+=1
   elif s[end]=="}":depth-=1
   end+=1
  old=s[start:end];new=old.replace("->Result<RetainedCloneStep,ValueError>","->crate::RetainedWorkOutcome")
  new=new[:new.index("{")+1]+"let mut work=0;let result=(||{"+new[new.index("{")+1:-1]+"})();crate::RetainedWorkOutcome{result,work}}"
  new=new.replace("drop(state.take());return", "drop(state.take());work+=1;return")
  new=new.replace("*state=None;return", "*state=None;work+=1;return")
  if "state.as_mut().unwrap().step(grant)" in new:new=new.replace("state.as_mut().unwrap().step(grant)","{let owner=state.as_mut().unwrap();let result=owner.step(grant);work+=owner.last_turn_work();result}")
  else:new=new.replace("owner.step(grant)","{let result=owner.step(grant);work+=owner.last_turn_work();result}")
 assert old in s,(p,old);s=s.replace(old,new,1);p.write_text(s)
p=b/"🧪️tests/🦀️.rs";s=p.read_text();old="fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { self.original.step(grant) }";assert old in s
s=s.replace(old,"fn close_step(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome{let result=self.original.step(grant);crate::RetainedWorkOutcome{result,work:self.original.last_turn_work()}}");p.write_text(s)
print("Five real provider preparation/state adapters and direct protocol owner migrated")
