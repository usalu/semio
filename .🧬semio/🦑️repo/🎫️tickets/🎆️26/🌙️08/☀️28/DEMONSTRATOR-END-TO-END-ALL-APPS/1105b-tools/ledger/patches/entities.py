#!/usr/bin/env python3
"""Ports tool-run entities (👥️entities) to the effect-taking retained-map and retained-clone API.
Fail-closed: every anchor must match exactly once or nothing is written. Run from anywhere; idempotence is not provided (apply once).
Callers (presentation update/slot, tick native) are listed in entities.md and are NOT touched here."""
import sys
B="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏯️tool-run/👥️entities/"
def load(p):return open(B+p,encoding='utf8').read()
def edit(text,pairs):
    for a,b,n in pairs:
        assert text.count(a)==n,(text.count(a),n,a[:110])
        text=text.replace(a,b)
    return text
out={}
# ---- 🦀️.rs
s=load("🦀️.rs")
s=edit(s,[
 ("use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedClone,",
  "use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedAllocationEffect,RetainedWorkPoll,RetainedClone,",1),
 ("pub type EntityMap=RetainedOrderedMap<u64,()>;",
  """pub type EntityMap=RetainedOrderedMap<u64,()>;

/// 🏁️ Settles one finished retirement poll into its physical receipt; a refused or suspended close is a fault of this workspace.
pub(crate) fn settle_close(poll:semio_framework_value::OriginalRetirementPoll)->Result<RetainedCloneProgress,ValueError>{match poll{RetainedWorkPoll::Complete(turn)=>match turn.result{Ok(step)=>Ok(step.progress()),Err(failure)=>{let progress=failure.retained_progress();drop(failure);Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"entity workspace close was refused").with_retained_progress(progress))}},RetainedWorkPoll::Pending{..}=>Err(ValueError::invariant("entity workspace close stayed pending"))}}""",1),
 (" fn advance(&mut self,grant:ToolRunEntityEditGrant)->Result<ToolRunEntityEditProgress,ValueError>{match self{Self::Append(owner)=>{let progress=match owner.advance(RetainedOrderedMapInsertGrant{retirement:grant.retirement,comparison:grant.comparison,maximum_moved_items:grant.maximum_moved_items,maximum_moved_bytes:grant.maximum_moved_bytes,maximum_capacity_bytes:grant.maximum_capacity_bytes}).result?{",
  " fn advance(&mut self,grant:ToolRunEntityEditGrant,effect:&mut RetainedAllocationEffect<'_>)->Result<ToolRunEntityEditProgress,ValueError>{match self{Self::Append(owner)=>{let progress=match owner.advance(RetainedOrderedMapInsertGrant{retirement:grant.retirement,comparison:grant.comparison,maximum_moved_items:grant.maximum_moved_items,maximum_moved_bytes:grant.maximum_moved_bytes,maximum_capacity_bytes:grant.maximum_capacity_bytes},effect).result?{",1),
 ("let progress=owner.advance(RetainedOrderedMapRemoveGrant{retirement:grant.retirement,comparison:grant.comparison,maximum_moved_items:grant.maximum_moved_items,maximum_moved_bytes:grant.maximum_moved_bytes}).result?.progress();",
  "let progress=owner.advance(RetainedOrderedMapRemoveGrant{retirement:grant.retirement,comparison:grant.comparison,maximum_moved_items:grant.maximum_moved_items,maximum_moved_bytes:grant.maximum_moved_bytes},effect).result?.progress();",1),
 (" fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{match self{Self::Append(owner)=>Ok(owner.close_step(grant).result?.progress()),Self::Retract(owner)=>Ok(owner.close_step(grant).result?.progress())}}",
  " fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,ValueError>{settle_close(match self{Self::Append(owner)=>owner.close_step(effect),Self::Retract(owner)=>owner.close_step(effect)})}",1),
 (" pub fn advance(&mut self,grant:ToolRunEntityEditGrant)->Result<ToolRunEntityEditProgress,ValueError>{\n  if grant.retirement.maximum_items==0||self.state.cancelled||self.state.phase==3{",
  " pub fn advance(&mut self,grant:ToolRunEntityEditGrant,effect:&mut RetainedAllocationEffect<'_>)->Result<ToolRunEntityEditProgress,ValueError>{\n  if grant.retirement.maximum_items==0||self.state.cancelled||self.state.phase==3{",1),
 ("return Ok(ToolRunEntityEditProgress{retirement:owner.close_step(grant.retirement)?,..Default::default()});","return Ok(ToolRunEntityEditProgress{retirement:owner.close_step(effect)?,..Default::default()});",1),
 ("return owner.advance(grant);}\n  if state.index==state.targets.len()","return owner.advance(grant,effect);}\n  if state.index==state.targets.len()",1),
])
out["🦀️.rs"]=s
# ---- 📸️version
v=load("📸️version/🦀️.rs")
v=edit(v,[
 ("use super::{StagedSourceAdmission,publish_sealed,","use super::{settle_close,StagedSourceAdmission,publish_sealed,",1),
 ("retained_clone::{RetainedClone,RetainedCloneCursor,","retained_clone::{OriginalEffectReply,RetainedAllocationEffect,RetainedClone,RetainedCloneCursor,",1),
 ("1=>state.clone.as_ref().unwrap().advance_demands(state.source.as_ref().unwrap().borrow(),body)?,","1=>state.clone.as_ref().unwrap().advance_demands(state.source.as_ref().unwrap().borrow(),body,u64::MAX)?,",1),
 (" pub fn advance(&mut self,body:usize,grant:ToolRunEntityEditGrant)->Result<ToolRunEntityEditProgress,ValueError>{"," pub fn advance(&mut self,body:usize,grant:ToolRunEntityEditGrant,effect:&mut RetainedAllocationEffect<'_>)->Result<ToolRunEntityEditProgress,ValueError>{",1),
 ("receipt=admission.advance(&mut state.source,grant.retirement).result?.progress();","receipt=admission.advance(&mut state.source,effect).result?.progress();",1),
 ("let step=state.clone.as_mut().ok_or_else(||ValueError::invariant(\"clone absent\"))?.advance(state.source.as_ref().ok_or_else(||ValueError::invariant(\"source absent\"))?.borrow(),body,grant.retirement).result?;",
  "let step=match state.clone.as_mut().ok_or_else(||ValueError::invariant(\"clone absent\"))?.advance(state.source.as_ref().ok_or_else(||ValueError::invariant(\"source absent\"))?.borrow(),body,effect.maximum_work_units(),effect).result{Ok(step)=>step,Err(failure)=>{let progress=failure.retained_progress();drop(failure);return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,\"entity clone was refused\").with_retained_progress(progress));}};",1),
 ("let map=state.clone.as_mut().ok_or_else(||ValueError::invariant(\"clone absent\"))?.take(grant.retirement).result?.map(|(map,_)|map).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,\"entity clone completed without its original candidate\"))?;",
  "let mut candidate=None;let mut reply_slot=None;{let reply=OriginalEffectReply::borrow(&mut reply_slot)?;state.clone.as_mut().ok_or_else(||ValueError::invariant(\"clone absent\"))?.take(&mut candidate,effect,reply)?;}reply_slot.take().ok_or_else(||ValueError::invariant(\"entity clone take left no reply\"))??;let map=candidate.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,\"entity clone completed without its original candidate\"))?;",1),
 ("if !child.terminal_is_empty(){receipt=child.close_step(grant.retirement).result?.progress();}else{state.clone=None;state.phase=4;}","if !child.terminal_is_empty(){receipt=settle_close(child.close_step(effect))?;}else{state.clone=None;state.phase=4;}",1),
 ("if child.complete(){state.phase=5;}else{return child.advance(grant);}","if child.complete(){state.phase=5;}else{return child.advance(grant,effect);}",1),
 ("if !child.terminal_is_empty(){receipt=child.step(grant.retirement)?.progress();}else{state.edit_close=None;state.phase=7;}","if !child.terminal_is_empty(){receipt=settle_close(child.step(effect))?;}else{state.edit_close=None;state.phase=7;}",1),
])
out["📸️version/🦀️.rs"]=v
# ---- 🧾️provenance
p=load("🧾️provenance/🦀️.rs")
p=edit(p,[
 ("use super::{StagedSourceAdmission,publish_sealed,","use super::{settle_close,StagedSourceAdmission,publish_sealed,",1),
 ("retained_clone::{RetainedCloneGrant,","retained_clone::{RetainedAllocationEffect,RetainedCloneGrant,",1),
 (" pub fn advance(&mut self,grant:ToolRunEntityEditGrant)->Result<ToolRunEntityEditProgress,ValueError>{\n"," pub fn advance(&mut self,grant:ToolRunEntityEditGrant,effect:&mut RetainedAllocationEffect<'_>)->Result<ToolRunEntityEditProgress,ValueError>{\n",1),
 ("receipt.retirement=admission.advance(&mut state.source,grant.retirement).result?.progress();","receipt.retirement=admission.advance(&mut state.source,effect).result?.progress();",1),
 ("if !child.complete(){return child.advance(grant);}","if !child.complete(){return child.advance(grant,effect);}",2),
 ("receipt.retirement=child.step(grant.retirement)?.progress();","receipt.retirement=settle_close(child.step(effect))?;",2),
])
out["🧾️provenance/🦀️.rs"]=p
if "--check" in sys.argv: print("anchors ok");sys.exit(0)
for k,t in out.items(): open(B+k,'w',encoding='utf8').write(t)
print("written",list(out))
