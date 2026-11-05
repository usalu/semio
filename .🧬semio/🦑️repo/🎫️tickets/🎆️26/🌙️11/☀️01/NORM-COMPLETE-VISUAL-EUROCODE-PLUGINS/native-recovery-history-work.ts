import fs from 'node:fs';
const base='🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/';
function edit(path:string, run:(s:string)=>string){const p=base+path;const before=fs.readFileSync(p,'utf8');const after=run(before);if(after!==before)fs.writeFileSync(p,after);}
function replace(s:string,a:string,b:string){if(!s.includes(a))throw Error('Missing exact source: '+a.slice(0,150));return s.replace(a,b);}
edit('🛬️decode/🗂️file/🦀️.rs',s=>{
 s=replace(s,'use std::mem::{ManuallyDrop,size_of_val};','use std::mem::{ManuallyDrop,size_of_val};\nuse semio_framework_value::RetainedWorkOutcome;\nuse std::cell::Cell;\nstruct ObservedSource<\'a,S:?Sized>{source:&\'a S,reads:Cell<u64>}\nimpl<S:RetainedWireByteSource+?Sized> RetainedWireByteSource for ObservedSource<\'_,S>{fn original_source_identity(&self)->OriginalWireSourceIdentity{self.source.original_source_identity()}fn byte_at(&self,index:usize)->Option<u8>{self.reads.set(self.reads.get().saturating_add(1));self.source.byte_at(index)}}');
 const admit='pub fn admit<S:RetainedWireByteSource+?Sized>(source:&S,range:(usize,usize),limits:HistoryReadLimits,persisted_document:bool,control:&NativeDecodeControl<\'_>,g:RetainedCloneGrant)->Result<Option<Self>,RetainedHistoryOpMetaFault>{';
 s=replace(s,admit,'pub fn admit<S:RetainedWireByteSource+?Sized>(source:&S,range:(usize,usize),limits:HistoryReadLimits,persisted_document:bool,control:&NativeDecodeControl<\'_>,g:RetainedCloneGrant)->RetainedWorkOutcome<Option<Self>,RetainedHistoryOpMetaFault>{let mut work=0;let result=Self::admit_result(source,range,limits,persisted_document,control,g,&mut work);RetainedWorkOutcome{work,result}}\n fn admit_result<S:RetainedWireByteSource+?Sized>(source:&S,range:(usize,usize),limits:HistoryReadLimits,persisted_document:bool,control:&NativeDecodeControl<\'_>,g:RetainedCloneGrant,work:&mut u64)->Result<Option<Self>,RetainedHistoryOpMetaFault>{');
 s=replace(s,';let(_,progress)=frames.take_receipt().unwrap();',';*work+=1;let(_,progress)=frames.take_receipt().unwrap();');
 for(const name of ['take_ready_into','take_fault_into','take_partial_into']){
  const signature=new RegExp('pub fn '+name+'\\(([^\\n]*?)\\)->Result<bool,RetainedHistoryOpMetaFault>\\{');const match=s.match(signature);if(!match)throw Error(name);
  const args=match[1];s=s.replace(signature,'pub fn '+name+'('+args+')->RetainedWorkOutcome<bool,RetainedHistoryOpMetaFault>{self.last_work=0;let result=self.'+name+'_result(recipient,g);RetainedWorkOutcome{work:self.last_work,result}}\n fn '+name+'_result('+args+')->Result<bool,RetainedHistoryOpMetaFault>{');
 }
 s=replace(s,'self.closed=true;self.publish(g,metadata())?;Ok(true)}\n pub fn take_fault_into','self.closed=true;self.last_work=1;self.publish(g,metadata())?;Ok(true)}\n pub fn take_fault_into');
 s=replace(s,'*recipient=self.fault.take();self.cancel();','*recipient=self.fault.take();self.last_work=1;self.cancel();');
 s=replace(s,'*recipient=self.partial.take();self.publish(g,metadata())?;Ok(true)}','*recipient=self.partial.take();self.last_work=1;self.publish(g,metadata())?;Ok(true)}');
 s=replace(s,'self.last_work=0;let result=self.step_result(source,control,g);','self.last_work=0;let source=ObservedSource{source,reads:Cell::new(0)};let result=self.step_result(&source,control,g);self.last_work=self.last_work.saturating_add(source.reads.get());');
 s=replace(s,'self.last_work=0;let result=self.close_result(source,control,g);','self.last_work=0;let source=ObservedSource{source,reads:Cell::new(0)};let result=self.close_result(&source,control,g);self.last_work=self.last_work.saturating_add(source.reads.get());');
 s=replace(s,'let equal=prior.as_bytes()[self.validation_byte..end]==candidate.as_bytes()[self.validation_byte..end];','let mut compared=0u64;let equal=prior.as_bytes()[self.validation_byte..end].iter().zip(&candidate.as_bytes()[self.validation_byte..end]).all(|(a,b)|{compared+=1;a==b});self.last_work=self.last_work.saturating_add(compared);');
 s=replace(s,'};let receipt=child.take_receipt();let step=self.accept(g,child_grant,receipt)?;result?;Ok(step)}','};self.last_work=self.last_work.saturating_add(match child{Child::Edit(c)=>c.take_delegated_work(),Child::Transition(c)=>c.take_delegated_work(),Child::Conflict(c)=>c.take_delegated_work(),Child::Identity(_)=>0});let receipt=child.take_receipt();let step=self.accept(g,child_grant,receipt)?;result?;Ok(step)}');
 s=replace(s,'};let receipt=child.take_receipt();let step=self.accept(g,child_grant,receipt)?;result?;Ok(step)\n }','};self.last_work=self.last_work.saturating_add(match child{Child::Edit(c)=>c.take_delegated_work(),Child::Transition(c)=>c.take_delegated_work(),Child::Conflict(c)=>c.take_delegated_work(),Child::Identity(_)=>0});let receipt=child.take_receipt();let step=self.accept(g,child_grant,receipt)?;result?;Ok(step)\n }');
 return s;
});
for(const [folder,type] of [['🧵️edit','RetainedHistoryEditDecode'],['⚔️conflict','RetainedHistoryConflictDecode'],['🔀️transition','RetainedHistoryTransitionDecode'],['🧾️metadata','RetainedHistoryOpMetaDecode']])edit('🛬️decode/'+folder+'/🦀️.rs',s=>{
 s=replace(s,'pub struct '+type+'{','pub struct '+type+'{delegated_work:u64,');
 s=replace(s,'Self{source:identity,','Self{delegated_work:0,source:identity,');
 s=replace(s,'impl '+type+'{','impl '+type+'{\n /// 🧮️ Transfers actual nested computational work independently of byte receipts.\n pub fn take_delegated_work(&mut self)->u64{std::mem::take(&mut self.delegated_work)}');
 s=s.replace(/let result=(owner|child)\.close_step\((RetainedCloneGrant\{maximum_depth:g.maximum_depth-2,\.\.g\}|nested)\)\.result;/g,'let turn=$1.close_step($2);self.delegated_work=self.delegated_work.saturating_add(turn.work);let result=turn.result;');
 s=s.replace(/match admit_owned_retirement\(original,RetainedCloneGrant\{maximum_depth:g.maximum_depth-1,\.\.g\}\)\.result\{/g,'let turn=admit_owned_retirement(original,RetainedCloneGrant{maximum_depth:g.maximum_depth-1,..g});self.delegated_work=self.delegated_work.saturating_add(turn.work);match turn.result{');
 s=s.replace(/(let turn=SharedUtf8::receive_original\([^;]+;)/g,'$1self.delegated_work=self.delegated_work.saturating_add(turn.work);');
 if(folder==='🧵️edit'){
  s=replace(s,'let receipt=meta.take_receipt();','self.delegated_work=self.delegated_work.saturating_add(meta.take_delegated_work());let receipt=meta.take_receipt();');
  s=replace(s,'else{meta.close_step(source,prior,control,child)?;}let receipt=meta.take_receipt();','else{let result=meta.close_step(source,prior,control,child);self.delegated_work=self.delegated_work.saturating_add(meta.take_delegated_work());result?;}let receipt=meta.take_receipt();');
 }
 if(folder==='🧾️metadata'){
  s=replace(s,'control:&mut dyn NativeDecodeCloseControl,g:RetainedCloneGrant)->Result<RetainedCloneProgress,RetainedHistoryOpMetaFault>{control.charge','control:&mut dyn NativeDecodeCloseControl,g:RetainedCloneGrant,work:&mut u64)->Result<RetainedCloneProgress,RetainedHistoryOpMetaFault>{control.charge');
  s=replace(s,'self.delegated_work=self.delegated_work.saturating_add(turn.work);match turn.result{Ok((owner,p))=>{*recipient','*work=work.saturating_add(turn.work);match turn.result{Ok((owner,p))=>{*recipient');
  s=s.replaceAll('&mut self.retiring,control,g)?','&mut self.retiring,control,g,&mut self.delegated_work)?');
  s=replace(s,'let p=partial.close_step(child)?;self.publish(g,p)?','let result=partial.close_step(child);self.delegated_work=self.delegated_work.saturating_add(partial.take_delegated_work());let p=result?;self.publish(g,p)?');
 }
 return s;
});
edit('🛬️decode/🔀️origin/🦀️.rs',s=>{
 s=replace(s,'pub struct RetainedHistoryOriginPartial{','pub struct RetainedHistoryOriginPartial{delegated_work:u64,');
 s=replace(s,'Self{cells:ManuallyDrop::new(cells),','Self{delegated_work:0,cells:ManuallyDrop::new(cells),');
 s=replace(s,'impl RetainedHistoryOriginPartial{','impl RetainedHistoryOriginPartial{\n /// 🧮️ Transfers actual delegated retirement work, including failed prefixes.\n pub fn take_delegated_work(&mut self)->u64{std::mem::take(&mut self.delegated_work)}');
 s=replace(s,'let result=child.close_step(nested).result;','let turn=child.close_step(nested);self.delegated_work=self.delegated_work.saturating_add(turn.work);let result=turn.result;');
 s=replace(s,'match admit_owned_retirement(original,nested).result{','let turn=admit_owned_retirement(original,nested);self.delegated_work=self.delegated_work.saturating_add(turn.work);match turn.result{');
 return s;
});
