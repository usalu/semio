from pathlib import Path
p=Path("🧰️framework/🔨️modules/🌱️value/📋️list/🔺️edit/🔑️named/🦀️.rs")
s=p.read_text()
start=s.index(" pub fn advance(&mut self,source:");brace=s.index("{",start);end=brace+1;depth=1
while depth:
 if s[end]=="{":depth+=1
 elif s[end]=="}":depth-=1
 end+=1
old=s[start:end];body=old[old.index("{")+1:-1]
a="let step=self.plan.as_mut().unwrap().advance(Self::child(grant),|kind,ordinal|match kind{IndexedEditKind::Removed=>removed[ordinal],IndexedEditKind::Modified=>modified[ordinal],IndexedEditKind::Added=>source.added_index(ordinal)})?;"
b="let step=take_work!(work,self.plan.as_mut().unwrap().advance(Self::child(grant),|kind,ordinal|match kind{IndexedEditKind::Removed=>crate::RetainedWorkOutcome{result:Ok(removed[ordinal]),work:1},IndexedEditKind::Modified=>crate::RetainedWorkOutcome{result:Ok(modified[ordinal]),work:1},IndexedEditKind::Added=>source.added_index(ordinal)}));"
assert a in body;body=body.replace(a,b)
body=body.replace("IndexedEditStep::Refused(fault,_)=>self.refuse(NamedEditFault::Coordinates(fault),progress)","IndexedEditStep::Refused(fault,_)=>{work+=1;self.refuse(NamedEditFault::Coordinates(fault),progress)}").replace("IndexedEditStep::Admitted(_)=>{self.phase=10;","IndexedEditStep::Admitted(_)=>{work+=1;self.phase=10;")
for a,b in [
 ("source.observe_key(a.0,a.1,grant)?","take_work!(work,source.observe_key(a.0,a.1,grant))"),
 ("source.observe_key(b.0,b.1,grant)?","take_work!(work,source.observe_key(b.0,b.1,grant))"),
 ("self.compare.advance(a,b,child)?","take_work!(work,self.compare.advance(a,b,child))"),
 ("if order.is_some(){self.compare=Default::default();}","if order.is_some(){self.compare=Default::default();work+=1;}"),
 ("0|4|6|10=>{let count=","0|4|6|10=>{let count="),
 ("let mut output=Vec::new();output.try_reserve_exact(count)","let mut output=Vec::new();work+=1;output.try_reserve_exact(count)"),
 ("1|11=>{let count=","1|11=>{work+=1;let count="),
 ("if let Some(order)=observation{self.sort.advance(Some(order))","if let Some(order)=observation{work+=1;self.sort.advance(Some(order))"),
 ("HeapSortWork::Swap{left,right}=>{if","HeapSortWork::Swap{left,right}=>{work+=1;if"),
 ("HeapSortWork::Advance=>self.sort.advance(None).map_err(|_|invalid())?,","HeapSortWork::Advance=>{work+=1;self.sort.advance(None).map_err(|_|invalid())?;},"),
 ("HeapSortWork::Complete=>{self.cursor=1;","HeapSortWork::Complete=>{work+=1;self.cursor=1;"),
 ("3|13=>{let count=","3|13=>{let count="),
 ("if self.cursor>=count{self.cursor=0;","if self.cursor>=count{work+=1;self.cursor=0;"),
 ("else if let Some(order)=observation{if order==Ordering::Equal","else if let Some(order)=observation{work+=1;if order==Ordering::Equal"),
 ("if self.cursor==count{self.cursor=0;","if self.cursor==count{work+=1;self.cursor=0;"),
 ("else if !self.searching{self.low=0;","else if !self.searching{work+=1;self.low=0;"),
 ("else if self.low==self.high{if","else if self.low==self.high{work+=1;if"),
 ("else if let Some(order)=observation{let middle=","else if let Some(order)=observation{work+=1;let middle="),
 ("8=>{self.plan=","8=>{work+=1;self.plan=")]:
 assert a in body,(a,p);body=body.replace(a,b)
sig=old[:old.index("{")].replace("->Result<NamedEditStep,ValueError>","->crate::RetainedWorkOutcome<NamedEditStep>")
s=s[:start]+sig+"{let mut work=0;let result=(||{"+body+"})();crate::RetainedWorkOutcome{result,work}}"+s[end:]
start=s.index(" pub fn close_step(");end=s.index("\n pub fn terminal_is_empty(",start)
s=s[:start]+r""" pub fn close_step(&mut self,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome{let mut work=0;let result=(||{
  if !self.closing{return Err(invalid());}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
  let demand=self.close_demand(grant.maximum_copy_bytes)?;if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
  if let Some(plan)=self.plan.as_mut(){if !plan.terminal_is_empty(){let step=take_work!(work,plan.close_step(Self::child(grant)));return Ok(RetainedCloneStep::Progress(Self::receipt(step.progress())));}self.plan=None;work+=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:Self::fixed_copy_bytes(),..Default::default()}));}
  if let Some(index)=self.base_index.as_mut(){if !index.terminal_is_empty(){let step=take_work!(work,index.close_step(Self::child(grant)));return Ok(RetainedCloneStep::Progress(Self::receipt(step.progress())));}self.base_index=None;work+=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:Self::fixed_copy_bytes(),..Default::default()}));}
  if !self.retirement.is_empty(){let step=take_work!(work,self.retirement.step_granted(Self::child(grant)));return Ok(RetainedCloneStep::Progress(Self::receipt(step.progress())));}
  for owner in [&mut *self.base,&mut *self.added,&mut *self.removed,&mut *self.modified]{if let Some(step)=take_work!(work,self.retirement.begin_granted(owner,Self::child(grant))){return Ok(RetainedCloneStep::Progress(Self::receipt(step.progress())));}}
  Err(invalid())
 })();crate::RetainedWorkOutcome{result,work}}
""" +s[end:]
p.write_text(s)
print("Named plan normal/close direct child and real bounded phase work forwarded")
