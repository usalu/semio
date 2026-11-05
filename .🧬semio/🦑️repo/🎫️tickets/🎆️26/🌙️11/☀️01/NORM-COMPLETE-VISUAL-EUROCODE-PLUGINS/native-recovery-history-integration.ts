import fs from 'node:fs';
const b='🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/';
function replace(s:string,a:string,c:string){if(!s.includes(a))throw Error('Missing source: '+a.slice(0,120));return s.replace(a,c);}
{
const p=b+'🧾️metadata/🦀️.rs';let s=fs.readFileSync(p,'utf8');
if(!s.includes('fn last_turn_work')){
 s=replace(s,'pub struct RetainedHistoryOpMetaDecode{','pub struct RetainedHistoryOpMetaDecode{last_work:u64,');
 s=replace(s,'Self{source:identity,','Self{last_work:0,source:identity,');
 s=replace(s,'impl RetainedHistoryOpMetaDecode{','impl RetainedHistoryOpMetaDecode{\n /// 🧮️ Preserves the actual selected operation and delegated failed prefix.\n pub fn last_turn_work(&self)->u64{self.last_work}');
 s=s.replace(/(pub fn (?:step|close_step)<[^\n]+\{)\n/g,'$1\n  self.last_work=0;');
 s=replace(s,'if let Some(status)=self.auxiliary_step(control,g)?{return Ok(status)}let child=','if let Some(status)=self.auxiliary_step(control,g)?{return Ok(status)}self.last_work=1;let child=');
 s=replace(s,'let result=child.close_step(nested).result;','let turn=child.close_step(nested);self.last_work=turn.work;let result=turn.result;');
 s=replace(s,'if child.terminal_is_empty(){drop(self.retiring.take());','if child.terminal_is_empty(){self.last_work=1;drop(self.retiring.take());');
 s=replace(s,'control:&mut dyn NativeDecodeCloseControl,g:RetainedCloneGrant)->Result<RetainedCloneProgress,RetainedHistoryOpMetaFault>{control.charge','control:&mut dyn NativeDecodeCloseControl,g:RetainedCloneGrant,work:&mut u64)->Result<RetainedCloneProgress,RetainedHistoryOpMetaFault>{control.charge');
 s=replace(s,'match admit_owned_retirement(original,RetainedCloneGrant{maximum_depth:g.maximum_depth-1,..g}).result{','let turn=admit_owned_retirement(original,RetainedCloneGrant{maximum_depth:g.maximum_depth-1,..g});*work=turn.work;match turn.result{');
 s=s.replaceAll('&mut self.retiring,control,g)?','&mut self.retiring,control,g,&mut self.last_work)?');
 s=replace(s,'let p=partial.close_step(child)?;self.publish(g,p)?','let result=partial.close_step(child);self.last_work=partial.last_turn_work();let p=result?;self.publish(g,p)?');
 s=replace(s,'if partial.terminal_is_empty(){drop(self.origin_partial.take());','if partial.terminal_is_empty(){self.last_work=1;drop(self.origin_partial.take());');
 s=replace(s,'if projection.terminal_is_empty(){drop(self.projection.take());','if projection.terminal_is_empty(){self.last_work=1;drop(self.projection.take());');
 s=replace(s,'if projection.take_partial_into(&mut slot,child)?{','if projection.take_partial_into(&mut slot,child)?{self.last_work=1;');
 s=replace(s,'projection.close_step(child)?;let receipt=projection.take_receipt();','let result=projection.close_step(child);self.last_work=projection.last_turn_work();result?;let receipt=projection.take_receipt();');
 s=s.replace(/(let turn=SharedUtf8::receive_original\([^;]+;)/g,'$1self.last_work=turn.work;');
 fs.writeFileSync(p,s);
}
}
{
const p=b+'🗂️file/🦀️.rs';let s=fs.readFileSync(p,'utf8');
s=s.replace(/self\.last_work=self\.last_work\.saturating_add\(match child\{Child::Edit\(c\)=>c\.take_delegated_work\(\),Child::Transition\(c\)=>c\.take_delegated_work\(\),Child::Conflict\(c\)=>c\.take_delegated_work\(\),Child::Identity\(_\)=>0\}\);/g,'');
for(const [call,out] of [['c.step(source,dictionary,prior,control,child_grant)','RetainedHistoryOpMetaFault'],['c.step(source,dictionary,control,child_grant)','ValueError'],['c.close_step(source,prior,control,child_grant)','RetainedHistoryOpMetaFault'],['c.close_step(source,control,child_grant)','ValueError']]){
 if(call.includes('dictionary,control')){
  s=replace(s,call+'.map(|_|())','{let result='+call+';self.last_work=c.last_turn_work();result.map(|_|())}');
 }else{
  s=s.replaceAll(call+'.map(|_|())','{let result='+call+';self.last_work=c.last_turn_work();result.map(|_|())}');
 }
}
const identityArm=/Child::Identity\(c\)=>\{[^\n]+/g;
s=s.replace(identityArm,line=>line.replaceAll('self.last_work=c.last_turn_work();',''));
s=replace(s,'let candidate=self.candidate_id();if let Some(prior)=self.prior_id(self.validation_index)','let candidate=self.candidate_id();let candidate_len=candidate.len();if let Some(prior)=self.prior_id(self.validation_index)');
s=replace(s,'else if end==candidate.len(){','else if end==candidate_len{');
fs.writeFileSync(p,s);
}
