#!/usr/bin/env python3
"""Ports the presentation update/slot callers of tool-run entities to the effect-taking API (apply after entities.py). Seed, progress-clone and step-ring-insert are other crates and stay untouched."""
R="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏯️tool-run/"
def edit(path,pairs):
    t=open(R+path,encoding='utf8').read()
    for a,b,n in pairs:
        assert t.count(a)==n,(path,t.count(a),n,a[:100])
        t=t.replace(a,b)
    open(R+path,'w',encoding='utf8').write(t)
U="📊️progress/🪟️presentation/"
edit(U+"🧬️update/🦀️.rs",[
 ("use semio_framework_tool_run_entities::{ToolRunEntityEdit,","use semio_framework_tool_run_entities::{settle_close,ToolRunEntityEdit,",1),
 ("retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneSource,StagedSourceAdmission}","retained_clone::{RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneSource,StagedSourceAdmission}",1),
 (" pub fn advance(self,grant:ToolRunEntityEditGrant)->"," pub fn advance(self,grant:ToolRunEntityEditGrant,effect:&mut RetainedAllocationEffect<'_>)->",1),
 ("maximum_depth:self.demand.depth,..grant.retirement},..grant})}","maximum_depth:self.demand.depth,..grant.retirement},..grant},effect)}",1),
 (" pub fn advance(&mut self,grant:ToolRunEntityEditGrant)->"," pub fn advance(&mut self,grant:ToolRunEntityEditGrant,effect:&mut RetainedAllocationEffect<'_>)->",1),
 ("admission.advance(&mut state.source,grant.retirement)","admission.advance(&mut state.source,effect)",1),
 ("return child.step(grant.retirement).map(|step|ToolRunEntityEditProgress{retirement:step.progress(),..Default::default()});","return settle_close(child.step(effect)).map(|retirement|ToolRunEntityEditProgress{retirement,..Default::default()});",4),
 ("if !child.complete(){return child.advance(grant);}let(projected,progress)","if !child.complete(){return child.advance(grant,effect);}let(projected,progress)",1),
])
edit(U+"🗃️slot/🦀️.rs",[
 ("use semio_framework_tool_run_entities::{ToolRunEntityEditGrant,","use semio_framework_tool_run_entities::{settle_close,ToolRunEntityEditGrant,",1),
 ("retained_clone::{RetainedCloneGrant,RetainedCloneProgress}","retained_clone::{RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress}",1),
 (" pub fn advance(&mut self,grant:ToolRunEntityEditGrant)->"," pub fn advance(&mut self,grant:ToolRunEntityEditGrant,effect:&mut RetainedAllocationEffect<'_>)->",1),
 ("return owner.step(grant.retirement).map(|step|ToolRunEntityEditProgress{retirement:step.progress(),..Default::default()});","return settle_close(owner.step(effect)).map(|retirement|ToolRunEntityEditProgress{retirement,..Default::default()});",1),
 ("}else{return update.advance(grant);}return Ok(receipt);}","}else{return update.advance(grant,effect);}return Ok(receipt);}",1),
])
print("ok")
