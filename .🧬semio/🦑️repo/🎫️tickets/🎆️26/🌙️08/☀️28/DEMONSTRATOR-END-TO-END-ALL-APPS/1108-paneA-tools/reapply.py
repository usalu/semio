import sys
P="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
T=P+"/🧵️instance/🧾️typed-operations/📥️staging"
S=P+"/⏪️time-travel/🧵️instance"
only=sys.argv[1] if len(sys.argv)>1 else ""
fails=[]
def edit(path,pairs):
    if only and only not in path: return
    s=open(path).read();ok=True
    for a,b,*n in pairs:
        n=n[0] if n else 1
        if s.count(a)!=n:
            print("FAIL",path.split('/')[-2],a[:90],s.count(a));ok=False;continue
        s=s.replace(a,b)
    if ok:
        open(path,'w').write(s);print("OK",path.split('/')[-2])
    else: fails.append(path)
edit(T+"/🪆️child-group/🦀️.rs",[
("semio_plugin_retained_command::typed_operation::pending_result_page","semio_framework_plugin_retained_command::typed_operation::pending_result_page",4),
('''return group.advance_inputs(&self.child_composition.children,&envelope.id,dialect,&mounted.meta.actor,mounted.operation.operation,mounted.operation.generation,owner.expires_at_us,cx.now_us().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"private child source requires the incoming caller clock"))?,grant).map_err(error);}''',
'''let now_us=cx.now_us().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"private child source requires the incoming caller clock"))?;return semio_plugin_ownership::bounded_ownership::plugin_effect_close::<ValueError>(cx,grant,|effect|group.advance_inputs(&self.child_composition.children,&envelope.id,dialect,&mounted.meta.actor,mounted.operation.operation,mounted.operation.generation,owner.expires_at_us,now_us,effect)).map_err(error);}'''),
])
def recv(path):
    if only and only not in path: return
    s=open(path).read()
    a=s.index("let result=cx.with_native_receiving::<_,semio_plugin_ownership")
    b=s.index("return result.map_err(Into::into);}",a)+len("return result.map_err(Into::into);}")
    new='''let mut out=None;let mut stepped=None;let ran=cx.with_native_receiving(&mut out,|native,issued|{stepped=Some(group.advance_publications(&mut *self.stores.store,&mut self.child_composition.children,mounted.operation.operation,issued.maximum_copy_bytes,issued,native));Ok(())})?;if !ran{return Ok(RetainedCloneStep::Progress(Default::default()))}let outcome=out.ok_or_else(||plugin_sdk_fault("private group receiving scope produced no outcome"))?;cx.settle_receiving_scope()?;if outcome.source_refusal.is_some()||outcome.call.is_err(){return Err(semio_framework_job::OriginalTurnFailure::from(semio_framework_job::OriginalTurnPrimary::NativeSource(outcome)).into())}return stepped.ok_or_else(||plugin_sdk_fault("private group receiving scope lost its step result"))?.map_err(semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure::from);}'''
    s=s[:a]+new+s[b:];open(path,'w').write(s);print("OK recv")
try: recv(T+"/🪆️child-group/🦀️.rs")
except Exception as e: print("FAIL recv",e)
edit(T+"/⌨️typing/🦀️.rs",[
("close_original_retained_step(cx,demand,|_|{let result=(||{self.tools.original_scrub_dispatch.take()","close_original_retained_step(cx,demand,|_|{let result:Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>=(||{self.tools.original_scrub_dispatch.take()"),
])
edit(T+"/⌨️typing/🎟️original/🦀️.rs",[
("use semio_framework_job::{StepContext,OriginalSourceBinding,OriginalTurnFailure,OriginalConstructionDemand,close_original_retained_step};","use semio_plugin_job::original_steps::begin;\nuse semio_framework_job::{StepContext,OriginalSourceBinding,OriginalTurnFailure,OriginalConstructionDemand,close_original_effect_step};"),
("use semio_framework_value::{SharedUtf8,RetirementDemand,","use semio_framework_value::{OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll,SharedUtf8,RetirementDemand,"),
("use std::mem::{ManuallyDrop,size_of};\n","use std::mem::{ManuallyDrop,size_of};\ntype Decoder=(semio_framework_value::retained_clone::NativeAllocationIdentity,u64);\n"),
("fn lease_demands(","""fn selected(close:&mut RetainedCloneClose,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{match close.select(body){Ok(selection)=>selection.step(effect),Err(error)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})}}
fn finished(step:RetainedCloneStep)->OriginalRetirementPoll{RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(step),work:1})}
fn lease_demands("""),
("binding:OriginalSourceBinding,decoder:(u64,u64),ledger:usize","binding:OriginalSourceBinding,decoder:Decoder,ledger:usize"),
("binding:Option<OriginalSourceBinding>,decoder:Option<(u64,u64)>,body:usize","binding:Option<OriginalSourceBinding>,decoder:Option<Decoder>,body:usize"),
("let decoder=cx.native_receiving_identity().ok_or_else(invalid)?;\n  birth.built(cx,output,construction(d),||{let(window,mut alias)=window.admit_clone(grant).result?;alias.copied_bytes",
"let decoder=cx.native_receiving_identity().ok_or_else(invalid)?;\n  let mut staged=None;if output.is_none(){if !birth.stage_shared_clone(cx,window)?{return Ok(false)}staged=birth.take_staged_clone();}\n  let ready=birth.built(cx,output,construction(d),||{let(window,mut alias)=staged.take().ok_or_else(invalid)?;alias.copied_bytes"),
("retirement:Default::default()}),alias))})\n }","retirement:Default::default()}),alias))})?;birth.restage_clone(staged);Ok(ready)\n }"),
("let grant=cx.retained_grant();let decoder=cx.native_receiving_identity()","let decoder=cx.native_receiving_identity()"),
("close_original_retained_step(cx,d,|g|match self.retirement.select(self.body){Ok(selection)=>selection.step(g),Err(error)=>semio_framework_value::RetainedWorkOutcome{result:Err(error),work:0}})?;","close_original_effect_step(cx,d,|effect|selected(&mut self.retirement,self.body,effect))?;",3),
("close_original_retained_step(cx,d,|g|{let result=(||if self.retirement.is_empty(){Ok(self.retirement.begin_granted(&mut self.owners,g).result?.unwrap_or(RetainedCloneStep::Progress(Default::default())))}else{self.retirement.select(self.body)?.step(g).result})();semio_framework_value::RetainedWorkOutcome{result,work:1}})?;",
"close_original_effect_step(cx,d,|effect|if self.retirement.is_empty(){begin(&mut self.retirement,&mut self.owners,effect)}else{selected(&mut self.retirement,self.body,effect)})?;"),
("close_original_retained_step(cx,d,|_|{let result=(||{drop(self.row.take());Ok(RetainedCloneStep::Progress(receipt(d)))})();semio_framework_value::RetainedWorkOutcome{result,work:1}})?;","close_original_effect_step(cx,d,|_|{drop(self.row.take());finished(RetainedCloneStep::Progress(receipt(d)))})?;",2),
("close_original_retained_step(cx,d,|g|{let result=(||Ok(self.retirement.begin_granted(&mut self.receipt,g).result?.unwrap_or(RetainedCloneStep::Progress(Default::default()))))();semio_framework_value::RetainedWorkOutcome{result,work:1}})?;","close_original_effect_step(cx,d,|effect|begin(&mut self.retirement,&mut self.receipt,effect))?;"),
("close_original_retained_step(cx,d,|g|{let result=(||Ok(self.retirement.begin_granted(&mut self.previous,g).result?.unwrap_or(RetainedCloneStep::Progress(Default::default()))))();semio_framework_value::RetainedWorkOutcome{result,work:1}})?;","close_original_effect_step(cx,d,|effect|begin(&mut self.retirement,&mut self.previous,effect))?;"),
("close_original_retained_step(cx,d,|g|{let result=(||Ok(self.retirement.begin_granted(&mut self.captured,g).result?.unwrap_or(RetainedCloneStep::Progress(Default::default()))))();semio_framework_value::RetainedWorkOutcome{result,work:1}})?;","close_original_effect_step(cx,d,|effect|begin(&mut self.retirement,&mut self.captured,effect))?;"),
("close_original_retained_step(cx,d,|g|{let result=(||{\n   if !self.retirement.is_empty(){return self.retirement.select(self.body)?.step(g).result}","close_original_effect_step(cx,d,|effect|{\n   if !self.retirement.is_empty(){return selected(&mut self.retirement,self.body,effect)}"),
("return Ok(self.retirement.begin_granted(&mut self.$field,g).result?.unwrap_or(RetainedCloneStep::Progress(Default::default())))}}}","return begin(&mut self.retirement,&mut self.$field,effect)}}}"),
("self.compared=0;Ok(RetainedCloneStep::Complete(receipt(d)))\n  })();semio_framework_value::RetainedWorkOutcome{result,work:1}})?;Ok(self.terminal_is_empty())","self.compared=0;finished(RetainedCloneStep::Complete(receipt(d)))\n  })?;Ok(self.terminal_is_empty())"),
("let grant=cx.retained_grant();let ready=self.birth.built(cx,&mut self.captured,construction(d),||{let(window,mut alias)=window.admit_clone(grant).result?;alias.copied_bytes+=header.copy_bytes;alias.retained_capacity_bytes+=header.capacity_bytes;alias.released_bytes+=header.release_bytes;Ok((window,alias))})?;if !ready{return Ok(false)}",
"let mut staged=None;if self.captured.is_none(){if !self.birth.stage_shared_clone(cx,window)?{return Ok(false)}staged=self.birth.take_staged_clone();}let ready=self.birth.built(cx,&mut self.captured,construction(d),||{let(window,mut alias)=staged.take().ok_or_else(invalid)?;alias.copied_bytes+=header.copy_bytes;alias.retained_capacity_bytes+=header.capacity_bytes;alias.released_bytes+=header.release_bytes;Ok((window,alias))})?;self.birth.restage_clone(staged);if !ready{return Ok(false)}"),
])
U=T+"/📬️publication/📬️typed-operation-unit/🦀️.rs"
edit(U,[
("let result = cx.with_native_receiving(|receiving|match pending {","let mut out=None;let mut stepped=None;let ran=cx.with_native_receiving(&mut out,|receiving,issued|{let body_grant=RetainedCloneGrant{maximum_copy_bytes:issued.maximum_copy_bytes.saturating_sub(capture),..issued};let grant=store::os_store::one_item::ArtifactStoreOneItemGrant::from_retained(body_grant);stepped=Some(match pending {"),
("""map_err(OriginalPublicationAdvanceFailure::Primary),
            });
""","""map_err(OriginalPublicationAdvanceFailure::Primary),
            });Ok(())})?;
            if !ran{return Ok(())}
            let outcome=out.ok_or_else(||plugin_sdk_fault("typed publication receiving scope produced no outcome"))?;let receipt=cx.settle_receiving_scope();
            if outcome.source_refusal.is_some()||outcome.call.is_err(){receipt?;return Err(semio_framework_job::OriginalTurnFailure::from(semio_framework_job::OriginalTurnPrimary::NativeSource(outcome)).into())}
            let result=stepped.ok_or_else(||plugin_sdk_fault("typed publication receiving scope lost its step result"))?;
"""),
("            let receipt=cx.consume_receiving_retained(retained,ownership);\n",""),
("close_original_retained_step(cx,d,|_|{mounted.pending_window_config_receipt=Some(receipt);semio_framework_value::RetainedWorkOutcome{work:1,","close_original_retained_step(cx,d,|_|{mounted.pending_window_config_receipt=Some(receipt);semio_framework_value::RetainedWorkOutcome::<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>{work:1,"),
])
edit(S+"/🪄️supersede-authoring-and-extension/🦀️.rs",[
("semio_framework_job::close_original_retained_step(cx,d,|_|{owner.result=None;semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}})?;}else{let g=cx.retained_grant();let step=cx.with_native_retirement(|control|result.close_original_step_controlled(g,control))?;cx.consume_retained(step.progress())?;cx.consume_fuel(step.progress().copied_items as u64);}return Ok(SupersedeAuthored::Pending)}",
 "semio_plugin_job::original_steps::forgotten(cx,d,||owner.result=None)?;}else{let grant=cx.retained_grant();let Some((poll,performed))=cx.with_original_effect(grant,|cx,effect|cx.with_native_retirement(|control|Ok(result.close_original_step_controlled(effect,control))))?else{return Ok(SupersedeAuthored::Pending)};let step=semio_plugin_ownership::bounded_ownership::plugin_settle_poll(poll?)?;cx.consume_retained(step.progress())?;cx.consume_work(performed);}return Ok(SupersedeAuthored::Pending)}"),
("semio_framework_job::close_original_retained_step(cx,d,|g|owner.retirement.step_granted(g))?;","semio_framework_job::close_original_effect_step(cx,d,|effect|owner.retirement.step_granted(body,effect))?;"),
("semio_framework_job::close_original_retained_step(cx,d,|g|{let turn=owner.retirement.begin_granted(&mut owner.$field,g);semio_framework_value::RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))),work:turn.work}})?;","semio_framework_job::close_original_effect_step(cx,d,|effect|semio_plugin_job::original_steps::begin(&mut owner.retirement,&mut owner.$field,effect))?;"),
("let mut complete=false;semio_framework_job::close_original_retained_step(cx,d,|_|{self.time_travel.authoring=None;complete=true;semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}})?;Ok(",
 "let complete=semio_plugin_job::original_steps::forgotten(cx,d,||self.time_travel.authoring=None)?;Ok("),
("let mut done=None;semio_framework_job::admit_original_construction(cx,&mut done,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:0,release_bytes:0,depth:1},|_|{match self.stores.store.take_derived_report_result(&mut owner.replay){Ok(result)=>owner.result=Some(result),Err(error)=>{owner.failure=Some(error);owner.closing=true;}}semio_framework_value::RetainedWorkOutcome{result:Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}})?;return Ok(SupersedeAuthored::Pending)}",
 "semio_plugin_job::original_steps::acted(cx,d,||{match self.stores.store.take_derived_report_result(&mut owner.replay){Ok(result)=>owner.result=Some(result),Err(error)=>{owner.failure=Some(error);owner.closing=true;}}Ok(())})?;return Ok(SupersedeAuthored::Pending)}"),
("let result=cx.with_native_receiving(|native|self.stores.store.step_derived_report_replay(owner.replay.as_mut().unwrap(),grant,native,&mut deadline));let p=match &result{Err(vcs::VcsError::InverseRefused(error))=>error.retained_progress(),_=>owner.replay.as_mut().unwrap().take_preparation_receipt()};let work=owner.replay.as_mut().unwrap().take_preparation_work().unwrap_or(p.copied_items as u64);cx.consume_fuel(work);if let Some(issued)=cx.receiving_issued_grant(){cx.consume_receiving_retained(issued,p)?;}else{cx.consume_retained(p)?;}match result{",
 "let mut out=None;let mut stepped=None;let ran=cx.with_native_receiving(&mut out,|native,issued|{stepped=Some(self.stores.store.step_derived_report_replay(owner.replay.as_mut().unwrap(),issued,native,&mut deadline));Ok(())})?;if !ran{return Ok(SupersedeAuthored::Pending)}let outcome=out.ok_or_else(||semio_framework_value::ValueError::invariant(\"original supersede replay receiving scope produced no outcome\"))?;cx.settle_receiving_scope()?;if outcome.source_refusal.is_some()||outcome.call.is_err(){return Err(semio_framework_job::OriginalTurnFailure::from(semio_framework_job::OriginalTurnPrimary::NativeSource(outcome)).into())}let result=stepped.ok_or_else(||semio_framework_value::ValueError::invariant(\"original supersede replay receiving scope lost its step result\"))?;let p=match &result{Err(vcs::VcsError::InverseRefused(error))=>error.retained_progress(),_=>owner.replay.as_mut().unwrap().take_preparation_receipt()};let work=owner.replay.as_mut().unwrap().take_preparation_work().unwrap_or(p.copied_items as u64);cx.consume_fuel(work);match result{"),
])
edit(S+"/🏃️turn-driving-and-replay-steps/🦀️.rs",[
("if self.drive_original_discard_command(grant)?{return Ok(())}","if self.drive_original_discard_command(grant,cx)?{return Ok(())}"),
("pub fn drive_original_discard_command(&mut self,grant:RetainedCloneGrant)->Result<bool,Fault>{","pub fn drive_original_discard_command(&mut self,grant:RetainedCloneGrant,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,Fault>{"),
("return time_travel.command_retirements.admit_typed_retirement(&mut time_travel.discard_cursor,grant).result.map(|receipt|Some(receipt.unwrap_or_default()))",
 "return effected(cx,grant,&mut |effect|time_travel.command_retirements.admit_typed_retirement(&mut time_travel.discard_cursor,effect).result.map(|receipt|Some(receipt.unwrap_or_default())))"),
("return time_travel.command_retirements.reserve_step(grant).result.map(Some)}","return effected(cx,grant,&mut |effect|time_travel.command_retirements.reserve_step(effect).result.map(Some))}"),
("return custody.close_step(grant).map(|step|Some(step.progress()))","return effected(cx,grant,&mut |effect|semio_plugin_ownership::bounded_ownership::plugin_settle_value(custody.close_step(effect)).map(|step|Some(step.progress())))"),
("return time_travel.command_retirements.step(grant).result.map(|step|Some(step.progress()))}","return effected(cx,grant,&mut |effect|semio_plugin_ownership::bounded_ownership::plugin_settle_value(time_travel.command_retirements.step(grant.maximum_copy_bytes,effect)).map(|step|Some(step.progress())))}"),
("let receipt=(||->Result<Option<RetainedCloneProgress>,ValueError>{\n            if let Some(cursor)=time_travel.discard_cursor","let effected=|cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant,run:&mut dyn FnMut(&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<Option<RetainedCloneProgress>,ValueError>|->Result<Option<RetainedCloneProgress>,ValueError>{match cx.with_original_effect(grant,|_,effect|run(effect))?{Some((result,performed))=>{cx.consume_work(performed);result},None=>Ok(Some(Default::default()))}};\n        let receipt=(||->Result<Option<RetainedCloneProgress>,ValueError>{\n            if let Some(cursor)=time_travel.discard_cursor"),
("let mut done=None;semio_framework_job::admit_original_construction(cx,&mut done,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:0,release_bytes:0,depth:1},|_|{self.time_travel.finalize_outcome=None;self.time_travel.authoring_finalization=false;semio_framework_value::RetainedWorkOutcome{result:Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}})?;if done.is_some(){",
"if semio_plugin_job::original_steps::acted(cx,d,||{self.time_travel.finalize_outcome=None;self.time_travel.authoring_finalization=false;Ok(())})?{"),
("let mut performed=None;semio_framework_job::admit_original_construction(cx,&mut performed,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:0,depth:1},|_|{let result=(||->Result<((),RetainedCloneProgress),ValueError>{","let performed=semio_plugin_job::original_steps::acted(cx,d,||{"),
("self.time_travel.finalize_outcome=None;Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()}))})();semio_framework_value::RetainedWorkOutcome{result,work:1}})?;\n  if performed.is_some(){","self.time_travel.finalize_outcome=None;Ok(())})?;\n  if performed{"),
])
